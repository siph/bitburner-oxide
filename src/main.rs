#[macro_use]
extern crate clap;
use clap::App;
use reqwest::{Error, Response};

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

async fn get_response(config: GameConfig, filename: &str, code: &str) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    let bitburner_request = BitburnerRequest {
        filename: filename.to_string(),
        code: code.to_string()
    };
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
