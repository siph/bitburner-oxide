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
use serde::{Serialize};
#[cfg(test)]
use mockito;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "info")
        .write_style_or("LOG_STYLE", "always");
    env_logger::init_from_env(env);
    let config = get_config()?;
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
                None => panic!("Must set a token value through --token; or place it in a file named 'token'")
            }
        }
    };
    Ok(Config {
        bearer_token: String::from(token),
        port: String::from("9990"),
        url: String::from("http://localhost"),
        valid_extensions: vec!["script".to_string(), "js".to_string(), "ns".to_string(), "txt".to_string()],
        directory: directory
    })
}

fn handle_event(config: &Config, event: &DebouncedEvent) -> Result<(), Box<dyn std::error::Error>> {
    debug!("event: {:?}", event);
    match event {
        Write(file) | Create(file) | Chmod(file) => {
            if file.extension().is_some() && config.valid_extensions.contains(&file.extension().unwrap().to_str().unwrap().to_owned()) {
                let code = base64::encode(fs::read_to_string(file.as_path()).unwrap());
                let filename = String::from(file.file_name().unwrap().to_str().unwrap());
                info!("file change detected for file: {:?}", &filename);
                let request = BitburnerRequest {
                    filename,
                    code
                };
                match send_file(config, request) {
                    Ok(res) => debug!("Response: {:?}", res),
                    Err(e) => error!("Network error: {:?}", e)
                }
            }
        },
        Remove(path) => trace!("file deleted: {:?}",path.file_name().unwrap()),
        _ => ()
    }
    Ok(())
}

fn send_file(config: &Config, bitburner_request: BitburnerRequest) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let body = serde_json::to_string(&bitburner_request).unwrap();
    #[cfg(not(test))]
    let url = format!("{}:{}", config.url, config.port);
    #[cfg(test)]
    let url = &mockito::server_url();
    let token = config.bearer_token.clone();
    return client.put(url)
        .bearer_auth(token)
        .body(body)
        .send()
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use mockito::mock;
    use reqwest::StatusCode;
    use super::*;

    #[test]
    fn assert_write_even_is_successful() {
        let _m = mock("PUT", "/")
            .with_status(200)
            .with_body("written")
            .create();
        let config = get_mock_config();
        let event = DebouncedEvent::Write(PathBuf::from(""));
        assert!(handle_event(&config, &event).is_ok());
    }

    fn get_mock_request(filename: &str, code: &str) -> BitburnerRequest {
        BitburnerRequest {
            filename: String::from(filename),
            code: String::from(code),
        }
    }

    fn get_mock_config() -> Config {
        Config { bearer_token: String::from("token"), 
            port: String::from("9990"),
            url: String::from("url"),
            valid_extensions: vec![String::from("")],
            directory: String::from("") }
    }
}
