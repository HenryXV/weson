use crate::context::app::App;
use crate::events::process_event::EventProcessor;
use crate::fs;
use crate::ui::backend::Backend;
use crate::ui::views::layout::DirectoriesLayout;
use crate::ui::widgets::audio_list::AudioList;
use crate::ui::widgets::dir_list::DirList;
use audio::player::Player;
use audio::queue::Queue;
use crossterm::event;
use crossterm::event::{Event, KeyEventKind};
use std::error::Error;
use std::process;
use std::time::Duration;
use tokio::sync::mpsc::channel;

pub async fn run_loop<'a>(backend: &mut Backend<'a>) -> Result<(), Box<dyn Error>> {
    let audio_dir = dirs_next::audio_dir().unwrap().to_path_buf();

    let dir_entries = fs::read_dir::from(&audio_dir);

    let mut app = App::new(dir_entries, &audio_dir);

    let mut player = Player::default();

    let (tx, mut rx) = channel(50);

    let mut queue = Queue::new(tx);

    while !app.get_quit() {
        if player.queue_len() == 0 {
            if let Ok(source) = rx.try_recv() {
                let name = queue.audio_list().lock().unwrap().pop_front().unwrap();
                queue.set_current_audio(name);
                player.play(source)
            }
        }

        let dir_list = DirList::default().dir_entries(app.current_dir_state().items().clone());

        let audio_list = AudioList::default()
            .audios(Vec::from(queue.audio_list().lock().unwrap().clone()))
            .current_audio(queue.current_audio().clone());

        let layout = DirectoriesLayout::default();

        if let Err(e) = backend.render_ui(&mut app, layout, dir_list, audio_list) {
            log::error!("Error rendering UI. {e}");
            process::exit(1);
        }

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    EventProcessor::process_key_press(
                        &mut app,
                        key.code,
                        &mut player,
                        &mut queue,
                        backend.pool,
                    )
                    .await;
                }
            }
        }
    }

    Ok(())
}
