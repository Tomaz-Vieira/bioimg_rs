
use crate::rdf::{BoundedString, NonEmptyList};
use crate::rdf::author::Author;
use crate::rdf::file_reference::FileReference;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ModelRdfV0_4 {
    pub name: BoundedString<1, 1024>,

    /// A human-readable name of this model.
    /// It should be no longer than 64 characters
    /// and may only contain letter, number, underscore, minus, parentheses and spaces.
    /// We recommend to chose a name that refers to the model's task and image modality.
    pub description: String,

    /// The authors are the creators of the model RDF and the primary points of contact.
    pub authors: NonEmptyList<Author>,

    /// ∈📦 URL or relative path to a markdown file with additional documentation.
    /// The recommended documentation file name is `README.md`. An `.md` suffix is mandatory.
    pub documentation: FileReference, //Note: the spec doesn't actually enforce .md extension

    // pub inputs: NonEmptyList<InputTensorDescrV4>

    // /// ∈📦 Cover images. Please use an image smaller than 500KB and an aspect ratio width to height of 2:1.
    // #[serde(default)]
    // pub covers: Vec<CoverImageSource>,
    
}
