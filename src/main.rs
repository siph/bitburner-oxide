#[macro_use]
extern crate log;
extern crate serde;

pub mod bitburner;
pub mod config;
pub mod websocket;

use anyhow::Result;
use config::Config;
use env_logger::Env;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use once_cell::sync::Lazy;
use std::{path::PathBuf, sync::mpsc::channel};

use crate::{bitburner::operation::BitburnerOperation, websocket::client::send_message};

#[cfg(not(test))]
pub static CONFIG: Lazy<Config> = Lazy::new(|| confy::load("filesync", None).unwrap());
#[cfg(test)]
pub static CONFIG: Lazy<Config> = Lazy::new(|| Config {
    dry: true,
    ..Default::default()
});

fn main() -> Result<()> {
    let env = Env::default()
        .write_style("always")
        .filter(match &CONFIG.quiet {
            true => "info",
            false => "error",
        });
    env_logger::init_from_env(env);
    info!("bitburner-oxide initialized with config:");
    info!("{:#?}", &CONFIG);
    let (sender, receiver) = channel();
    let mut watcher = RecommendedWatcher::new(sender, notify::Config::default())?;
    watcher.watch(&CONFIG.scripts_folder, RecursiveMode::Recursive)?;
    for result in receiver {
        match result {
            Ok(event) => {
                if event.clone().paths.into_iter().all(|it| is_valid_file(&it)) {
                    send_message(BitburnerOperation::from(event))?;
                }
                return Ok(());
            }
            Err(e) => error!("error: {:#?}", e),
        }
    }
    Ok(())
}

fn is_valid_file(path_buf: &PathBuf) -> bool {
    path_buf
        .extension()
        .map(|ex| ex.to_str().unwrap_or("").to_string())
        .map(|s| CONFIG.allowed_filetypes.contains(&s))
        .unwrap_or(false)
}
