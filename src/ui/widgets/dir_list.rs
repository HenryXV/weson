use crate::fs::dir_entry::DirectoryEntry;
use crate::utils::format;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style;
use ratatui::style::{Color, Style};
use ratatui::widgets::{List, ListItem, Widget};

#[derive(Default, Clone)]
pub struct DirList {
    dir_entries: Vec<DirectoryEntry>,
}

impl Widget for DirList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style = Style::default()
            .fg(Color::Red)
            .add_modifier(style::Modifier::BOLD)
            .add_modifier(style::Modifier::REVERSED);

        List::new(
            self.dir_entries
                .iter()
                .map(|entry| {
                    let path = format::get_formatted_path(entry.name(), area.width as usize - 4);
                    ListItem::new(path)
                })
                .collect::<Vec<_>>(),
        )
        .highlight_style(style)
        .render(area, buf);
    }
}

impl DirList {
    pub fn set_dir_entries(mut self, dir_entries: Vec<DirectoryEntry>) -> DirList {
        self.dir_entries = dir_entries;
        self
    }

    pub fn get_selected_dir_entry(&self) -> Option<&DirectoryEntry> {
        self.dir_entries.iter().find(|&entry| entry.is_selected())
    }
}
