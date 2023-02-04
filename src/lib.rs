use std::{
    collections::{BTreeMap, BTreeSet},
    io,
    ops::{Deref, DerefMut},
};

use camino::{Utf8Path as Path, Utf8PathBuf as PathBuf};
use rm_document::{self as rm, TryLoad};
use thiserror::Error;

pub use rm::Entry;

#[derive(Debug, Error)]
pub enum LibraryError {
    #[error("i/o error: {0}")]
    Io(#[from] io::Error),
}

#[derive(Clone, Debug)]
pub struct Library {
    entries: BTreeMap<String, rm::Entry>,
    template_dir: Option<PathBuf>,
}

impl Library {
    pub fn load<P1: AsRef<Path>, P2: AsRef<Path>>(
        library_path: P1,
        template_dir: Option<P2>,
    ) -> Result<Self, LibraryError> {
        let paths: BTreeSet<_> = library_path
            .as_ref()
            .read_dir_utf8()?
            .filter_map(|d| {
                if let Ok(dirent) = d {
                    dirent.path().file_stem().map(String::from)
                } else {
                    None
                }
            })
            .collect();

        let entries = paths
            .iter()
            .filter_map(|p| {
                rm::Entry::try_load(library_path.as_ref().join(p))
                    .ok()
                    .map(|e| (p.into(), e))
            })
            .collect();

        Ok(Self {
            entries,
            template_dir: template_dir.map(|d| d.as_ref().into()),
        })
    }

    pub fn template_dir(&self) -> Option<PathBuf> {
        self.template_dir.clone()
    }
}

impl Deref for Library {
    type Target = BTreeMap<String, rm::Entry>;

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl DerefMut for Library {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entries
    }
}
