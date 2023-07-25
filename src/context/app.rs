use crate::context::dir_list_state::DirListState;
use crate::fs::dir_entry::DirectoryEntry;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct App<'a> {
    visited_dirs: HashMap<PathBuf, DirListState<'a>>,
    dir_list_state: DirListState<'a>,
    current_dir: &'a PathBuf,
    quit: bool,
}

impl<'a> App<'a> {
    pub fn new(current_dirs: &'a mut Vec<DirectoryEntry>, parent_path: &'a PathBuf) -> Self {
        let visited_dirs: HashMap<PathBuf, DirListState<'a>> = HashMap::new();

        // visited_dirs.insert(parent_path.clone(), DirListState::new(current_dirs));

        Self {
            visited_dirs,
            dir_list_state: DirListState::new(current_dirs),
            current_dir: parent_path,
            quit: false,
        }
    }

    pub fn get_focused_view_state(&mut self) -> &mut DirListState<'a> {
        &mut self.dir_list_state
    }

    pub fn add_visited_dir(&mut self, path: &PathBuf, dirs: &'a mut Vec<DirectoryEntry>) {
        self.visited_dirs
            .insert(path.clone(), DirListState::new(dirs));
    }

    pub fn set_current_dir(&mut self, path: &'a PathBuf) {
        self.current_dir = path;
    }

    pub fn quit(&mut self) {
        self.quit = true
    }

    pub fn get_quit(&self) -> bool {
        self.quit
    }
}
