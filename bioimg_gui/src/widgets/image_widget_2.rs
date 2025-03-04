use std::{borrow::Borrow, marker::PhantomData};
use std::sync::Arc;
use std::io::Cursor;
use std::error::Error;

use image::GenericImageView;
use parking_lot as pl;
use bioimg_runtime::{self as rt, FileSource};

use crate::{project_data::{ImageWidget2LoadingStateRawData, ImageWidget2RawData, SpecialImageWidgetRawData}, result::{GuiError, Result}};
use super::{Restore, StatefulWidget, ValueWidget};
use super::error_display::show_error;
use super::file_source_widget::FileSourceWidget;
use super::util::DynamicImageExt;

pub type ArcDynImg = Arc<image::DynamicImage>;
pub type Generation = i64;

pub struct Texture{
    name: String,
    context: egui::Context,
    handle: egui::TextureHandle,
}

impl Texture{
    pub fn load(img: &image::DynamicImage, context: egui::Context) -> Self{
        let texture_name: String = uuid::Uuid::new_v4().to_string();
        let texture_handle = img.to_egui_texture_handle(&texture_name, &context);
        Self{
            name: texture_name,
            context,
            handle: texture_handle,
        }
    }
    pub fn show(&self, ui: &mut egui::Ui, display_size: egui::Vec2){
        let ui_img = egui::Image::new(
            egui::ImageSource::Texture(
                egui::load::SizedTexture {
                    id: self.handle.id(),
                    size: display_size,
                }
            )
        );
        ui.add(ui_img);
    }
}
impl Drop for Texture {
    fn drop(&mut self) {
        self.context.forget_image(&self.name);
    }
}

enum LoadingState{
    Empty{generation: i64},
    Loading{generation: i64, source: rt::FileSource},
    Ready{generation: i64, source: rt::FileSource, img: ArcDynImg, texture: Option<Texture>},
    Forced{generation: i64, img: ArcDynImg, texture: Option<Texture>},
    Failed{generation: i64, source: Option<rt::FileSource>, err: GuiError},
}

impl Default for LoadingState{
    fn default() -> Self {
        Self::Empty { generation: 0 }
    }
}

impl LoadingState{
    pub fn generation(&self) -> i64{
        match self {
            Self::Empty{generation, ..} => *generation,
            Self::Loading {generation,  ..} => *generation,
            Self::Ready {generation,  ..} => *generation,
            Self::Forced {generation,  .. } => *generation,
            Self::Failed {generation,  .. } => *generation,
        }
    }
    pub fn file_source(&self) -> Option<&FileSource>{
        match self {
            Self::Empty{..} => None,
            Self::Loading { source, ..} => Some(source),
            Self::Ready { source, ..} => Some(source),
            Self::Forced { .. } => None,
            Self::Failed { source, .. } => source.as_ref()
        }
    }
}

pub struct ImageWidget2{
    file_source_widget: FileSourceWidget,
    loading_state: Arc<pl::Mutex<LoadingState>>,
}

impl Default for ImageWidget2{
    fn default() -> Self {
        Self{
            file_source_widget: Default::default(),
            loading_state: Arc::new(pl::Mutex::new(Default::default()))
        }
    }
}

impl Restore for ImageWidget2{
    type RawData = ImageWidget2RawData;
    fn dump(&self) -> Self::RawData {
        let loading_state_guard = self.loading_state.lock();
        let loading_state: &LoadingState = &*loading_state_guard;
        ImageWidget2RawData{
            file_source_widget: self.file_source_widget.dump(),
            loading_state: match loading_state{
                LoadingState::Forced{img, ..} => {
                    let mut raw_out = Vec::<u8>::new();
                    if let Err(err) = img.write_to(&mut Cursor::new(&mut raw_out), image::ImageFormat::Png){
                        eprintln!("[WARNING] Could not save pathless image: {err}");
                    }
                    ImageWidget2LoadingStateRawData::Forced { img_bytes: raw_out }
                },
                _ => ImageWidget2LoadingStateRawData::Empty,
            }
        }
    }
    fn restore(&mut self, raw: Self::RawData) {
        self.file_source_widget.restore(raw.file_source_widget);
        let generation = 0;
        let loading_state = match raw.loading_state{
            ImageWidget2LoadingStateRawData::Empty => LoadingState::Empty{generation},
            ImageWidget2LoadingStateRawData::Forced { img_bytes } => 'forced: {
                let Ok(reader) = image::io::Reader::new(Cursor::new(img_bytes)).with_guessed_format() else {
                    eprintln!("[WARNING] Could not guess format of saved image");
                    break 'forced LoadingState::Empty{generation};
                };
                let Ok(image) = reader.decode() else {
                    eprintln!("[WARNING] Could not decoded saved image");
                    break 'forced LoadingState::Empty{generation};
                };
                LoadingState::Forced { generation, img: Arc::new(image), texture: None }
            }
        };
        self.loading_state = Arc::new(pl::Mutex::new(loading_state));
    }
}

