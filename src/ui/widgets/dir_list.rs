use crate::fs::dir_entry::DirectoryEntry;
use crate::utils::format;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style;
use ratatui::style::{Color, Style};
use ratatui::widgets::Widget;

#[derive(Default, Clone)]
pub struct DirList {
    dir_entries: Vec<DirectoryEntry>,
}

impl Widget for DirList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut y = area.y;

        for entry in self.dir_entries {
            let path = format::get_formatted_path(entry.name(), area.width as usize - 3);

            let style = match entry.is_selected() {
                false => Style::default(),
                true => Style::default()
                    .fg(Color::Red)
                    .add_modifier(style::Modifier::BOLD)
                    .add_modifier(style::Modifier::REVERSED),
            };

            if y <= area.height + 1 {
                let area = Rect::new(area.x, y, area.width, 1);
                buf.set_string(area.x, y, path, style);
                buf.set_style(area, style);
                y += 1;
            }
        }
    }
}

impl DirList {
    pub fn dir_entries(&self) -> &Vec<DirectoryEntry> {
        &self.dir_entries
    }

    pub fn set_dir_entries(mut self, dir_entries: Vec<DirectoryEntry>) -> DirList {
        self.dir_entries = dir_entries;
        self
    }

    pub fn get_selected_dir_entry(&self) -> Option<&DirectoryEntry> {
        self.dir_entries.iter().find(|&entry| entry.is_selected())
    }
}
