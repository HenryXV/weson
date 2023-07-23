use std::fs::{DirEntry, Metadata};
use std::path::PathBuf;

#[derive(Clone, Debug, Default)]
pub struct DirectoryEntry {
    path: String,
    name: String,
    parent_path: PathBuf,
    metadata: Option<Metadata>,
    is_selected: bool,
}

impl DirectoryEntry {
    pub fn from(dir_entry: DirEntry) -> Option<Self> {
        let dir_entry = Self {
            path: dir_entry.path().to_str().unwrap().to_string(),
            name: dir_entry.file_name().to_str().unwrap().to_string(),
            parent_path: dir_entry.path().parent().unwrap().to_path_buf(),
            metadata: Some(dir_entry.metadata().unwrap()),
            is_selected: false,
        };

        if dir_entry.name != "desktop.ini" {
            return Some(dir_entry);
        }

        None
    }
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn parent(&self) -> &PathBuf {
        &self.parent_path
    }

    pub fn is_selected(&self) -> bool {
        self.is_selected
    }

    pub fn set_is_selected(&mut self, is_selected: bool) {
        self.is_selected = is_selected;
    }

    pub fn metadata(&self) -> &Option<Metadata> {
        &self.metadata
    }
}
