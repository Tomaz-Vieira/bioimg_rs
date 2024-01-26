use url::Url;

use super::StatefulWidget;

pub struct StagingUrl{
    raw: String,
    parsed: Result<Url, url::ParseError>,
}

impl Default for StagingUrl{
    fn default() -> Self {
        let raw = String::default();
        Self { raw: raw.clone(), parsed: Url::try_from(raw.as_str()) }
    }
}

impl StatefulWidget for StagingUrl{
    type Value<'p> = Result<Url, url::ParseError>;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, _id: egui::Id){
        ui.add(
            egui::TextEdit::singleline(&mut self.raw).min_size(egui::Vec2{x: 200.0, y: 10.0})
        );
        self.parsed = Url::try_from(self.raw.as_str())
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.parsed.clone()
    }
}