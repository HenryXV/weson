use std::fs::DirEntry;

#[derive(Clone, Debug)]
pub struct DirectoryEntry {
    path: String,
    name: String,
    is_selected: bool,
}

impl DirectoryEntry {
    pub fn from(dir_entry: DirEntry) -> Self {
        Self {
            path: dir_entry.path().to_str().unwrap().to_string(),
            name: dir_entry.file_name().to_str().unwrap().to_string(),
            is_selected: false,
        }
    }
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_selected(&self) -> bool {
        self.is_selected
    }

    pub fn set_is_selected(&mut self, is_selected: bool) {
        self.is_selected = is_selected;
    }
}
