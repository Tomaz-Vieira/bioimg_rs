use bioimg_spec::rdf::model::{self as modelrdf, preprocessing::ClipDescr};

use super::{error_display::show_if_error, staging_num::StagingNum, StatefulWidget};

use crate::result::{GuiError, Result};

pub struct ClipWidget{
    pub min_widget: StagingNum<f32, f32>,
    pub max_widget: StagingNum<f32, f32>,
    pub parsed: Result<modelrdf::preprocessing::ClipDescr>,
}

impl Default for ClipWidget{
    fn default() -> Self {
        Self{
            min_widget: Default::default(),
            max_widget: Default::default(),
            parsed: Err(GuiError::new("empty".into()))
        }
    }
}

impl StatefulWidget for ClipWidget{
    type Value<'p> = &'p Result<modelrdf::preprocessing::ClipDescr>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.horizontal(|ui|{
            ui.strong("Min Percentile");
            self.min_widget.draw_and_parse(ui, id.with("min"));
            ui.strong("Max Percentile");
            self.min_widget.draw_and_parse(ui, id.with("max"));
        });

        self.parsed = || -> Result<ClipDescr> {
            let min = self.min_widget.state()?;
            let max = self.max_widget.state()?;
            Ok(ClipDescr::try_from_min_max(min, max)?)
        }();
        show_if_error(ui, &self.parsed)
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        &self.parsed
    }
}
