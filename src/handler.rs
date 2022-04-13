use std::{
    path::PathBuf, 
    fs
};
use log::{
    debug,
    info,
    error,
};
use notify::DebouncedEvent::{Write, Create, Chmod, Remove, self};
use crate::{
    config::Config, 
    bitburner::{
        BitburnerRequest, 
        write_file_to_server,
        delete_file_from_server,
    },
};

pub fn handle_event(config: &Config, event: &DebouncedEvent) -> Result<(), Box<dyn std::error::Error>> {
    debug!("event: {:?}", event);
    match event {
        Write(file) | Create(file) | Chmod(file) => {
            if is_valid_file(&file, &config) {
                let code = base64::encode(fs::read_to_string(file.as_path()).unwrap());
                let filename = String::from(file.file_name().unwrap().to_str().unwrap());
                info!("file change detected for file: {:?}", &filename);
                let bitburner_request = BitburnerRequest {
                    filename,
                    code: Some(code)
                };
                match write_file_to_server(config, &bitburner_request) {
                    Ok(res) => debug!("Response: {:?}", res),
                    Err(e) => error!("Network error: {:?}", e)
                }
            }
        },
        Remove(file) => {
            if is_valid_file(&file, &config) {
                let filename = String::from(file.file_name().unwrap().to_str().unwrap());
                info!("file deleted: {:?}",file.file_name().unwrap());
                let bitburner_request = BitburnerRequest {
                    filename: filename,
                    code: None
                };
                match delete_file_from_server(config, &bitburner_request) {
                    Ok(res) => debug!("Response: {:?}", res),
                    Err(e) => error!("Network error: {:?}", e)
                }
            }
        },
        unhandled_event => debug!("Unhandled event: {:?}", unhandled_event)
    }
    Ok(())
}

fn is_valid_file(path_buf: &PathBuf, config: &Config) -> bool {
    path_buf.extension().is_some() && config.valid_extensions.contains(&path_buf.extension().unwrap().to_str().unwrap().to_owned())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use mockito::mock;
    use notify::DebouncedEvent;
    use super::*;

    #[test]
    fn assert_write_event_is_successful() {
        let _m = mock("PUT", "/")
            .with_status(200)
            .with_body("written")
            .create();
        let config = get_mock_config();
        let event = DebouncedEvent::Write(PathBuf::from(""));
        assert!(handle_event(&config, &event).is_ok());
    }

    #[test]
    fn assert_remove_event_is_successful() {
        let _m = mock("DELETE", "/")
            .with_status(200)
            .with_body("deleted")
            .create();
        let config = get_mock_config();
        let event = DebouncedEvent::Remove(PathBuf::from(""));
        assert!(handle_event(&config, &event).is_ok());
    }

    fn get_mock_config() -> Config {
        Config { bearer_token: String::from("token"), 
            port: String::from("9990"),
            url: String::from("url"),
            valid_extensions: vec![String::from("")],
            directory: String::from("") }
    }
}
