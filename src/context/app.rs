use crate::context::dir_list_state::DirListState;
use crate::fs::dir_entry::DirectoryEntry;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
enum FocusedView {
    Root,
    Expanded,
}

pub struct App<'a> {
    dir_list_states: HashMap<FocusedView, DirListState<'a>>,
    focused_view: FocusedView,
}

impl<'a> App<'a> {
    pub fn new(root_dirs: &'a Vec<DirectoryEntry>, dirs_expanded: &'a Vec<DirectoryEntry>) -> Self {
        let mut dir_list_states = HashMap::new();

        dir_list_states.insert(FocusedView::Root, DirListState::new(root_dirs));
        dir_list_states.insert(FocusedView::Expanded, DirListState::new(dirs_expanded));

        Self {
            dir_list_states,
            focused_view: FocusedView::Root,
        }
    }

    pub fn get_focused_view_state(&self) -> Option<&DirListState> {
        match self.focused_view {
            FocusedView::Root => self.dir_list_states.get(&FocusedView::Root),
            FocusedView::Expanded => self.dir_list_states.get(&FocusedView::Expanded),
        }
    }
}
