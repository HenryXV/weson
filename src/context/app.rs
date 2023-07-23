use crate::context::dir_list_state::DirListState;
use crate::fs::dir_entry::DirectoryEntry;

#[derive(Hash, PartialEq, Eq)]
enum FocusedView {
    Current,
    Expanded,
}

pub struct App<'a> {
    dir_list_state: DirListState<'a>,
    dir_list_expanded_state: DirListState<'a>,
    focused_view: FocusedView,
    quit: bool,
}

impl<'a> App<'a> {
    pub fn new(
        current_dirs: &'a mut Vec<DirectoryEntry>,
        dirs_expanded: &'a mut Vec<DirectoryEntry>,
    ) -> Self {
        Self {
            dir_list_state: DirListState::new(current_dirs),
            dir_list_expanded_state: DirListState::new(dirs_expanded),
            focused_view: FocusedView::Current,
            quit: false,
        }
    }

    pub fn get_focused_view_state(&mut self) -> &mut DirListState<'a> {
        match self.focused_view {
            FocusedView::Current => &mut self.dir_list_state,
            FocusedView::Expanded => &mut self.dir_list_expanded_state,
        }
    }

    pub fn get_expanded_view_state(&mut self) -> &mut DirListState<'a> {
        &mut self.dir_list_expanded_state
    }

    pub fn quit(&mut self) {
        self.quit = true
    }

    pub fn get_quit(&self) -> bool {
        self.quit
    }
}
