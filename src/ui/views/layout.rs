use crate::ui::widgets::dir_list::DirList;
use crate::ui::widgets::dir_list_expanded::DirListExpanded;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Widget};

#[derive(Default)]
pub struct DirectoriesLayout {
    dir_list: DirList,
    dir_list_expanded: DirListExpanded,
}

impl Widget for DirectoriesLayout {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let total_ratio: u32 = 2 + 4 + 4;
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Ratio(2u32, total_ratio),
                    Constraint::Ratio(4u32, total_ratio),
                    Constraint::Ratio(4u32, total_ratio),
                ]
                .as_ref(),
            )
            .split(area);

        let inner_chunks_left = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .horizontal_margin(2)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(chunks[0]);

        let inner_chunks_middle = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .horizontal_margin(2)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(chunks[1]);

        self.default_block().render(chunks[0], buf);
        self.default_block().render(chunks[1], buf);

        self.dir_list.render(inner_chunks_left[0], buf);
        self.dir_list_expanded.render(inner_chunks_middle[0], buf);
    }
}

impl DirectoriesLayout {
    pub fn dir_list(mut self, dir_list: DirList) -> DirectoriesLayout {
        self.dir_list = dir_list;
        self
    }

    pub fn dir_list_expanded(mut self, dir_list_expanded: DirListExpanded) -> DirectoriesLayout {
        self.dir_list_expanded = dir_list_expanded;
        self
    }

    fn default_block(&self) -> Block {
        Block::default()
            .borders(Borders::all())
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(Color::Black))
    }
}
