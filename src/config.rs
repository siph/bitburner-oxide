use anyhow::Result;
use std::{
    path::Path, 
    fs,
};
use log::{
    info, 
    debug,
};
use clap::Parser;
use crate::app_args::AppArgs;


pub fn get_config() -> Result<Config> {
    let args = AppArgs::parse();
    let directory = match args.directory {
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
            match args.bearer_token {
                Some(val) => val.to_string(),
                None => panic!("Must set a token value through --token; or place it in a file named 'token'")
            }
        }
    };
    Ok(Config {
        bearer_token: String::from(token),
        directory: directory,
        ..Default::default()
    })
}

pub fn get_mock_config() -> Result<Config> {
    let config = Config { bearer_token: String::from("token"), 
        port: String::from("9990"),
        url: String::from("url"),
        valid_extensions: vec![String::from("")],
        directory: String::from("") };
    Ok(config)
}

#[derive(Debug)]
pub struct Config {
    pub bearer_token: String,
    pub port: String,
    pub url: String,
    pub valid_extensions: Vec<String>,
    pub directory: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            bearer_token: String::from(""),
            port: String::from("9990"),
            url: String::from("http://localhost"),
            valid_extensions: vec!["script".to_string(), "js".to_string(), "ns".to_string(), "txt".to_string()],
            directory: String::from(""),
        }
    }
}


