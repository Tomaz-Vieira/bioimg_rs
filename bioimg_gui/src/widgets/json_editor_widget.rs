use crate::{project_data::JsonObjectEditorWidgetRawData, result::Result};
use super::{Draw, DrawArgs, Restore, StatefulWidget, ValueWidget, GetValue};
use super::code_editor_widget::{CodeEditorWidget, JsonLanguage};
use super::error_display::show_if_error;

pub type JsonObject = serde_json::Map<String, serde_json::Value>;

pub struct JsonObjectEditorWidget{
    pub code_editor_widget: CodeEditorWidget<JsonLanguage>,
    pub parsed: Result<JsonObject>
}

impl JsonObjectEditorWidget{
    pub fn update(&mut self){
        self.parsed = serde_json::from_str(&self.code_editor_widget.raw)
            .map_err(|err| err.into());
    }
}

impl Restore for JsonObjectEditorWidget{
    type RawData = JsonObjectEditorWidgetRawData;
    fn dump(&self) -> Self::RawData {
        JsonObjectEditorWidgetRawData{
            code_editor_widget: self.code_editor_widget.dump()
        }
    }
    fn restore(&mut self, raw: Self::RawData) {
        self.code_editor_widget.restore(raw.code_editor_widget);
        self.update()
    }
}

impl ValueWidget for JsonObjectEditorWidget{
    type Value<'v> = serde_json::Map<String, serde_json::Value>;

    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        self.code_editor_widget.raw = serde_json::to_string_pretty(&value).unwrap();
        self.parsed = Ok(value)
    }
}

impl Default for JsonObjectEditorWidget{
    fn default() -> Self {
        let default_value = serde_json::Map::new();
        Self{
            code_editor_widget: {
                let mut widget: CodeEditorWidget<JsonLanguage> = Default::default();
                widget.raw = serde_json::to_string(&default_value).unwrap();
                widget
            },
            parsed: Ok(default_value)
        }
    }
}

impl StatefulWidget for JsonObjectEditorWidget{
    type Value<'p> = &'p Result<serde_json::Map<String, serde_json::Value>>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            self.code_editor_widget.draw_and_parse(ui, id.with("code".as_ptr()));
            self.parsed = serde_json::from_str(&self.code_editor_widget.raw)
                .map_err(|err| err.into());
            show_if_error(ui, &self.parsed);
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        &self.parsed
    }
}

impl Draw for JsonObjectEditorWidget{
    type Resp = ();

    fn draw<'args>(&mut self, args: DrawArgs<'args>) -> egui::InnerResponse<Self::Resp> {
        args.ui.vertical(|ui|{
            self.code_editor_widget.draw_and_parse(ui, args.id.with("code".as_ptr()));
            self.parsed = serde_json::from_str(&self.code_editor_widget.raw)
                .map_err(|err| err.into());
            show_if_error(ui, &self.parsed);
        })
    }
}

impl GetValue for JsonObjectEditorWidget{
    type Value<'val> = Result<&'val JsonObject> where Self: 'val;

    fn get_value<'val>(&'val self) -> Self::Value<'val> {
        self.parsed.as_ref().map_err(|e| e.clone())
    }
}
