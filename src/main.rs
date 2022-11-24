#[macro_use]
extern crate log;
extern crate serde;

pub mod bitburner;
pub mod config;

use anyhow::Result;
use config::Config;
use env_logger::Env;
use jsonrpc_ws_server::{jsonrpc_core::IoHandler, ServerBuilder};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use once_cell::sync::Lazy;
use std::{path::PathBuf, sync::mpsc::channel};

use crate::bitburner::message::BitburnerMessage;
use crate::bitburner::operation::BitburnerOperation;

#[cfg(not(test))]
pub static CONFIG: Lazy<Config> = Lazy::new(|| confy::load("filesync", None).unwrap());
#[cfg(test)]
pub static CONFIG: Lazy<Config> = Lazy::new(|| Config {
    dry: true,
    ..Default::default()
});

fn main() -> Result<()> {
    println!("hi");
    let env = Env::default()
        .write_style("always")
        .filter(match &CONFIG.quiet {
            true => "error",
            false => "info",
        });
    println!("{:#?}", env);
    env_logger::init_from_env(env);
    info!("bitburner-oxide initialized with config:");
    info!("{:#?}", &CONFIG);
    let server = ServerBuilder::new(IoHandler::default())
        .start(&format!("127.0.0.1:{}", &CONFIG.port).parse()?)?;
    let (sender, receiver) = channel();
    let mut watcher = RecommendedWatcher::new(sender, notify::Config::default())?;
    watcher.watch(&CONFIG.scripts_folder, RecursiveMode::Recursive)?;
    for result in receiver {
        match result {
            Ok(event) => {
                if event.clone().paths.into_iter().all(|it| is_valid_file(&it)) && !CONFIG.quiet {
                    let operation = BitburnerOperation::from(event);
                    let messages: Vec<BitburnerMessage> = Vec::from(operation);
                    messages
                        .into_iter()
                        .for_each(|it| server.broadcaster().send(it).unwrap())
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
