use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Widget};
use std::rc::Rc;

#[derive(Default)]
pub struct DirectoriesLayout {}

impl Widget for DirectoriesLayout {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = DirectoriesLayout::default_layout(area);

        DirectoriesLayout::default_block().render(chunks[0], buf);
        DirectoriesLayout::default_block().render(chunks[1], buf);
    }
}

impl DirectoriesLayout {
    pub fn default_layout(area: Rect) -> Rc<[Rect]> {
        let total_ratio: u32 = 5 + 5;

        Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Ratio(5u32, total_ratio),
                    Constraint::Ratio(5u32, total_ratio),
                ]
                .as_ref(),
            )
            .split(area)
    }

    pub fn get_inner_chunk(area: Rect) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .horizontal_margin(2)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(area)
    }

    fn default_block<'a>() -> Block<'a> {
        Block::default()
            .borders(Borders::all())
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(Color::Black))
    }
}
