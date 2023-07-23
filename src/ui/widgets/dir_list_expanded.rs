use crate::fs::dir_entry::DirectoryEntry;
use crate::ui::widgets::dir_list::DirList;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

#[derive(Default)]
pub struct DirListExpanded {
    dirs_expanded: Vec<DirectoryEntry>,
}

impl Widget for DirListExpanded {
    fn render(self, area: Rect, buf: &mut Buffer) {
        DirList::default()
            .set_dir_entries(self.dirs_expanded)
            .render(area, buf)
    }
}

impl DirListExpanded {
    pub fn selected_dir_entry(mut self, dirs_expanded: Vec<DirectoryEntry>) -> DirListExpanded {
        self.dirs_expanded = dirs_expanded;
        self
    }
}
