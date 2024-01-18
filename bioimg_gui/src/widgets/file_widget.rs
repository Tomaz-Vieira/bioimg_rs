use std::path::PathBuf;

use super::DrawAndParse;


#[derive(thiserror::Error, Debug, Clone)]
pub enum FilePickerError{
    #[error("Empty")]
    Empty,
    #[error("Loading")]
    Loading,
    #[error("Could not open {path}: {reason}")]
    IoError{path: PathBuf, reason: String},
}

pub struct FileWidget{
    contents: Result<(PathBuf, Vec<u8>), FilePickerError>,
}

impl Default for FileWidget{
    fn default() -> Self {
        Self{contents: Err(FilePickerError::Empty)}
    }
}

impl FileWidget{
    pub fn path(&self) -> Option<&PathBuf>{
        self.contents.as_ref().ok().map(|(path, _)| path)
    }
}

impl DrawAndParse for FileWidget{
    type Parsed<'p> = &'p (PathBuf, Vec<u8>);
    type Error= FilePickerError;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, _id: egui::Id) -> Result<Self::Parsed<'p>, Self::Error>{
        ui.horizontal(|ui|{
            match &self.path(){
                None => ui.label("None"),
                Some(path) => ui.label(path.to_string_lossy())
            };

            if ui.button("Open...").clicked(){
                // FIXME: async + web
                let path_buf = rfd::FileDialog::new()
                    .set_directory("/")
                    .pick_file();
                self.contents = Err(FilePickerError::Empty);

                'file_read: {
                    let Some(pth) = path_buf else{
                        break 'file_read;
                    };
                    match std::fs::read(&pth){
                        Ok(d) => {
                            self.contents = Ok((pth, d));
                        },
                        Err(err) => {
                            self.contents = Err(FilePickerError::IoError { path: pth, reason: err.to_string() });
                        }
                    }
                }
            }
            self.contents.as_ref().map_err(|err| err.clone())
        }).inner
    }
}
