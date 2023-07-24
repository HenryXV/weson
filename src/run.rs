use crate::audio_player::player::Player;
use crate::context::app::App;
use crate::events::process_event::EventProcessor;
use crate::fs;
use crate::ui::backend::Backend;
use crate::ui::views::layout::DirectoriesLayout;
use crate::ui::widgets::dir_list::DirList;
use crate::ui::widgets::dir_list_expanded::DirListExpanded;
use crossterm::event;
use crossterm::event::{Event, KeyEventKind};
use std::error::Error;
use std::time::Duration;

pub fn run_loop(backend: &mut Backend) -> Result<(), Box<dyn Error>> {
    let home_dir = dirs_next::home_dir().unwrap();

    let mut previous_dir_entries =
        fs::read_dir::from(&home_dir.parent().unwrap().parent().unwrap().to_path_buf());
    let mut dir_entries = fs::read_dir::from(&home_dir.parent().unwrap().to_path_buf());
    let mut dirs_expanded = fs::read_dir::from(&home_dir.to_path_buf());

    let mut app = App::new(
        &mut previous_dir_entries,
        &mut dir_entries,
        &mut dirs_expanded,
    );
    let player = Player::new();
    while !app.get_quit() {
        let mut previous_dir_list = DirList::default();
        let mut dir_list = DirList::default();
        let mut dirs_expanded = DirListExpanded::default();

        let selected_dir_entry = app.get_focused_view_state().get_selected_entry();

        if let Some(entry) = selected_dir_entry {
            let previous_dir = entry.path().parent().unwrap().parent();

            if let Some(path) = previous_dir {
                previous_dir_list =
                    previous_dir_list.dir_entries(fs::read_dir::from(&path.to_path_buf()));
            }

            dir_list = dir_list.dir_entries(fs::read_dir::from(
                &entry.path().parent().unwrap().to_path_buf(),
            ));
            dirs_expanded = dirs_expanded.dir_expanded(fs::read_dir::from(entry.path()));
        }

        let layout = DirectoriesLayout::default();

        backend.render_ui(&mut app, layout, previous_dir_list, dir_list, dirs_expanded)?;

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
