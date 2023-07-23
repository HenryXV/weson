use crate::fs::dir_entry::DirectoryEntry;
use ratatui::widgets::ListState;

pub struct DirListState<'a> {
    items: &'a Vec<DirectoryEntry>,
    state: ListState,
}

impl<'a> DirListState<'a> {
    pub fn new(items: &'a Vec<DirectoryEntry>) -> Self {
        Self {
            items,
            state: ListState::default(),
        }
    }

    pub fn set_items(&mut self, items: &'a Vec<DirectoryEntry>) {
        self.items = items;
        self.state = ListState::default();
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

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
