use crate::bitburner::operation::{Action, BitburnerOperation};
use jsonrpc_ws_server::ws::Message;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};

pub static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Json message to be broadcast to bitburner websocket client.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BitburnerMessage {
    jsonrpc: JsonrpcVersion,
    id: usize,
    method: BitburnerMethod,
    params: BitburnerMessageParams,
}

impl From<BitburnerOperation> for Vec<BitburnerMessage> {
    fn from(operation: BitburnerOperation) -> Self {
        match operation.action {
            Action::MOVE => vec![
                // source
                BitburnerMessage {
                    id: COUNTER.fetch_add(1, Ordering::Relaxed),
                    method: BitburnerMethod::DeleteFile,
                    params: BitburnerMessageParams {
                        filename: Some(String::from(operation.files[0].filename.to_str().unwrap())),
                        content: None,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                // destination
                BitburnerMessage {
                    id: COUNTER.fetch_add(1, Ordering::Relaxed),
                    method: BitburnerMethod::PushFile,
                    params: BitburnerMessageParams {
                        filename: Some(String::from(operation.files[1].filename.to_str().unwrap())),
                        content: operation.files[1].code.clone(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            Action::REMOVE => vec![BitburnerMessage {
                id: COUNTER.fetch_add(1, Ordering::Relaxed),
                method: BitburnerMethod::DeleteFile,
                params: BitburnerMessageParams {
                    filename: Some(String::from(operation.files[0].filename.to_str().unwrap())),
                    content: None,
                    ..Default::default()
                },
                ..Default::default()
            }],
            Action::CREATE => vec![BitburnerMessage {
                id: COUNTER.fetch_add(1, Ordering::Relaxed),
                method: BitburnerMethod::PushFile,
                params: BitburnerMessageParams {
                    filename: Some(String::from(operation.files[0].filename.to_str().unwrap())),
                    content: operation.files[0].code.clone(),
                    ..Default::default()
                },
                ..Default::default()
            }],
            Action::IGNORE => vec![],
        }
    }
}

impl Default for BitburnerMessage {
    fn default() -> BitburnerMessage {
        BitburnerMessage {
            jsonrpc: JsonrpcVersion::Two,
            id: 0, // TODO: reference global atomic count here?
            method: BitburnerMethod::GetDefinitionFile,
            params: BitburnerMessageParams::default(),
        }
    }
}

impl From<BitburnerMessage> for Message {
    fn from(bitburner_message: BitburnerMessage) -> Self {
        Message::Text(serde_json::to_string(&bitburner_message).unwrap())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BitburnerMessageParams {
    filename: Option<String>,
    content: Option<String>,
    server: Option<String>,
}

impl Default for BitburnerMessageParams {
    fn default() -> BitburnerMessageParams {
        BitburnerMessageParams {
            filename: None,
            content: None,
            server: Some(String::from("home")),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BitburnerMethod {
    PushFile,
    GetFile,
    DeleteFile,
    GetFileNames,
    GetAllFiles,
    CalculateRam,
    GetDefinitionFile,
}

impl BitburnerMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            BitburnerMethod::PushFile => "pushFile",
            BitburnerMethod::GetFile => "getFile",
            BitburnerMethod::DeleteFile => "deleteFile",
            BitburnerMethod::GetFileNames => "getFileNames",
            BitburnerMethod::GetAllFiles => "getAllFiles",
            BitburnerMethod::CalculateRam => "calculateRam",
            BitburnerMethod::GetDefinitionFile => "getDefinitionsFile",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum JsonrpcVersion {
    Two,
}

impl JsonrpcVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            JsonrpcVersion::Two => "2.0",
        }
    }
}

// Because of the atomic counter, these tests need to be ran in parallel (`cargo test -- --test-threads=-`)
// This isn't ideal but I don't know any simple work-around.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::bitburner::operation::{ extract_file_name, extract_file_contents, File, BitburnerOperation};
    use std::path::PathBuf;

    #[test]
    fn create_operation_transforms_into_messages() {
        let target_file = PathBuf::from("./Cargo.toml");
        let source_filename = PathBuf::from(extract_file_name(&target_file));
        let operation = BitburnerOperation {
            action: Action::CREATE,
            files: vec![File {
                filename: PathBuf::from(extract_file_name(&target_file)),
                code: Some(String::from(extract_file_contents(&target_file))),
            }],
        };
        let messages = Vec::from(operation);
        assert_eq!(
            messages[0],
            BitburnerMessage {
                id: 0,
                method: BitburnerMethod::PushFile,
                params: BitburnerMessageParams {
                    filename: Some(source_filename.to_str().unwrap().to_owned()),
                    content: Some(String::from(extract_file_contents(&target_file))),
                    ..Default::default()
                },
                ..Default::default()
            }
        );
    }

    #[test]
    fn move_operation_transforms_into_messages() {
        let target_file = PathBuf::from("./Cargo.toml");
        let destination_file = PathBuf::from("./Cargo.lock");
        let source_filename = PathBuf::from(extract_file_name(&target_file));
        let destination_filename = PathBuf::from(extract_file_name(&destination_file));
        let operation = BitburnerOperation {
            action: Action::MOVE,
            files: vec![
                File {
                    filename: PathBuf::from(extract_file_name(&target_file)),
                    code: None,
                },
                File {
                    filename: PathBuf::from(extract_file_name(&destination_file)),
                    code: Some(String::from(extract_file_contents(&destination_file))),
                },
            ],
        };
        let messages = Vec::from(operation);
        assert_eq!(
            messages[0],
            BitburnerMessage {
                id: 1,
                method: BitburnerMethod::DeleteFile,
                params: BitburnerMessageParams {
                    filename: Some(source_filename.to_str().unwrap().to_owned()),
                    content: None,
                    ..Default::default()
                },
                ..Default::default()
            }
        );
        assert_eq!(
            messages[1],
            BitburnerMessage {
                id: 2,
                method: BitburnerMethod::PushFile,
                params: BitburnerMessageParams {
                    filename: Some(destination_filename.to_str().unwrap().to_owned()),
                    content: Some(String::from(extract_file_contents(&destination_file))),
                    ..Default::default()
                },
                ..Default::default()
            }
        );
    }

    #[test]
    fn remove_operation_transforms_into_messages() {
        let target_file = PathBuf::from("./Cargo.toml");
        let source_filename = PathBuf::from(extract_file_name(&target_file));
        let operation = BitburnerOperation {
            action: Action::REMOVE,
            files: vec![File {
                filename: PathBuf::from(extract_file_name(&target_file)),
                code: None,
            }],
        };
        let messages = Vec::from(operation);
        assert_eq!(
            messages[0],
            BitburnerMessage {
                id: 3,
                method: BitburnerMethod::DeleteFile,
                params: BitburnerMessageParams {
                    filename: Some(source_filename.to_str().unwrap().to_owned()),
                    content: None,
                    ..Default::default()
                },
                ..Default::default()
            }
        );
    }
}
