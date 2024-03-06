use std::borrow::Borrow;
use std::collections::HashSet;
use std::io::{Seek, Write};
use std::path::PathBuf;

use bioimg_spec::rdf::non_empty_list::NonEmptyList;
use bioimg_spec::rdf::FileReference;
use paste::paste;
use uuid::Uuid;

use crate::axis_size_resolver::SlotResolver;
use crate::npy_array::NpyArray;
use crate::zip_writer_ext::ModelZipWriter;
use crate::zoo_model::ModelPackingError;
use bioimg_spec::rdf::model::axis_size::QualifiedAxisId;
use bioimg_spec::rdf::model::AnyAxisSize;
use bioimg_spec::rdf::model::{self as modelrdf, TensorId};

use super::axis_size_resolver::AxisSizeResolutionError;

#[rustfmt::skip]
macro_rules! declare_slot {( $struct_name:ident, $inout:ident) => { paste!{
    #[allow(dead_code)]
    pub struct $struct_name <DATA: Borrow<NpyArray>> {
        descr: modelrdf::[<$inout TensorDescr>],  //FIXME: size should always be resolved, no the case with spec structs
        test_tensor: DATA,
    }

    impl<DATA: Borrow<NpyArray>> $struct_name <DATA> {
        pub fn dump(&self, zip_file: &mut ModelZipWriter<impl Write + Seek>) -> Result<modelrdf::[<$inout TensorDescr>], ModelPackingError> {
            let test_tensor_zip_path = format!("/{}", Uuid::new_v4());
            zip_file.write_file(&test_tensor_zip_path, |writer| self.test_tensor.borrow().write_npy(writer))?;
            Ok(modelrdf::[<$inout TensorDescr>] {
                test_tensor: FileReference::Path(PathBuf::from(test_tensor_zip_path)),
                ..self.descr.clone()
            })
        }
    }
}};}

declare_slot!(InputSlot, Input);
declare_slot!(OutputSlot, Output);

#[derive(thiserror::Error, Debug)]
pub enum TensorValidationError {
    #[error("{0}")]
    ReadNpyError(#[from] ndarray_npy::ReadNpyError),
    #[error("Urls file references are unsupported for now")]
    UrlUnsupported,
    #[error("Test tensor with shape {test_tensor_shape:?} does not map number of reported axes ({num_described_axes})")]
    MismatchedNumDimensions {
        test_tensor_shape: Vec<usize>,
        num_described_axes: usize,
    },
    #[error("Axis '{qualified_axis_id}' is incompatible with test tensor dim #{axis_index} with extent {expected_extent}")]
    IncompatibleAxis {
        qualified_axis_id: QualifiedAxisId,
        expected_extent: usize,
        axis_index: usize,
    },
    #[error("{0}")]
    AxisSizeResolutionError(#[from] AxisSizeResolutionError),
    #[error("Duplicate tensor id: {0}")]
    DuplicateTensorId(TensorId),
    #[error("Empty model interface inputs")]
    EmptyInputs,
    #[error("Empty model interface outputs")]
    EmptyOutputs,
}

#[allow(dead_code)]
pub struct ModelInterface<DATA: Borrow<NpyArray>> {
    inputs: Vec<InputSlot<DATA>>,
    outputs: Vec<OutputSlot<DATA>>,
}

impl<DATA: Borrow<NpyArray>> ModelInterface<DATA> {
    pub fn dump(
        &self,
        zip_writer: &mut ModelZipWriter<impl Write + Seek>,
    ) -> Result<
        (
            NonEmptyList<modelrdf::InputTensorDescr>,
            NonEmptyList<modelrdf::OutputTensorDescr>,
        ),
        ModelPackingError,
    > {
        let inputs: NonEmptyList<_> = self
            .inputs
            .iter()
            .map(|inp| inp.dump(zip_writer))
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .unwrap();
        let outputs: NonEmptyList<_> = self
            .outputs
            .iter()
            .map(|inp| inp.dump(zip_writer))
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .unwrap();
        Ok((inputs, outputs))
    }
    pub fn try_build(
        mut inputs: Vec<(modelrdf::InputTensorDescr, DATA)>,
        mut outputs: Vec<(modelrdf::OutputTensorDescr, DATA)>,
    ) -> Result<Self, TensorValidationError> {
        if inputs.len() == 0 {
            return Err(TensorValidationError::EmptyInputs);
        }
        if outputs.len() == 0 {
            return Err(TensorValidationError::EmptyOutputs);
        }
        let mut axes_sizes: Vec<(QualifiedAxisId, AnyAxisSize)> = Vec::with_capacity(inputs.len() + outputs.len());
        let mut seen_tensor_ids = HashSet::<TensorId>::with_capacity(inputs.len() + outputs.len());

        #[rustfmt::skip]
        macro_rules! collect_sizes {($slots:ident) => { paste! {
            for slot in $slots.iter().map(|i| &i.0) {
                if !seen_tensor_ids.insert(slot.id.clone()){
                    return Err(TensorValidationError::DuplicateTensorId(slot.id.clone()))
                }
                for axis in slot.axes.iter() {
                    let Some(size) = axis.size() else{
                        continue;
                    };
                    let qual_id = QualifiedAxisId {
                        tensor_id: slot.id.clone(),
                        axis_id: axis.id().clone(),
                    };
                    axes_sizes.push((qual_id, size.clone()));
                }
            }
        }};}
        collect_sizes!(inputs);
        collect_sizes!(outputs);

        let size_map = SlotResolver::new(axes_sizes)?.solve()?;

        #[rustfmt::skip] macro_rules! resolve_and_validate {($slots:ident) => {
            for (slot, test_tensor) in $slots.iter_mut() {
                let test_tensor_shape = (*test_tensor).borrow().shape();
                let mut test_tensor_dims = test_tensor_shape.iter();
                let num_described_axes = slot.axes.len();
                for (axis_index, resolved_size) in slot.axes.resolve_sizes_with(&size_map).iter().enumerate() {
                    let Some(dim) = test_tensor_dims.next() else {
                        return Err(TensorValidationError::MismatchedNumDimensions {
                            test_tensor_shape: test_tensor_shape.into(),
                            num_described_axes,
                        });
                    };
                    let Some(resolved_size) = resolved_size else {
                        continue;
                    };
                    if !resolved_size.is_compatible_with_extent(*dim) {
                        return Err(TensorValidationError::IncompatibleAxis {
                            qualified_axis_id: QualifiedAxisId{
                                tensor_id: slot.id.clone(),
                                axis_id: slot.axes[axis_index].id().clone(), //FIXME: alternative to indexing?
                            },
                            expected_extent: *dim,
                            axis_index,
                        });
                    }
                }
            }
        }}
        resolve_and_validate!(inputs);
        resolve_and_validate!(outputs);

        Ok(Self {
            inputs: inputs
                .into_iter()
                .map(|inp| InputSlot { descr: inp.0, test_tensor: inp.1 })
                .collect(),
            outputs: outputs
                .into_iter()
                .map(|out| OutputSlot { descr: out.0, test_tensor: out.1 })
                .collect(),
        })
    }
}
