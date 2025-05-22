use std::{fmt::{Debug, Display}, io::{Read, Seek}, path::{Path, PathBuf}, sync::{Arc, Mutex}};

use bioimg_spec::rdf;
use rc_zip_sync as rczip;
// use zip::{read::ZipFile, ZipArchive};


pub trait SeekReadSend: Seek + Read + Send{}
impl<T: Seek + Read + Send> SeekReadSend for T{}


/// Something that uniquely identifies a zip archive
///
/// Either its path if it lives in the fs, or a name if its, say, in memory
#[derive(Clone, Debug)]
pub enum ZipArchiveIdentifier<'a>{
    Path(&'a Path),
    /// For archives that don't live in the file system, like on memory or other web abstraction
    Name(&'a str),
}

impl std::fmt::Display for ZipArchiveIdentifier<'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::Path(path) => write!(f, "{}", path.to_string_lossy()),
            Self::Name(name) => write!(f, "{name}"),
        }
    }
}

pub enum ZipArchive{
    Memory{identif: String, contents: Vec<u8>},
    File{identif: PathBuf, file: std::fs::File},
}

impl ZipArchive{
    pub fn identifier(&self) -> ZipArchiveIdentifier<'_>{
        match self{
            Self::Memory { identif, .. } => ZipArchiveIdentifier::Name(identif),
            Self::File { identif, .. } => ZipArchiveIdentifier::Path(identif),
        }
    }
    pub fn open<P: AsRef<Path>>(p: P) -> Result<Self, std::io::Error>{
        let file = std::fs::File::open(p.as_ref())?;
        Ok(Self::File {
            identif: p.as_ref().to_owned(),
            file,
        })
    }
    pub fn from_raw_data(contents: Vec<u8>, identif: String) -> Self{
        ZipArchive::Memory{identif, contents}
    }
    pub fn with_entry<F, Out>(&self, name: &str, entry_reader: F) -> Result<Out, std::io::Error>
    where
        F: FnOnce(&mut zip::read::ZipFile<'_, BoxDynHasCursor>) -> Out,
        Out: 'static,
    {
        let mut archive_guard = self.archive.lock().unwrap();
        let mut f = archive_guard.by_name(name)?;
        let out = entry_reader(&mut f);
        Ok(out)
    }
    pub fn has_entry(&self, name: &str) -> bool{
        self.archive.lock().unwrap().by_name(name).is_ok()
    }
    pub fn with_file_names<F, Out>(&self, f: F) -> Out
    where
        F: for<'a> FnOnce(Box<dyn Iterator<Item=&'a str> + 'a>) -> Out,
        Out: 'static,
    {
        let archive_guard = self.archive.lock().unwrap();
        let file_names = Box::new(archive_guard.file_names());
        f(file_names)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RdfFileReferenceReadError{
    #[error("{0}")]
    ZipError(#[from] zip::result::ZipError),
    #[error("Url file reference not supported yet")]
    UrlFileReferenceNotSupportedYet,
}

pub trait RdfFileReferenceExt{
    fn try_read<F, Out>(
        &self, archive: &SharedZipArchive, reader: F
    ) -> Result<Out, RdfFileReferenceReadError>
    where
        F: FnOnce(&mut zip::read::ZipFile<'_, BoxDynHasCursor>) -> Out,
        Out: 'static;
}
impl RdfFileReferenceExt for rdf::FileReference{
    fn try_read<F, Out>(&self, archive: &SharedZipArchive, reader: F) -> Result<Out, RdfFileReferenceReadError>
    where
        F: FnOnce(&mut zip::read::ZipFile<'_, BoxDynHasCursor>) -> Out,
        Out: 'static,
    {
        let inner_path: String = match self{
            rdf::FileReference::Url(_) => return Err(RdfFileReferenceReadError::UrlFileReferenceNotSupportedYet),
            rdf::FileReference::Path(path) => path.into(),
        };
        Ok(archive.with_entry(&inner_path, reader)?)
    }
}