impl ValueWidget for ImageWidget2{
    type Value<'v> = (Option<rt::FileSource>, Option<ArcDynImg>);

    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        let generation = 0;
        match value{
            (None, Some(img)) => {
                self.file_source_widget = Default::default();
                self.loading_state = Arc::new(pl::Mutex::new(LoadingState::Forced { generation, img, texture: None}));
            },
            (None, None) => {
                self.file_source_widget = Default::default();
                self.loading_state = Arc::new(pl::Mutex::new(LoadingState::Empty{generation}));
            },
            (Some(file_source), _) => {
                self.file_source_widget.set_value(file_source);
                self.loading_state = Arc::new(pl::Mutex::new(LoadingState::Empty{generation}));
            }
        }
        // self.update(); //FIXME: call once set_value takes a context
    }
}

impl ImageWidget2{
    fn spawn_load_image_task(
        generation: Generation,
        file_source: FileSource,
        loading_state: Arc<pl::Mutex<LoadingState>>,
        ctx: egui::Context,
    ){
        std::thread::spawn(move ||{
            let res = || -> Result<ArcDynImg>{
                let mut img_data = Vec::<u8>::new();
                file_source.read_to_end(&mut img_data)?;
                let img = image::io::Reader::new(Cursor::new(img_data)).with_guessed_format()?.decode()?;
                Ok(Arc::new(img))
            }();
            let mut guard = loading_state.lock();
            if guard.generation() != generation {
                eprintln!("Dropping stale image: {file_source:?}");
                return
            }
            *guard = match res{
                Err(e) => LoadingState::Failed { generation, source: Some(file_source), err: e },
                Ok(img) => LoadingState::Ready { generation, source: file_source, img, texture: None },
            };
            ctx.request_repaint();
        });
    }
}

impl StatefulWidget for ImageWidget2{
    type Value<'p> = Result<Arc<image::DynamicImage>>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id){
        let mut loading_state_guard = self.loading_state.lock();
        let loading_state: &mut LoadingState = &mut *loading_state_guard;

        fn fill_and_show_texture(ui: &mut egui::Ui, tex: &mut Option<Texture>, img: &image::DynamicImage) {
                let tex = tex.get_or_insert_with(|| Texture::load(&img, ui.ctx().clone()));
                let (width, height) = img.dimensions();
                let ratio =  width as f64 / height as f64;
                tex.show(ui, egui::Vec2 { y: 50.0, x: 50.0 * ratio as f32 }); //FIXME: can we not hardcode this?
        }

