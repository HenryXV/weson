use crate::context::app::App;
use crate::ui::views::layout::DirectoriesLayout;
use crate::ui::widgets::dir_list::DirList;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::error::Error;
use std::io;
use std::io::Stdout;

pub struct Backend {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Backend {
    pub fn new() -> Self {
        let terminal = Backend::setup_terminal().unwrap();

        Self { terminal }
    }

    pub fn quit(&mut self) -> Result<(), Box<dyn Error>> {
        self.restore_terminal()?;

        Ok(())
    }

    pub fn render_ui(
        &mut self,
        app: &mut App,
        layout: DirectoriesLayout,
        dir_list: DirList,
    ) -> Result<(), Box<dyn Error>> {
        self.terminal.draw(|frame| {
            let area = frame.size();

            frame.render_widget(layout, area);

            let chunks = DirectoriesLayout::default_layout(area);

            let inner_chunk_middle = DirectoriesLayout::get_inner_chunk(chunks[0]);

            frame.render_stateful_widget(
                dir_list,
                inner_chunk_middle[0],
                app.get_focused_view_state().state_mut(),
            );
        })?;

        Ok(())
    }

    fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
        let mut stdout = io::stdout();
        enable_raw_mode()?;
        execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
        Ok(Terminal::new(CrosstermBackend::new(stdout))?)
    }

    fn restore_terminal(&mut self) -> Result<(), Box<dyn Error>> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            crossterm::terminal::LeaveAlternateScreen
        )?;
        Ok(self.terminal.show_cursor()?)
    }
}
