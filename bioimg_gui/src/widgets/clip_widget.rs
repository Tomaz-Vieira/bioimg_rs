use bioimg_spec::rdf::model::{self as modelrdf, preprocessing::ClipDescr};
use bioimg_spec::rdf::model::preprocessing as preproc;

use super::staging_float::StagingFloat;
use super::Restore;
use super::{error_display::show_if_error, StatefulWidget, ValueWidget};

use crate::result::{GuiError, Result};

#[derive(Restore)]
pub struct ClipWidget{
    pub min_widget: StagingFloat<f32>,
    pub max_widget: StagingFloat<f32>,
    #[restore_on_update]
    pub parsed: Result<modelrdf::preprocessing::ClipDescr>,
}

impl ClipWidget{
    pub fn update(&mut self){
        self.parsed = || -> Result<ClipDescr> {
            let min = self.min_widget.state()?;
            let max = self.max_widget.state()?;
            Ok(ClipDescr::try_from_min_max(min, max)?)
        }();
    }
}

impl ValueWidget for ClipWidget{
    type Value<'v> = preproc::ClipDescr;
    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        self.min_widget.set_value(value.min());
        self.max_widget.set_value(value.max());
        self.parsed = Ok(value)
    }
}

impl Default for ClipWidget{
    fn default() -> Self {
        Self{
            min_widget: Default::default(),
            max_widget: Default::default(),
            parsed: Err(GuiError::new("empty"))
        }
    }
}

impl StatefulWidget for ClipWidget{
    type Value<'p> = &'p Result<modelrdf::preprocessing::ClipDescr>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        self.update();
        ui.horizontal(|ui|{
            ui.strong("Min Percentile");
            self.min_widget.draw_and_parse(ui, id.with("min"));
            ui.strong("Max Percentile");
            self.min_widget.draw_and_parse(ui, id.with("max"));
        });
        show_if_error(ui, &self.parsed)
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        &self.parsed
    }
}
