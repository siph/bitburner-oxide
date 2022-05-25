#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate serde;

pub mod bitburner;
pub mod config;
pub mod app_args;
pub mod handler;

use once_cell::sync::Lazy;
use anyhow::Result;
use env_logger::Env;
use std::sync::mpsc::channel;
use std::time::Duration;
use notify::{
    RecursiveMode, 
    Watcher, 
    watcher
};
use handler::handle_event;
#[allow(unused_imports)]
use config::{ 
    Config, 
    get_config, 
    get_mock_config 
};

#[cfg(not(test))]
pub static CONFIG: Lazy<Config> = Lazy::new(|| { get_config().expect("Unable to initialize configuration") });
#[cfg(test)]
pub static CONFIG: Lazy<Config> = Lazy::new(|| { get_mock_config().expect("Unable to initialize configuration") });

fn main() -> Result<()> {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "info")
        .write_style_or("LOG_STYLE", "always");
    env_logger::init_from_env(env);
    let config = get_config()?;
    info!("bitburner-oxide version {:?}", crate_version!());
    info!("bitburner-oxide initialized with config:");
    info!("{:?}", &config);
    let (sender, receiver) = channel();
    let mut watcher = watcher(sender, Duration::from_secs(1))?;
    watcher.watch(&config.directory, RecursiveMode::Recursive)?;
    loop {
        match receiver.recv() {
            Ok(event) => handle_event(&event)?,
            Err(e) => error!("error: {:?}", e),
        }
    }
}

