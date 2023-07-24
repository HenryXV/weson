use crate::audio_player::player::Player;
use crate::context::app::App;
use crossterm::event::KeyCode;

pub struct EventProcessor {}

impl EventProcessor {
    pub fn process_key_press(app: &mut App, key: KeyCode, player: &Player) {
        match key {
            KeyCode::Char('q') => app.quit(),
            KeyCode::Left => app.get_focused_view_state().go_back(),
            KeyCode::Down => app.get_focused_view_state().next(),
            KeyCode::Up => app.get_focused_view_state().previous(),
            KeyCode::Right | KeyCode::Enter => app.get_focused_view_state().enter_selected_dir(),
            KeyCode::Char('p') => player.add(
                app.get_focused_view_state()
                    .get_selected_entry()
                    .unwrap()
                    .path(),
            ),
            KeyCode::Char('e') => player.pause(),
            KeyCode::Char('r') => player.resume(),
            _ => {}
        }
    }
}
