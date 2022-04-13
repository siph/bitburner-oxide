#[cfg(test)]
use mockito;
use serde::Serialize;

use crate::config::Config;

pub fn delete_file_from_server(config: &Config, bitburner_request: &BitburnerRequest) -> Result<reqwest::blocking::Response, reqwest::Error> {
    send_request(config, bitburner_request, reqwest::Method::DELETE)
}

pub fn write_file_to_server(config: &Config, bitburner_request: &BitburnerRequest) -> Result<reqwest::blocking::Response, reqwest::Error> {
    send_request(config, bitburner_request, reqwest::Method::PUT)
}

pub fn send_request(config: &Config, bitburner_request: &BitburnerRequest, method: reqwest::Method) -> Result<reqwest::blocking::Response, reqwest::Error> {
    #[cfg(not(test))]
    let url = format!("{}:{}", config.url, config.port);
    #[cfg(test)]
    let url = &mockito::server_url();
    let body = serde_json::to_string(&bitburner_request).unwrap();
    let client = reqwest::blocking::Client::new();
    let token = config.bearer_token.clone();
    match method {
        reqwest::Method::PUT => client.put(url),
        reqwest::Method::DELETE => client.delete(url),
        _ => client.get(url),
    }.bearer_auth(token)
     .body(body)
     .send()
}

#[derive(Debug, Serialize)]
pub struct BitburnerRequest {
    pub filename: String,
    pub code: Option<String>,
}
