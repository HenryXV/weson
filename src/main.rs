mod fs;
mod run;
mod ui;
mod utils;
mod context;
mod events;
mod audio_player;

use crate::ui::backend::Backend;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut backend = Backend::new();
    run::run_loop(&mut backend)?;
    backend.quit()?;
    Ok(())
}
