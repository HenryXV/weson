use crate::fs::dir_entry::DirectoryEntry;
use crate::ui::widgets::dir_list::DirList;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style;
use ratatui::style::{Color, Style};
use ratatui::widgets::{ListState, StatefulWidget};

#[derive(Default)]
pub struct DirListExpanded {
    dir_expanded: Vec<DirectoryEntry>,
}

impl StatefulWidget for DirListExpanded {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut ListState) {
        DirList::default()
            .dir_entries(self.dir_expanded)
            .render(area, buf, state);
    }
}

impl DirListExpanded {
    pub fn dir_expanded(mut self, dir_expanded: Vec<DirectoryEntry>) -> DirListExpanded {
        self.dir_expanded = dir_expanded;
        self
    }
}
