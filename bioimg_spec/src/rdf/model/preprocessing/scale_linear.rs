use crate::{rdf::{model::axes::NonBatchAxisId, non_empty_list::NonEmptyList}, util::SingleOrMultiple};

use super::{_default_to_1, _default_to_single_1, _default_to_single_0};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum ScaleLinearDescr{
    Simple(SimpleScaleLinearDescr),
    AlongAxis(ScaleLinearAlongAxisDescr)
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct SimpleScaleLinearDescr{
    /// multiplicative factor
    #[serde(default = "_default_to_1")]
    gain: f32,
    /// additive term
    #[serde(default)]
    offset: f32,
}

// //////////////////////

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(try_from="ScaleLinearAlongAxisDescrMessage")]
#[serde(into="ScaleLinearAlongAxisDescrMessage")]
pub struct ScaleLinearAlongAxisDescr{
    pub axis: NonBatchAxisId,
    pub gain_offsets: NonEmptyList<(f32, f32)>,
}

#[derive(thiserror::Error, Debug)]
pub enum ScaleLinearDescrParsingError{
    #[error("Number of items in 'gains' and 'offsets' are incompatible")]
    MismatchedGainsAndOffsets{num_gains: usize, num_offsets: usize},
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ScaleLinearAlongAxisDescrMessage{
    /// The axis of of gains/offsets values
    pub axis: NonBatchAxisId,

    /// multiplicative factor
    #[serde(default = "_default_to_single_1")]
    pub gain: SingleOrMultiple<f32>,

    /// additive term
    #[serde(default = "_default_to_single_0")]
    pub offset: SingleOrMultiple<f32>,
}

impl From<ScaleLinearAlongAxisDescr> for ScaleLinearAlongAxisDescrMessage{
    fn from(value: ScaleLinearAlongAxisDescr) -> Self {
        let (gains, offsets): (Vec<_>, Vec<_>) = value.gain_offsets.iter().map(|t| *t).unzip();
        Self {
            axis: value.axis,
            gain: SingleOrMultiple::Multiple(gains),
            offset: SingleOrMultiple::Multiple(offsets),
        }
    }
}

impl TryFrom<ScaleLinearAlongAxisDescrMessage> for ScaleLinearAlongAxisDescr{
    type Error = ScaleLinearDescrParsingError;
    fn try_from(message: ScaleLinearAlongAxisDescrMessage) -> Result<Self, Self::Error> {
        let gain_offsets: Vec<(f32, f32)> = match (&message.gain, &message.offset){
            (SingleOrMultiple::Single(gain), SingleOrMultiple::Single(offset)) => {
                vec![ (gain.clone(), offset.clone()) ]
            },
            (SingleOrMultiple::Single(gain), SingleOrMultiple::Multiple(offsets)) => {
                offsets.iter().map(|offset| (*gain, *offset)).collect()
            },
            (SingleOrMultiple::Multiple(gains), SingleOrMultiple::Single(offset)) => {
                gains.iter().map(|gain| (*gain, *offset)).collect()
            },
            (SingleOrMultiple::Multiple(gains), SingleOrMultiple::Multiple(offsets)) => {
                let num_gains = gains.as_slice().len();
                let num_offsets = offsets.as_slice().len();
                if num_gains != num_offsets {
                    return Err(ScaleLinearDescrParsingError::MismatchedGainsAndOffsets {
                        num_gains, num_offsets
                    })
                }
                gains.iter().zip(offsets).map(|(gain, offset)| (*gain, *offset)).collect()
            }
        };
        Ok(ScaleLinearAlongAxisDescr{
            axis: message.axis,
            gain_offsets: gain_offsets.try_into().unwrap()
        })
    }
}