use crate::fs::dir_entry::DirectoryEntry;
use std::fs;

pub fn from(path: &str) -> Vec<DirectoryEntry> {
    let mut dir_entries: Vec<DirectoryEntry> = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            match entry {
                Ok(dir_entry) => {
                    if let Some(e) = DirectoryEntry::from(dir_entry) {
                        dir_entries.push(e);
                    }
                }
                Err(_) => continue,
            }
        }
    }

    dir_entries
}
