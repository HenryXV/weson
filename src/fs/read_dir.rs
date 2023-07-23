use crate::fs::dir_entry::DirectoryEntry;
use std::fs;

pub fn from(path: &str) -> Vec<DirectoryEntry> {
    let mut dir_entries: Vec<DirectoryEntry> = Vec::new();

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(dir_entry) => {
                        dir_entries.push(DirectoryEntry::from(dir_entry));
                    }
                    Err(err) => eprintln!("Error reading directory entry: {}", err),
                }
            }
        }
        Err(err) => eprintln!("Error reading directory: {}", err),
    }

    dir_entries
}
