use crate::utils::format;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::style;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{List, ListItem, Widget};

#[derive(Default, Clone)]
pub struct AudioList {
    audios: Vec<String>,
    current_audio: Option<String>,
}

impl Widget for AudioList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.audios.is_empty() {
            buf.set_string(
                area.x,
                area.y,
                "QUEUE IS EMPTY",
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
                    let path = format::get_formatted_name(entry, area.width as usize - 4);
                    ListItem::new(path)
                })
                .collect::<Vec<_>>(),
        )
        .highlight_style(style)
        .render(area, buf);

        if let Some(audio) = self.current_audio {
            let audio = format::get_formatted_name(audio.as_str(), area.width as usize - 4);
            let line = Line::from("Currently playing").alignment(Alignment::Center);
            buf.set_line(area.x, area.height - 1, &line, area.width);
            buf.set_string(area.x, area.height, audio, Style::default());
        }
    }
}

impl AudioList {
    pub fn audios(mut self, audios: Vec<String>) -> AudioList {
        self.audios = audios;
        self
    }

    pub fn current_audio(mut self, audio: Option<String>) -> AudioList {
        self.current_audio = audio;
        self
    }
}
