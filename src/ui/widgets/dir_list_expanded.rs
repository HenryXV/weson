use crate::fs;
use crate::ui::widgets::dir_list::DirList;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

#[derive(Default)]
pub struct DirListExpanded {
    dir_list: DirList,
}

impl Widget for DirListExpanded {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(entry) = self.dir_list.get_selected_dir_entry() {
            let dir_entries = fs::read_dir::read_dir_from(entry);
            DirList::default()
                .set_dir_entries(dir_entries)
                .render(area, buf)
        };
    }
}

impl DirListExpanded {
    pub fn dir_list(mut self, paths: DirList) -> DirListExpanded {
        self.dir_list = paths;
        self
    }
}
