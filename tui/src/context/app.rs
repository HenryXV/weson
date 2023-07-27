use crate::context::list_state::StatefulList;
use crate::fs;
use crate::fs::dir_entry::DirectoryEntry;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct App {
    visited_dirs: HashMap<PathBuf, StatefulList<DirectoryEntry>>,
    current_dir_state: StatefulList<DirectoryEntry>,
    quit: bool,
}

impl App {
    pub fn new(current_dirs: Vec<DirectoryEntry>, parent_path: &Path) -> Self {
        let mut visited_dirs: HashMap<PathBuf, StatefulList<DirectoryEntry>> = HashMap::new();

        let state = StatefulList::new(current_dirs.clone());
        visited_dirs.insert(PathBuf::from(parent_path.clone()), state.clone());

        Self {
            visited_dirs,
            current_dir_state: state,
            quit: false,
        }
    }

    pub fn enter_selected_dir(&mut self) {
        let current_state = self.current_dir_state();

        if let Some(entry) = current_state.get_selected_entry() {
            match self.visited_dirs.get(entry.path()) {
                Some(state) => {
                    log::info!("Using cached dirs to path {}", entry.name());
                    self.set_current_dir(state.clone())
                }
                None => {
                    if let Some(metadata) = entry.metadata() {
                        if metadata.is_dir() {
                            let dirs = fs::read_dir::from(entry.path());

                            if !dirs.is_empty() {
                                log::info!("Creating new hash: {}", entry.name());
                                let state = StatefulList::new(dirs);
                                self.visited_dirs
                                    .insert(entry.path().clone(), state.clone());
                                self.set_current_dir(state.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn go_back(&mut self) {
        let focused_state = self.current_dir_state();

        if let Some(entry) = focused_state.get_selected_entry() {
            if let Some(previous_dir) = entry.path().parent().unwrap().parent() {
                match self.visited_dirs.get(&*previous_dir.to_path_buf()) {
                    Some(state) => {
                        log::info!(
                            "Using cached dirs to path {:?}",
                            previous_dir.file_name().unwrap()
                        );
                        self.set_current_dir(state.clone())
                    }
                    None => {
                        log::info!("Creating new hash: {:?}", previous_dir.file_name().unwrap());
                        let dirs = fs::read_dir::from(&previous_dir.to_path_buf());

                        let state = StatefulList::new(dirs);
                        self.visited_dirs
                            .insert(previous_dir.to_path_buf().clone(), state.clone());
                        self.set_current_dir(state.clone());
                    }
                }
            }
        }
    }

    pub fn current_dir_state(&self) -> &StatefulList<DirectoryEntry> {
        &self.current_dir_state
    }

    pub fn get_focused_view_state_mut(&mut self) -> &mut StatefulList<DirectoryEntry> {
        &mut self.current_dir_state
    }

    pub fn set_current_dir(&mut self, state: StatefulList<DirectoryEntry>) {
        self.current_dir_state = state.clone();
    }

    pub fn quit(&mut self) {
        self.quit = true
    }

    pub fn get_quit(&self) -> bool {
        self.quit
    }
}
