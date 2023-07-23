use crate::fs;
use crate::fs::dir_entry::DirectoryEntry;
use ratatui::widgets::ListState;

pub struct DirListState<'a> {
    items: &'a mut Vec<DirectoryEntry>,
    state: ListState,
}

impl<'a> DirListState<'a> {
    pub fn new(items: &'a mut Vec<DirectoryEntry>) -> Self {
        let mut list_state = Self {
            items,
            state: ListState::default(),
        };

        list_state.next();

        list_state
    }

    pub fn set_items(&mut self, items: Vec<DirectoryEntry>) {
        self.items.clear();
        self.items.clone_from(&items);
        self.state = ListState::default();

        if !self.items.is_empty() {
            self.next();
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn get_selected_entry(&self) -> Option<&DirectoryEntry> {
        if let Some(index) = self.state.selected() {
            return self.items.get(index);
        }

        None
    }

    pub fn enter_selected_dir(&mut self) {
        if let Some(entry) = self.get_selected_entry() {
            if let Some(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    let dirs = fs::read_dir::from(entry.path());

                    if !dirs.is_empty() {
                        self.set_items(dirs);
                    }
                }
            }
        }
    }

    pub fn unselect(&mut self) {
        if let Some(entry) = self.get_selected_entry() {
            let previous_dir = entry.parent().parent();

            if let Some(path) = previous_dir {
                let dirs = fs::read_dir::from(path.to_str().unwrap());
                self.set_items(dirs);
            }
        }
    }

    pub fn state_mut(&mut self) -> &mut ListState {
        &mut self.state
    }
}
