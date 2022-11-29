#[macro_use]
extern crate log;

extern crate serde;
pub mod bitburner;
pub mod config;
use crate::bitburner::{message::BitburnerMessage, operation::BitburnerOperation};
use anyhow::Result;
use config::Config;
use env_logger::WriteStyle;
use jsonrpc_ws_server::{jsonrpc_core::IoHandler, ServerBuilder};
use log::LevelFilter;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::{fs, path::PathBuf, sync::mpsc::channel};
use serde_json;

fn main() -> Result<()> {
    let config: Config = match fs::File::open("filesync.json") {
        Ok(filesync) => serde_json::from_reader(filesync).expect("unable to parse `filesync.json`"),
        Err(_) => {
            let filesync = Config::default();
            fs::write("filesync.json", serde_json::to_string_pretty(&filesync)?)?;
            filesync
        }
    };
    env_logger::builder()
        .write_style(WriteStyle::Always)
        .filter(
            None,
            match &config.quiet {
                true => LevelFilter::Error,
                false => LevelFilter::Info,
            },
        )
        .init();
    info!("bitburner-oxide initialized with config:");
    info!("{:#?}", &config);
    let server = ServerBuilder::new(IoHandler::default())
        .start(&format!("127.0.0.1:{}", &config.port).parse()?)?;
    let (sender, receiver) = channel();
    let mut watcher = RecommendedWatcher::new(sender, notify::Config::default())?;
    watcher.watch(&config.scripts_folder, RecursiveMode::Recursive)?;
    for result in receiver {
        match result {
            Ok(event) => {
                if event
                    .clone()
                    .paths
                    .into_iter()
                    .all(|it| is_valid_file(&config, &it))
                    && !config.dry
                {
                    let operation = BitburnerOperation::build_operation(&config, event)?;
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

fn is_valid_file(config: &Config, path_buf: &PathBuf) -> bool {
    path_buf
        .extension()
        .map(|ex| ex.to_str().unwrap_or("").to_string())
        .map(|s| config.allowed_filetypes.contains(&s))
        .unwrap_or(false)
}
