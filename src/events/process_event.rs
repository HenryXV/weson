use crate::context::app::App;
use crossterm::event::KeyCode;

pub struct EventProcessor {}

impl EventProcessor {
    pub fn process_key_press(app: &mut App, key: KeyCode) {
        match key {
            KeyCode::Char('q') => app.quit(),
            KeyCode::Left => app.get_focused_view_state().unselect(),
            KeyCode::Down => app.get_focused_view_state().next(),
            KeyCode::Up => app.get_focused_view_state().previous(),
            KeyCode::Right => app.get_focused_view_state().enter_selected_dir(),
            _ => {}
        }
    }
}
