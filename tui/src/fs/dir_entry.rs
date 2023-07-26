use std::fs::{DirEntry, Metadata};
use std::path::PathBuf;

#[derive(Clone, Debug, Default)]
pub struct DirectoryEntry {
    path: PathBuf,
    name: String,
    metadata: Option<Metadata>,
}

impl DirectoryEntry {
    pub fn from(dir_entry: DirEntry) -> Option<Self> {
        let dir_entry = Self {
            path: dir_entry.path().to_path_buf(),
            name: dir_entry.file_name().to_str().unwrap().to_string(),
            metadata: Some(dir_entry.metadata().unwrap()),
        };

        if dir_entry.name != "desktop.ini" {
            return Some(dir_entry);
        }

        None
    }
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn metadata(&self) -> &Option<Metadata> {
        &self.metadata
    }
}
