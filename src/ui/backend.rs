use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::{StatefulWidget, Widget};
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

    pub fn render_widget<W: Widget>(&mut self, widget: W) -> Result<(), Box<dyn Error>> {
        self.terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(widget, area);
        })?;

        Ok(())
    }

    pub fn render_stateful_widget<W: StatefulWidget>(
        &mut self,
        widget: W,
        state: &mut W::State,
    ) -> Result<(), Box<dyn Error>> {
        self.terminal.draw(|frame| {
            let area = frame.size();
            frame.render_stateful_widget(widget, area, state);
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
