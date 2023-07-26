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
    let home_dir_parent = dirs_next::home_dir()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    let mut dir_entries = fs::read_dir::from(&home_dir_parent);

    let mut app = App::new(&mut dir_entries, &home_dir_parent);

    let mut player = Player::default();

    let (tx, mut rx) = channel(50);

    let mut queue = Queue::new(tx);

    while !app.get_quit() {
        if player.queue_len() == 0 {
            if let Ok(source) = rx.try_recv() {
                player.play(source)
            }
        }

        let mut dir_list = DirList::default();

        let selected_dir_entry = app.get_focused_view_state().get_selected_entry();

        if let Some(entry) = selected_dir_entry {
            dir_list = dir_list.dir_entries(fs::read_dir::from(
                &entry.path().parent().unwrap().to_path_buf(),
            ));
        }

        let audio_list = AudioList::default();

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
