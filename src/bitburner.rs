#[cfg(test)]
use mockito;
use serde::Serialize;
use crate::CONFIG;
use reqwest::{
    blocking::Response,
    Error,
};
use log::{ 
    info, 
    debug,
};

pub fn delete_file_from_server(bitburner_request: &BitburnerRequest) -> Result<Response, Error> {
    send_request(bitburner_request, reqwest::Method::DELETE)
}

pub fn write_file_to_server(bitburner_request: &BitburnerRequest) -> Result<Response, Error> {
    send_request(bitburner_request, reqwest::Method::PUT)
}

fn send_request(bitburner_request: &BitburnerRequest, method: reqwest::Method) -> Result<Response, Error> {
    #[cfg(not(test))]
    let url = format!("{}:{}", CONFIG.url, CONFIG.port);
    #[cfg(test)]
    let url = &mockito::server_url();
    let body = serde_json::to_string(&bitburner_request).unwrap();
    let client = reqwest::blocking::Client::new();
    let token = CONFIG.bearer_token.clone();
    info!("Sending request with body and url:");
    info!("Url: {:?}", &url);
    debug!("Body: {:?}", &body);
    info!("Token: {:?}", &token);
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

