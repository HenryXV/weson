mod context;
mod events;
mod fs;
mod run;
mod ui;
pub mod utils;

use crate::ui::backend::Backend;
use crate::utils::logger::init_logger;
use database::init::get_database_pool;
use std::error::Error;
use std::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _logger = match init_logger() {
        Ok(logger) => {
            log::info!("Logger initiated.");
            logger
        }
        Err(e) => {
            log::error!("Error initializing logger: {e}");
            process::exit(1);
        }
    };

    let join_handle = tokio::spawn(async move { get_database_pool().await });

    let pool = match join_handle.await.unwrap() {
        Ok(db) => {
            log::info!("Database initiated.");
            db
        }
        Err(e) => {
            log::error!("Error initializing database: {e}");
            process::exit(1);
        }
    };

    let mut backend = Backend::new(&pool);
    match run::run_loop(&mut backend).await {
        Ok(_) => log::info!("App finished executing successfully"),
        Err(e) => log::error!("Error during program execution: {e}"),
    }

    backend.quit()?;
    Ok(())
}
