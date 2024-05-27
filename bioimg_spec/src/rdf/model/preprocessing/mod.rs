pub mod scale_linear;
pub mod binarize;
pub mod clip;
pub mod sigmoid;
pub mod zero_mean_unit_variance;
pub mod scale_range;
pub mod ensure_dtype;

pub use self::scale_linear::{ScaleLinearDescr, SimpleScaleLinearDescr, ScaleLinearAlongAxisDescr};
pub use self::binarize::{BinarizeDescr, SimpleBinarizeDescr, BinarizeAlongAxisDescr};
pub use self::clip::ClipDescr;
pub use self::sigmoid::Sigmoid;
pub use self::scale_range::{ScaleRangeDescr, ScaleRangePercentile};
pub use self::ensure_dtype::EnsureDtype;
pub use self::zero_mean_unit_variance::Zmuv;
pub use self::zero_mean_unit_variance::{SimpleFixedZmuv, FixedZmuvAlongAxis, FixedZmuv};

use crate::util::SingleOrMultiple;

// //////////////

fn _default_to_0f32() -> f32{
    0.0
}

fn _default_to_100f32() -> f32{
    100.0
}

fn _default_to_1() -> f32{
    1.0
}

fn _default_to_single_1() -> SingleOrMultiple<f32>{
    SingleOrMultiple::Single(1.0)
}

fn _default_to_single_0() -> SingleOrMultiple<f32>{
    SingleOrMultiple::Single(0.0)
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum PreprocessingEpsilonParsingError{
    #[error("Preprocessing epsilon must be in open interval ]0, 0.1], found {0}")]
    OutOfRange(f32)
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug)]
pub struct PreprocessingEpsilon(f32);

impl TryFrom<f32> for PreprocessingEpsilon{
    type Error = PreprocessingEpsilonParsingError;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value > 0.0 && value <= 0.1{
            Ok(Self(value))
        }else{
            Err(PreprocessingEpsilonParsingError::OutOfRange(value))
        }
    }
}

impl From<PreprocessingEpsilon> for f32{
    fn from(value: PreprocessingEpsilon) -> Self {
        value.0
    }
}

// //////////////////

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(tag = "id", content = "kwargs")]
pub enum PreprocessingDescr {
    #[serde(rename = "binarize")]
    Binarize(BinarizeDescr),
    #[serde(rename = "clip")]
    Clip(ClipDescr),
    #[serde(rename = "ensure_dtype")]
    EnsureDtype(EnsureDtype),
    #[serde(rename = "scale_linear")]
    ScaleLinear(ScaleLinearDescr),
    #[serde(rename = "sigmoid")]
    Sigmoid(Sigmoid),
    #[serde(rename = "fixed_zero_mean_unit_variance")]
    FixedZeroMeanUnitVariance(FixedZmuv),
    #[serde(rename = "zero_mean_unit_variance")]
    ZeroMeanUnitVariance(Zmuv),
    #[serde(rename = "scale_range")]
    ScaleRange(ScaleRangeDescr),
}
