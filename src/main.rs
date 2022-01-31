#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate serde;

use env_logger::Env;
use std::fs;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;
use clap::App;
use notify::{DebouncedEvent, RecursiveMode, Watcher, watcher};
use notify::DebouncedEvent::{Create, Remove, Write, Chmod};
use reqwest::{Error, Response};
use serde::{Serialize};
use futures::executor::block_on;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "info")
        .write_style_or("LOG_STYLE", "always");
    env_logger::init_from_env(env);
    let config = get_config()?;
    info!("bitburner-oxide initialized with config:");
    info!("{:?}", &config);
    let (sender, receiver) = channel();
    let mut watcher = watcher(sender, Duration::from_secs(2)).unwrap();
    watcher.watch(&config.directory, RecursiveMode::NonRecursive).unwrap();
    loop {
        match receiver.recv() {
            Ok(event) => handle_event(&config, &event).unwrap(),
            Err(e) => error!("error: {:?}", e),
        }
    }
}

fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    let yaml = load_yaml!("cli.yaml");
    let arg_matches = App::from_yaml(yaml).get_matches();
    let directory = match arg_matches.value_of("directory") {
        Some(val) => String::from(val),
        None => String::from(std::env::current_dir().unwrap().to_str().unwrap())
    };
    let token_path = String::from(Path::new(&directory).join("token").to_str().unwrap());
    debug!("looking for token at: {:?}", &token_path);
    let token = match fs::read_to_string(token_path) {
        Ok(val) => {
            info!("Found token file");
            String::from(val.trim())
        },
        Err(_) => {
            match arg_matches.value_of("token") {
                Some(val) => val.to_string(),
                None => panic!("Must set a token value through --token; or place in file name 'token'")
            }
        }
    };
    let url = String::from("http://localhost");
    Ok(Config {
        bearer_token: String::from(token),
        port: String::from("9990"),
        url: url,
        valid_extensions: vec!["script".to_string(), "js".to_string(), "ns".to_string(), "txt".to_string()],
        directory: directory
    })
}

fn handle_event(config: &Config, event: &DebouncedEvent) -> Result<(), Box<dyn std::error::Error>> {
    debug!("event: {:?}", event);
    match event {
        Write(p) | Create(p) | Chmod(p) => {
            if p.extension().is_some() && config.valid_extensions.contains(&p.extension().unwrap().to_str().unwrap().to_owned()) {
                // file contents must be encoded to base64
                let code = base64::encode(fs::read_to_string(p.as_path()).unwrap());
                let filename = String::from(p.file_name().unwrap().to_str().unwrap());
                info!("file change detected for file: {:?}", &filename);
                let request = BitburnerRequest {
                    filename,
                    code
                };
                block_on(send_file(config, request))?;               
            }
        },
        Remove(path) => trace!("file deleted: {:?}",path.file_name().unwrap()),
        _ => ()
    }
    Ok(())
}

async fn send_file(config: &Config, bitburner_request: BitburnerRequest) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    let body = serde_json::to_string(&bitburner_request).unwrap();
    let url = format!("{}:{}", config.url, config.port);
    let token = config.bearer_token.clone();
    return client.put(url)
        .bearer_auth(token)
        .body(body)
        .send()
        .await
}

#[derive(Debug)]
struct Config {
    bearer_token: String,
    port: String,
    url: String,
    valid_extensions: Vec<String>,
    directory: String,
}

#[derive(Debug, Serialize)]
struct BitburnerRequest {
    filename: String,
    code: String,
}
