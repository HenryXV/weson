use crate::context::app::App;
use audio::player::Player;
use crossterm::event::KeyCode;
use database::playlist::Playlist;
use sqlx::SqlitePool;

pub struct EventProcessor {}

impl EventProcessor {
    pub async fn process_key_press<'a>(
        app: &mut App<'a>,
        key: KeyCode,
        player: &mut Player,
        pool: &SqlitePool,
    ) {
        match key {
            KeyCode::Char('q') => app.quit(),
            KeyCode::Left => app.get_focused_view_state().go_back(),
            KeyCode::Down => app.get_focused_view_state().next(),
            KeyCode::Up => app.get_focused_view_state().previous(),
            KeyCode::Right | KeyCode::Enter => app.get_focused_view_state().enter_selected_dir(),
            KeyCode::Char('a') => player.add_to_queue(
                app.get_focused_view_state()
                    .get_selected_entry()
                    .unwrap()
                    .path(),
            ),
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
