use crate::fs::dir_entry::DirectoryEntry;
use crate::ui::backend::Backend;
use crate::ui::views::layout::DirectoriesLayout;
use crate::ui::widgets::dir_list::DirList;
use crate::ui::widgets::dir_list_expanded::DirListExpanded;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use std::error::Error;
use std::fs;
use std::time::Duration;

pub fn run_loop(backend: &mut Backend) -> Result<(), Box<dyn Error>> {
    let home_dir = dirs_next::home_dir().unwrap();
    let mut dir_entries: Vec<DirectoryEntry> = Vec::new();

    match fs::read_dir(home_dir.parent().unwrap()) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(dir_entry) => {
                        let mut dir_entry = DirectoryEntry::from(dir_entry);

                        if dir_entry.name() == "henri" {
                            dir_entry.set_is_selected(true);
                        }
                        dir_entries.push(dir_entry);
                    }
                    Err(err) => eprintln!("Error reading directory entry: {}", err),
                }
            }
        }
        Err(err) => eprintln!("Error reading directory: {}", err),
    }

    loop {
        let dir_list = DirList::default().set_dir_entries(dir_entries.clone());
        let dir_list_expanded = DirListExpanded::default().dir_list(dir_list.clone());
        let layout = DirectoriesLayout::default()
            .dir_list(dir_list.clone())
            .dir_list_expanded(dir_list_expanded);

        backend.render_widget(layout)?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code {
                    break;
                }
            }
        }
    }

    Ok(())
}
