use anyhow::{ Result, Context };
use std::{
    path::PathBuf,
    fs,
};
use log::{
    debug,
    info,
};
use notify::DebouncedEvent::{
    Write, 
    Create, 
    Chmod, 
    Remove, 
    Rename,
    self,
};
use crate::{
    CONFIG,
    bitburner::{
        BitburnerRequest, 
        write_file_to_server,
        delete_file_from_server,
    },
};

pub fn handle_event(event: &DebouncedEvent) -> Result<()> {
    match event {
        Write(file) | Create(file) | Chmod(file) => {
            if is_valid_file(&file) {
                info!("file change detected for file: {:#?}", &file);
                let bitburner_request = build_bitburner_request(file, true)?;
                write_file_to_server(&bitburner_request)?;
            }
        },
        Rename(source, destination) => {
            info!("file {:#?} has been moved to {:#?}", &source, &destination);
            if is_valid_file(&destination) {
                let bitburner_request = build_bitburner_request(destination, true)?;
                write_file_to_server(&bitburner_request)?;
            }
            if is_valid_file(&source) {
                let bitburner_request = build_bitburner_request(source, false)?;
                delete_file_from_server(&bitburner_request)?;
            }
        },
        Remove(file) => {
            if is_valid_file(&file) {
                info!("file deleted: {:#?}", &file);
                let bitburner_request = build_bitburner_request(file, false)?;
                delete_file_from_server(&bitburner_request)?;
            }
        },
        unhandled_event => debug!("Unhandled event: {:#?}", unhandled_event)
    }
    Ok(())
}

#[allow(unused_variables)]
fn build_bitburner_request(path_buf: &PathBuf, include_code: bool) -> Result<BitburnerRequest> {
    #[cfg(test)]
    let include_code = false;
    let filename: String = extract_file_name(path_buf)?;
    let code: Option<String> = match include_code {
        true => {
            Some(
                base64::encode(
                    fs::read_to_string(
                        path_buf.as_path()
                    )
                    .expect("Unable to extract file contents")
                )
            )
        },
        false => None,
    };
    Ok(
        BitburnerRequest {
            filename,
            code,
        }
    )
}

fn extract_file_name(path_buf: &PathBuf) -> Result<String> {
    path_buf.strip_prefix(&CONFIG.directory)
        .map(|path| path.to_str())?
        .map(|s| Ok(s.to_string()))
        .context("Unable to extract file name")?
}

fn is_valid_file(path_buf: &PathBuf) -> bool {
    path_buf.extension()
        .map(|ex| ex.to_str().unwrap_or("").to_string())
        .map(|s| CONFIG.valid_extensions.contains(&s))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use mockito::mock;
    use notify::DebouncedEvent;
    use super::*;

    #[test]
    fn assert_path_prefix_is_stripped() {
        assert_eq!(
            extract_file_name(&PathBuf::from("/one/two/three.txt")).unwrap(),
            String::from("three.txt")
        )
    }
    
    #[test]
    fn assert_valid_file() {
        assert_eq!(is_valid_file(&PathBuf::from("test.js")), true);
    }
    
    #[test]
    fn assert_invalid_file() {
        assert_eq!(is_valid_file(&PathBuf::from("test.kt")), false);
    }

    #[test]
    fn assert_write_event_is_successful() {
        let _m1 = mock("PUT", "/")
            .with_status(200)
            .with_body("written")
            .create();
        let event = DebouncedEvent::Write(PathBuf::from("/one/two/test.js"));
        assert!(handle_event(&event).is_ok());
    }

    #[test]
    fn assert_rename_event_is_successful() {
        let _m2 = mock("PUT", "/")
            .with_status(200)
            .with_body("written")
            .create();
        let _m3 = mock("DELETE", "/")
            .with_status(200)
            .with_body("deleted")
            .create();
        let event = DebouncedEvent::Rename(PathBuf::from("/one/two/source.js"), PathBuf::from("/one/two/destination.js"));
        assert!(handle_event(&event).is_ok());
    }

    #[test]
    fn assert_remove_event_is_successful() {
        let _m4 = mock("DELETE", "/")
            .with_status(200)
            .with_body("deleted")
            .create();
        let event = DebouncedEvent::Remove(PathBuf::from("/one/two/test.js"));
        assert!(handle_event(&event).is_ok());
    }
}
