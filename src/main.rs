#[macro_use]
extern crate clap;
extern crate serde;
use clap::App;
use reqwest::{Error, Response};
use serde::{Serialize};

fn main()  -> Result<(), &'static str> {
    let yaml = load_yaml!("cli.yaml");
    let arg_matches = App::from_yaml(yaml).get_matches();
    let token = arg_matches.value_of("token").unwrap();
    let config = GameConfig {
        bearer_token: token.to_string(),
        port: String::from("9990"),
        url: String::from("localhost") };
    Ok(())
}

async fn send_file(config: GameConfig, bitburner_request: BitburnerRequest) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    let body = serde_json::to_string(&bitburner_request).unwrap();
    let url = format!("{}:{}", config.url, config.port);
    return client.put(url)
        .bearer_auth(config.bearer_token)
        .body(body)
        .send()
        .await
}

#[derive(Debug)]
struct GameConfig {
    bearer_token: String,
    port: String,
    url: String,
}

#[derive(Debug, Serialize)]
struct BitburnerRequest {
    filename: String,
    code: String,
}