        ui.vertical(|ui|{
            *loading_state = match std::mem::take(loading_state){
                LoadingState::Forced { generation, img, mut texture } => {
                    let reset_clicked = ui.horizontal(|ui|{
                        let reset_clicked = ui.button("Reset").clicked();
                        fill_and_show_texture(ui, &mut texture, &img);
                        reset_clicked
                    }).inner;
                    if reset_clicked{
                        self.file_source_widget = Default::default();
                        LoadingState::Empty{ generation: generation + 1 }
                    } else {
                        LoadingState::Forced { generation, img, texture }
                    }
                },
                LoadingState::Empty{mut generation} => 'empty: {
                    self.file_source_widget.draw_and_parse(ui, id.with("file source".as_ptr()));
                    let Ok(source) = self.file_source_widget.state() else{
                        break 'empty LoadingState::Empty{generation}
                    };
                    generation += 1;
                    Self::spawn_load_image_task(generation, source.clone(), Arc::clone(&self.loading_state), ui.ctx().clone());
                    LoadingState::Loading { generation, source }
                },
                LoadingState::Loading { generation, source } => 'loading: {
                    let reset_clicked = ui.horizontal(|ui|{
                        let reset_clicked = ui.button("Reset").clicked();
                        ui.weak("Loading");
                        reset_clicked
                    }).inner;
                    if reset_clicked{
                        self.file_source_widget = Default::default();
                        break 'loading LoadingState::Empty { generation: generation + 1 }
                    }
                    LoadingState::Loading { generation, source }
                },
                LoadingState::Ready { generation, source, img, mut texture } => {
                    ui.horizontal(|ui|{
                        self.file_source_widget.draw_and_parse(ui, id);
                        let Ok(widget_source) = self.file_source_widget.state() else {
                            self.file_source_widget = Default::default();
                            return LoadingState::Empty { generation: generation + 1 }
                        };
                        if widget_source == source{
                            fill_and_show_texture(ui, &mut texture, &img);
                            return LoadingState::Ready { generation, source, img, texture }
                        }
                        LoadingState::Empty { generation: generation + 1 }
                    }).inner
                },
                LoadingState::Failed { generation, source, err } => 'failed: {
                    self.file_source_widget.draw_and_parse(ui, id.with("file source".as_ptr()));
                    show_error(ui, &err);
                    let Ok(widget_source) = self.file_source_widget.state() else {
                        break 'failed LoadingState::Empty { generation: generation + 1 }
                    };
                    if source == Some(widget_source){
                        break 'failed LoadingState::Failed { generation, source, err };
                    }
                    LoadingState::Empty { generation: generation + 1 }
                },
            }
        });

    }

    fn state(&self) -> Result<ArcDynImg>{
        let loading_state_guard = self.loading_state.lock();
        let loading_state: &LoadingState = &*loading_state_guard;
        match loading_state{
            LoadingState::Empty{..} | LoadingState::Loading { .. } => Err(GuiError::new("No image selected".to_owned())),
            LoadingState::Failed { err, .. } => Err(err.clone()),
            LoadingState::Ready { img, .. } => Ok(img.clone()),
            LoadingState::Forced { img, .. } => Ok(img.clone()),
        }
    }
}

pub struct SpecialImageWidget<I>{
    image_widget: ImageWidget2,
    marker: PhantomData<I>
}

impl<I> ValueWidget for SpecialImageWidget<I>
where
    I: Borrow<ArcDynImg>
{
    type Value<'v> = (Option<rt::FileSource>, Option<I>);
    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        self.image_widget.set_value(
            (value.0, value.1.map(|val| val.borrow().clone()))
        )
    }
}

impl<I> Restore for SpecialImageWidget<I>{
    type RawData = SpecialImageWidgetRawData;
    fn restore(&mut self, value: Self::RawData){
        self.image_widget.restore(value.image_widget);
    }
    fn dump(&self) -> Self::RawData {
        SpecialImageWidgetRawData{image_widget: self.image_widget.dump()}
    }
}

impl<I> Default for SpecialImageWidget<I>{
    fn default() -> Self {
        Self{
            image_widget: Default::default(),
            marker: Default::default(),
        }
    }
}

impl<I> StatefulWidget for SpecialImageWidget<I>
where
    I : TryFrom<Arc<image::DynamicImage>>,
    <I as TryFrom<Arc<image::DynamicImage>>>::Error: Error,
{
    type Value<'p> = Result<I> where I: 'p;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id){
        ui.horizontal(|ui|{
            self.image_widget.draw_and_parse(ui, id.with("img widget".as_ptr()));
        });
    }

    fn state<'p>(&'p self) -> Result<I>{
        let gui_img = self.image_widget.state()?;
        //FIXME: is it always ok to do this every frame?
        I::try_from(gui_img).map_err(|err| GuiError::from(err))
    } 
}
