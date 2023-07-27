use crate::context::app::App;
use crate::ui::views::layout::DirectoriesLayout;
use crate::ui::widgets::audio_list::AudioList;
use crate::ui::widgets::dir_list::DirList;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use sqlx::SqlitePool;
use std::error::Error;
use std::io::Stdout;
use std::{io, process};

pub struct Backend<'a> {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    pub pool: &'a SqlitePool,
}

impl<'a> Backend<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        let terminal = match Backend::setup_terminal() {
            Ok(terminal) => {
                log::info!("Backend initiated");
                terminal
            }
            Err(e) => {
                log::error!("Failed to initialize backend. {e}");
                process::exit(1);
            }
        };

        Self { terminal, pool }
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
        audio_list: AudioList,
    ) -> Result<(), Box<dyn Error>> {
        self.terminal.draw(|frame| {
            let area = frame.size();

            frame.render_widget(layout, area);

            let chunks = DirectoriesLayout::default_layout(area);

            let inner_chunk_left = DirectoriesLayout::get_inner_chunk(chunks[0]);
            let inner_chunk_right = DirectoriesLayout::get_inner_chunk(chunks[1]);

            frame.render_stateful_widget(
                dir_list,
                inner_chunk_left[0],
                app.get_focused_view_state_mut().state_mut(),
            );

            frame.render_widget(audio_list, inner_chunk_right[0]);
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
