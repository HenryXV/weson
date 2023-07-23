use crate::context::app::App;
use crate::fs;
use crate::fs::dir_entry::DirectoryEntry;
use crate::ui::backend::Backend;
use crate::ui::views::layout::DirectoriesLayout;
use crate::ui::widgets::dir_list::DirList;
use crate::ui::widgets::dir_list_expanded::DirListExpanded;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use std::error::Error;
use std::time::Duration;

pub fn run_loop(backend: &mut Backend) -> Result<(), Box<dyn Error>> {
    let home_dir = dirs_next::home_dir().unwrap();

    let dir_entries: Vec<DirectoryEntry> =
        fs::read_dir::from(home_dir.parent().unwrap().to_path_buf().to_str().unwrap());
    let dirs_expanded: Vec<DirectoryEntry> = fs::read_dir::from(home_dir.to_str().unwrap());

    let app = App::new(&dir_entries, &dirs_expanded);
    loop {
        let dir_list = DirList::default().set_dir_entries(dir_entries.clone());
        let dir_list_expanded =
            DirListExpanded::default().selected_dir_entry(dirs_expanded.clone());
        let layout = DirectoriesLayout::default()
            .dir_list(dir_list.clone())
            .dir_list_expanded(dir_list_expanded);

        backend.render_widget(layout)?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => {
                        println!("pressed up");
                    }
                    _ => continue,
                };
            }
        }
    }

    Ok(())
}
