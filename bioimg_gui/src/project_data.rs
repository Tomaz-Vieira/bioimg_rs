use std::path::PathBuf;

use bioimg_spec::rdf::model::{self as modelrdf, AxisType};
use crate::widgets::author_widget::AuthorWidget;
use crate::widgets::maintainer_widget::MaintainerWidget;
use crate::widgets::onnx_weights_widget::OnnxWeightsWidget;
use crate::widgets::posstprocessing_widget::PostprocessingWidget;

use crate::widgets::file_source_widget::FileSourceWidget;
use crate::widgets::cite_widget::CiteEntryWidget;
use crate::widgets::pytorch_statedict_weights_widget::PytorchStateDictWidget;
use crate::widgets::weights_widget::{KerasHdf5WeightsWidget, TorchscriptWeightsWidget};
use crate::widgets::Restore;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AuthorWidgetRawData{
    pub name_widget: String,
    pub affiliation_widget: Option<String>,
    pub email_widget: Option<String>,
    pub github_user_widget: Option<String>,
    pub orcid_widget: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CiteEntryWidgetRawData {
    pub citation_text_widget: String,
    pub doi_widget: Option<String>,
    pub url_widget: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MaintainerWidgetRawData {
    pub github_user_widget: String,
    pub affiliation_widget: Option<String>,
    pub email_widget: Option<String>,
    pub orcid_widget: Option<String>,
    pub name_widget: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum FileWidgetRawData{
    Empty,
    AboutToLoad{path: PathBuf},
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum TestTensorWidgetRawData{
    Empty,
    Loaded{path: Option<PathBuf>, data: Vec<u8>},
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum LocalFileSourceWidgetRawData{
    Empty,
    AboutToLoad{path: String, inner_path: Option<String>}
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum FileSourceWidgetRawData{
    Local(LocalFileSourceWidgetRawData),
    Url(String),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ImageWidget2LoadingStateRawData{
    Empty,
    Forced{img_bytes: Vec<u8>}
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ImageWidget2RawData{
    pub file_source_widget: FileSourceWidgetRawData,
    pub loading_state: ImageWidget2LoadingStateRawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SpecialImageWidgetRawData{
    pub image_widget: ImageWidget2RawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum IconWidgetRawData{
    Emoji(String),
    Image(SpecialImageWidgetRawData),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CollapsibleWidgetRawData<Inner: Restore>{
    pub is_closed: bool,
    pub inner: Inner::RawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct VersionWidgetRawData{
    pub raw: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CodeEditorWidgetRawData{
    pub raw: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PhysicalScaleWidgetRawData<T>{
    pub raw_scale: String,
    pub unit_widget: Option<T>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct BatchAxisWidgetRawData{
    pub description_widget: String,
    pub staging_allow_auto_size: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ChannelNamesModeRawData{
    Explicit,
    Pattern,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum AxisSizeModeRawData{
    Fixed,
    Reference,
    Parameterized,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ParameterizedAxisSizeWidgetRawData {
    pub staging_min: usize,
    pub staging_step: usize,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AnyAxisSizeWidgetRawData {
    pub mode: AxisSizeModeRawData,

    pub staging_fixed_size: usize,
    pub staging_size_ref: AxisSizeReferenceWidgetRawData,
    pub staging_parameterized: ParameterizedAxisSizeWidgetRawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct IndexAxisWidgetRawData {
    pub description_widget: String,
    pub size_widget: AnyAxisSizeWidgetRawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AxisSizeReferenceWidgetRawData {
    pub staging_tensor_id: String,
    pub staging_axis_id: String,
    pub staging_offset: usize,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChannelAxisWidgetRawData {
    pub description_widget: String,

    pub channel_names_mode_widget: ChannelNamesModeRawData,
    pub channel_extent_widget: usize,
    pub channel_name_prefix_widget: String,
    pub channel_name_suffix_widget: String,

    pub staging_explicit_names: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputSpaceAxisWidgetRawData {
    pub id_widget: String,
    pub description_widget: String,

    pub size_widget: AnyAxisSizeWidgetRawData,
    pub physical_scale_widget: PhysicalScaleWidgetRawData<modelrdf::SpaceUnit>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputTimeAxisWidgetRawData {
    pub id_widget: String,
    pub description_widget: String,

    pub size_widget: AnyAxisSizeWidgetRawData,
    pub physical_scale_widget: PhysicalScaleWidgetRawData<modelrdf::TimeUnit>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputAxisWidgetRawData {
    pub axis_type_widget: bioimg_spec::rdf::model::axes::AxisType,
    pub batch_axis_widget: BatchAxisWidgetRawData,
    pub channel_axis_widget: ChannelAxisWidgetRawData,
    pub index_axis_widget: IndexAxisWidgetRawData,
    pub space_axis_widget: InputSpaceAxisWidgetRawData,
    pub time_axis_widget: InputTimeAxisWidgetRawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct WeightsDescrBaseWidgetRawData{
    pub source_widget: FileSourceWidgetRawData,
    pub authors_widget: Option<Vec<CollapsibleWidgetRawData<AuthorWidget>>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TorchscriptWeightsWidgetRawData{
    pub base_widget: WeightsDescrBaseWidgetRawData,
    pub pytorch_version_widget: VersionWidgetRawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct JsonObjectEditorWidgetRawData{
    pub code_editor_widget: CodeEditorWidgetRawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CondaEnvEditorWidgetRawData{
    pub code_editor_widget: CodeEditorWidgetRawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum PytorchArchModeRawData{
    FromFile,
    FromLib
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PytorchArchWidgetRawData{
    pub mode_widget: PytorchArchModeRawData,
    pub callable_widget: String,
    pub kwargs_widget: JsonObjectEditorWidgetRawData,
    pub import_from_widget: String,
    pub source_widget: FileSourceWidgetRawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PytorchStateDictWidgetRawData{
    pub base_widget: WeightsDescrBaseWidgetRawData,
    pub architecture_widget: PytorchArchWidgetRawData,
    pub version_widget: VersionWidgetRawData,
    pub dependencies_widget: Option<CondaEnvEditorWidgetRawData>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OnnxWeightsWidgetRawData{
    pub base_widget: WeightsDescrBaseWidgetRawData,
    pub opset_version_widget: u32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct KerasHdf5WeightsWidgetRawData{
    pub base_widget: WeightsDescrBaseWidgetRawData,
    pub tensorflow_version_widget: VersionWidgetRawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct WeightsWidgetRawData{
    pub keras_weights_widget: Option<CollapsibleWidgetRawData<KerasHdf5WeightsWidget>>,
    pub torchscript_weights_widget: Option<CollapsibleWidgetRawData<TorchscriptWeightsWidget>>,
    pub pytorch_state_dict_weights_widget: Option<CollapsibleWidgetRawData<PytorchStateDictWidget>>,
    pub onnx_weights_widget: Option<CollapsibleWidgetRawData<OnnxWeightsWidget>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputTensorWidgetRawData {
    pub id_widget: String,
    pub is_optional: bool,
    pub description_widget: String,
    pub axis_widgets: Vec<InputAxisWidgetRawData>,
    pub test_tensor_widget: TestTensorWidgetRawData,
    pub preprocessing_widget: Vec<PreprocessingWidgetRawData>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum PreprocessingWidgetModeRawData {
    Binarize,
    Clip,
    ScaleLinear,
    Sigmoid,
    ZeroMeanUnitVariance,
    ScaleRange,
    EnsureDtype,
    FixedZmuv,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum BinarizeModeRawData{
    Simple,
    AlongAxis,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SimpleBinarizeWidgetRawData{
    pub threshold_widget: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct BinarizeAlongAxisWidgetRawData{
    pub thresholds_widget: Vec<String>,
    pub axis_id_widget: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct BinarizePreprocessingWidgetRawData{
    pub mode: BinarizeModeRawData,
    pub simple_binarize_widget: SimpleBinarizeWidgetRawData,
    pub binarize_along_axis_wiget: BinarizeAlongAxisWidgetRawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ClipWidgetRawData{
    pub min_widget: String,
    pub max_widget: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ScaleLinearModeRawData{
    Simple,
    AlongAxis,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SimpleScaleLinearWidgetRawData{
    pub gain_widget: String,
    pub offset_widget: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ScaleLinearAlongAxisWidgetRawData{
    pub axis_widget: String,
    pub gain_offsets_widget: Vec<SimpleScaleLinearWidgetRawData>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ZeroMeanUnitVarianceWidgetRawData{
    pub axes_widget: Option<Vec<String>>,
    pub epsilon_widget: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PercentilesWidgetRawData{
    pub min_widget: String,
    pub max_widget: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ScaleRangeWidgetRawData{
    pub axes_widget: Option<Vec<String>>,
    pub percentiles_widget: PercentilesWidgetRawData,
    pub epsilon_widget: String,
    pub reference_tensor: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ZmuvWidgetModeRawData{
    Simple,
    AlongAxis,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SimpleFixedZmuvWidgetRawData{
    pub mean_widget: String,
    pub std_widget: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FixedZmuvAlongAxisWidgetRawData{
    pub axis_widget: String,
    pub mean_and_std_widget: Vec<SimpleFixedZmuvWidgetRawData>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FixedZmuvWidgetRawData{
    pub mode_widget: ZmuvWidgetModeRawData,
    pub simple_widget: SimpleFixedZmuvWidgetRawData,
    pub along_axis_widget: FixedZmuvAlongAxisWidgetRawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ScaleLinearWidgetRawData{
    pub mode_widget: ScaleLinearModeRawData,
    pub simple_widget: SimpleScaleLinearWidgetRawData,
    pub along_axis_widget: ScaleLinearAlongAxisWidgetRawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PreprocessingWidgetRawData{
    pub mode: PreprocessingWidgetModeRawData,
    pub binarize_widget: BinarizePreprocessingWidgetRawData,
    pub clip_widget: ClipWidgetRawData,
    pub scale_linear_widget: ScaleLinearWidgetRawData,
    // pub sigmoid sigmoid has no widget since it has no params
    pub zero_mean_unit_variance_widget: ZeroMeanUnitVarianceWidgetRawData,
    pub scale_range_widget: ScaleRangeWidgetRawData,
    pub ensure_dtype_widget: modelrdf::DataType,
    pub fixed_zmuv_widget: FixedZmuvWidgetRawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OutputSpacetimeSizeWidgetRawData{
    pub has_halo: bool,
    pub halo_widget: u64,
    pub size_widget: AnyAxisSizeWidgetRawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OutputTimeAxisWidgetRawData {
    pub id_widget: String,
    pub description_widget: String,

    pub size_widget: OutputSpacetimeSizeWidgetRawData,
    pub physical_scale_widget: PhysicalScaleWidgetRawData<modelrdf::TimeUnit>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OutputSpaceAxisWidgetRawData {
    pub id_widget: String,
    pub description_widget: String,

    pub size_widget: OutputSpacetimeSizeWidgetRawData,
    pub physical_scale_widget: PhysicalScaleWidgetRawData<modelrdf::SpaceUnit>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OutputAxisWidgetRawData {
    pub axis_type_widget: AxisType,

    pub batch_axis_widget: BatchAxisWidgetRawData,
    pub channel_axis_widget: ChannelAxisWidgetRawData,
    pub index_axis_widget: IndexAxisWidgetRawData,
    pub space_axis_widget: OutputSpaceAxisWidgetRawData,
    pub time_axis_widget: OutputTimeAxisWidgetRawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum PostprocessingWidgetModeRawData {
    Binarize,
    Clip,
    ScaleLinear,
    Sigmoid,
    ZeroMeanUnitVariance,
    ScaleRange,
    EnsureDtype,
    FixedZmuv,
    ScaleMeanVariance,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ScaleMeanVarianceWidgetRawData{
    pub reference_tensor_widget: String,
    pub axes_widget: Option<Vec<String>>,
    pub eps_widget: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PostprocessingWidgetRawData{
    pub mode: PostprocessingWidgetModeRawData,
    pub binarize_widget: BinarizePreprocessingWidgetRawData,
    pub clip_widget: ClipWidgetRawData,
    pub scale_linear_widget: ScaleLinearWidgetRawData,
    // pub sigmoid sigmoid has no widget since it has no params
    pub zero_mean_unit_variance_widget: ZeroMeanUnitVarianceWidgetRawData,
    pub scale_range_widget: ScaleRangeWidgetRawData,
    pub ensure_dtype_widget: modelrdf::DataType,
    pub fixed_zmuv_widget: FixedZmuvWidgetRawData,
    pub scale_mean_var_widget: ScaleMeanVarianceWidgetRawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OutputTensorWidgetRawData {
    pub id_widget: String,
    pub description_widget: String,
    pub axis_widgets: Vec<OutputAxisWidgetRawData>,
    pub test_tensor_widget: TestTensorWidgetRawData,
    pub postprocessing_widgets: Vec<CollapsibleWidgetRawData<PostprocessingWidget>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ModelInterfaceWidgetRawData {
    pub input_widgets: Vec<InputTensorWidgetRawData>,
    pub output_widgets: Vec<OutputTensorWidgetRawData>,
}

#[derive(serde::Serialize, serde::Deserialize, strum::VariantNames)]
#[serde(tag = "app_state_raw_data_version")]
pub enum AppStateRawData{
    Version1(AppState1RawData),
}

#[derive(thiserror::Error, Debug)]
pub enum ProjectLoadError{
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Could not parse BSON: {0}")]
    BsonParsingError(#[from] bson::de::Error),
    #[error("No version in project data")]
    MissingVersion,
    #[error("Could not parse project of version {found_version}")]
    FutureVersion{ found_version: String },
}

impl AppStateRawData{
    pub fn supported_versions() -> &'static [&'static str]{
        <Self as strum::VariantNames>::VARIANTS        
    }

    pub fn highest_supported_version() -> &'static str{
        *Self::supported_versions().last().unwrap()
    }

    pub fn save(&self, writer: impl std::io::Write) -> Result<(), bson::ser::Error>{
        let doc = bson::to_document(self)?;
        doc.to_writer(writer)
    }

    pub fn load(reader: impl std::io::Read) -> Result<Self, ProjectLoadError>{
        let doc: bson::Document = bson::from_reader(reader)?;
        let found_version = match doc.get("app_state_raw_data_version"){
            Some(bson::Bson::String(version)) => version.to_owned(),
            _ => return Err(ProjectLoadError::MissingVersion)
        };
        if Self::supported_versions().iter().find(|ver| **ver == found_version.as_str()).is_none(){
            return Err(ProjectLoadError::FutureVersion { found_version })
        }
        Ok(bson::from_document::<Self>(doc)?)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AppState1RawData{
    pub staging_name: String,
    pub staging_description: String,
    pub cover_images: Vec<SpecialImageWidgetRawData>,
    #[serde(default)] // added after AppState1RawData
    pub model_id_widget: Option<String>,
    pub staging_authors: Vec<CollapsibleWidgetRawData<AuthorWidget>>,
    pub attachments_widget: Vec<CollapsibleWidgetRawData<FileSourceWidget>>,
    pub staging_citations: Vec<CollapsibleWidgetRawData<CiteEntryWidget>>,
    #[serde(default)] // added after AppState1RawData
    pub custom_config_widget: Option<JsonObjectEditorWidgetRawData>,
    pub staging_git_repo: Option<String>,
    pub icon_widget: Option<IconWidgetRawData>,
    #[serde(default)] // added after AppState1RawData
    pub links_widget: Vec<String>,
    pub staging_maintainers: Vec<CollapsibleWidgetRawData<MaintainerWidget>>,
    pub staging_tags: Vec<String>,
    pub staging_version: Option<VersionWidgetRawData>,

    pub staging_documentation: CodeEditorWidgetRawData,
    pub staging_license: ::bioimg_spec::rdf::LicenseId,
    //badges
    pub model_interface_widget: ModelInterfaceWidgetRawData,
    ////
    pub weights_widget: WeightsWidgetRawData,
}
