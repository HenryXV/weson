use crate::context::app::App;
use audio::player::Player;
use audio::queue::Queue;
use crossterm::event::KeyCode;
use database::playlist::Playlist;
use sqlx::SqlitePool;

pub struct EventProcessor {}

impl EventProcessor {
    pub async fn process_key_press(
        app: &mut App,
        key: KeyCode,
        player: &mut Player,
        queue: &mut Queue,
        pool: &SqlitePool,
    ) {
        match key {
            KeyCode::Char('q') => app.quit(),
            KeyCode::Left => app.go_back(),
            KeyCode::Down => app.get_focused_view_state_mut().next(),
            KeyCode::Up => app.get_focused_view_state_mut().previous(),
            KeyCode::Right | KeyCode::Enter => app.enter_selected_dir(),
            KeyCode::Char('a') => {
                queue.add_audio(app.current_dir_state().get_selected_entry().unwrap().path())
            }
            KeyCode::Char('p') => player.pause(),
            KeyCode::Char('r') => player.resume(),
            KeyCode::Char('s') => Playlist::new("test".to_string(), "musica".to_string())
                .save_playlist(pool)
                .await
                .unwrap(),
            _ => {}
        }
    }
}
