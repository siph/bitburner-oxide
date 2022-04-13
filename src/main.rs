#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate serde;

use env_logger::Env;
use std::sync::mpsc::channel;
use std::time::Duration;
use notify::{RecursiveMode, Watcher, watcher};
use bitburner_oxide::{
    handler::handle_event, 
    config::get_config,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "info")
        .write_style_or("LOG_STYLE", "always");
    env_logger::init_from_env(env);
    let config = get_config()?;
    info!("bitburner-oxide version {:?}", crate_version!());
    info!("bitburner-oxide initialized with config:");
    info!("{:?}", &config);
    let (sender, receiver) = channel();
    let mut watcher = watcher(sender, Duration::from_secs(1)).unwrap();
    watcher.watch(&config.directory, RecursiveMode::NonRecursive).unwrap();
    loop {
        match receiver.recv() {
            Ok(event) => handle_event(&config, &event).unwrap(),
            Err(e) => error!("error: {:?}", e),
        }
    }
}

