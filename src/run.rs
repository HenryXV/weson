use crate::audio::player::Player;
use crate::context::app::App;
use crate::events::process_event::EventProcessor;
use crate::fs;
use crate::ui::backend::Backend;
use crate::ui::views::layout::DirectoriesLayout;
use crate::ui::widgets::dir_list::DirList;
use crossterm::event;
use crossterm::event::{Event, KeyEventKind};
use std::error::Error;
use std::time::Duration;

pub fn run_loop(backend: &mut Backend) -> Result<(), Box<dyn Error>> {
    let home_dir_parent = dirs_next::home_dir()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    let mut dir_entries = fs::read_dir::from(&home_dir_parent);

    let mut app = App::new(&mut dir_entries, &home_dir_parent);
    let player = Player::new();

    while !app.get_quit() {
        let mut dir_list = DirList::default();

        let selected_dir_entry = app.get_focused_view_state().get_selected_entry();

        if let Some(entry) = selected_dir_entry {
            dir_list = dir_list.dir_entries(fs::read_dir::from(
                &entry.path().parent().unwrap().to_path_buf(),
            ));
        }

        let layout = DirectoriesLayout::default();

        backend.render_ui(&mut app, layout, dir_list)?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    EventProcessor::process_key_press(&mut app, key.code, &player);
                }
            }
        }
    }

    Ok(())
}
