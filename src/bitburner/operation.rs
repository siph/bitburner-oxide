use crate::Config;
use anyhow::{Context, Error};
use notify::{event::EventKind, Event};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

/// Subject of [BitburnerOperation]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct File {
    /// File path with the scripts folder treated as root
    pub filename: PathBuf,
    /// File contents encoded into base64.
    pub code: Option<String>,
}

/// Action of [BitburnerOperation]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Action {
    REMOVE,
    CREATE,
    MOVE,
    IGNORE,
}

/// Collection of [File] and the [Action] to be preformed. Two separate files are needed for a move or rename operation.
/// In this instance, the source file will be at the first index and the destination file at the second index.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BitburnerOperation {
    pub action: Action,
    pub files: Vec<File>,
}

impl BitburnerOperation {
    /// Will attempt to build a [BitburnerOperation] from an [Event] and [Config]
    pub fn build_operation(config: &Config, event: Event) -> Result<BitburnerOperation, Error> {
        let target_file = &event.paths.get(0).unwrap();
        match &event.kind {
            EventKind::Create(_) => {
                info!("file created: {:#?}", &event);
                Ok(BitburnerOperation {
                    action: Action::CREATE,
                    files: vec![File {
                        filename: PathBuf::from(extract_file_name(config, target_file)),
                        code: Some(extract_file_contents(target_file)),
                    }],
                })
            }
            EventKind::Modify(_) => {
                let destination_file = &event.paths.get(1).unwrap();
                info!(
                    "file moved: {:#?} -> {:#?}",
                    &target_file, &destination_file
                );
                Ok(BitburnerOperation {
                    action: Action::MOVE,
                    files: vec![
                        File {
                            filename: PathBuf::from(extract_file_name(config, target_file)),
                            code: None,
                        },
                        File {
                            filename: PathBuf::from(extract_file_name(config, destination_file)),
                            code: Some(extract_file_contents(destination_file)),
                        },
                    ],
                })
            }
            EventKind::Remove(_) => {
                info!("file deleted: {:#?}", &event);
                Ok(BitburnerOperation {
                    action: Action::REMOVE,
                    files: vec![File {
                        filename: PathBuf::from(extract_file_name(config, target_file)),
                        code: None,
                    }],
                })
            }
            unhandled_event => {
                warn!("Unhandled event: {:#?}", unhandled_event);
                Ok(BitburnerOperation {
                    action: Action::IGNORE,
                    files: vec![],
                })
            }
        }
    }
}

pub fn extract_file_contents(path_buf: &PathBuf) -> String {
    base64::encode(fs::read_to_string(path_buf.as_path()).expect("Unable to extract file contents"))
}

pub fn extract_file_name(config: &Config, path_buf: &PathBuf) -> String {
    path_buf
        .strip_prefix(&config.scripts_folder)
        .map(|path| path.to_str())
        .unwrap()
        .map(|s| s.to_string())
        .context("Unable to extract file name")
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use notify::{event::CreateKind, Event};

    #[test]
    fn create_event_transforms_into_operation() {
        let config = Config::default();
        let target_file = PathBuf::from("./Cargo.toml");
        let create_event = Event {
            kind: EventKind::Create(CreateKind::Any),
            paths: vec![target_file.clone()],
            ..Default::default()
        };
        let operation = BitburnerOperation {
            action: Action::CREATE,
            files: vec![File {
                filename: PathBuf::from(extract_file_name(&config, &target_file)),
                code: Some(String::from(extract_file_contents(&target_file))),
            }],
        };
        assert_eq!(
            BitburnerOperation::build_operation(&config, create_event).unwrap(),
            operation
        );
    }

    #[test]
    fn remove_event_transforms_into_operation() {
        let config = Config::default();
        let target_file = PathBuf::from("./Cargo.toml");
        let remove_event = Event {
            kind: EventKind::Remove(notify::event::RemoveKind::Any),
            paths: vec![target_file.clone()],
            ..Default::default()
        };
        let operation = BitburnerOperation {
            action: Action::REMOVE,
            files: vec![File {
                filename: PathBuf::from(extract_file_name(&config, &target_file)),
                code: None,
            }],
        };
        assert_eq!(
            BitburnerOperation::build_operation(&config, remove_event).unwrap(),
            operation
        );
    }

    #[test]
    fn move_event_transforms_into_operation() {
        let config = Config::default();
        let target_file = PathBuf::from("./Cargo.toml");
        let destination_file = PathBuf::from("./Cargo.lock");
        let move_event = Event {
            kind: EventKind::Modify(notify::event::ModifyKind::Any),
            paths: vec![target_file.clone(), destination_file.clone()],
            ..Default::default()
        };
        let operation = BitburnerOperation {
            action: Action::MOVE,
            files: vec![
                File {
                    filename: PathBuf::from(extract_file_name(&config, &target_file)),
                    code: None,
                },
                File {
                    filename: PathBuf::from(extract_file_name(&config, &destination_file)),
                    code: Some(String::from(extract_file_contents(&destination_file))),
                },
            ],
        };
        assert_eq!(
            BitburnerOperation::build_operation(&config, move_event).unwrap(),
            operation
        );
    }

    #[test]
    fn unknown_event_transforms_into_operation() {
        let config = Config::default();
        let target_file = PathBuf::from("./Cargo.toml");
        let unknown_event = Event {
            kind: EventKind::Other,
            paths: vec![target_file.clone()],
            ..Default::default()
        };
        let operation = BitburnerOperation {
            action: Action::IGNORE,
            files: vec![],
        };
        assert_eq!(
            BitburnerOperation::build_operation(&config, unknown_event).unwrap(),
            operation
        );
    }
}
