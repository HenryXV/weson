use crate::utils::format;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style;
use ratatui::style::{Color, Style};
use ratatui::widgets::{List, ListItem, Widget};

#[derive(Default, Clone)]
pub struct AudioList {
    audios: Vec<String>,
}

impl Widget for AudioList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.audios.is_empty() {
            buf.set_string(
                area.x,
                area.y,
                "EMPTY",
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Rgb(255, 0, 0))
                    .add_modifier(style::Modifier::BOLD),
            );
        }

        let style = Style::default()
            .fg(Color::Yellow)
            .add_modifier(style::Modifier::BOLD)
            .add_modifier(style::Modifier::REVERSED);

        List::new(
            self.audios
                .iter()
                .map(|entry| {
                    let path = format::get_formatted_path(entry, area.width as usize - 4);
                    ListItem::new(path)
                })
                .collect::<Vec<_>>(),
        )
        .highlight_style(style)
        .render(area, buf);
    }
}

impl AudioList {
    pub fn audios(mut self, audios: Vec<String>) -> AudioList {
        self.audios = audios;
        self
    }
}
