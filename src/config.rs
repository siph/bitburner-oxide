use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};

/// Bitburner-oxide will watch for the creation, modification, or deletion of files within the chosen directory and its
/// child directories. Upon detection of these events, Bitburner-oxide will update the Bitburner game files to reflect
/// the changes made to the files and directories within the chosen directory.
///
/// Source for bitburner-oxide can be found at https://www.github.com/siph/bitburner-oxide
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Set to true to synchronize deletions
    pub allow_deleting_files: bool,
    /// Bitburner websocket port
    pub port: u16,
    /// Path to target scripts
    pub scripts_folder: PathBuf,
    /// Log output
    pub quiet: bool,
    /// Set true to simulate actions
    pub dry: bool,
    /// Synchronize on start
    pub push_all_on_connection: bool,
    /// Filetypes to synchronize
    pub allowed_filetypes: Vec<String>,
    pub definitions_file: DefinitionsFile,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            allowed_filetypes: vec![
                "script".to_string(),
                "js".to_string(),
                "ns".to_string(),
                "txt".to_string(),
            ],
            allow_deleting_files: false,
            port: 12525,
            scripts_folder: PathBuf::from_str(".").unwrap(),
            quiet: false,
            dry: false,
            definitions_file: DefinitionsFile::default(),
            push_all_on_connection: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefinitionsFile {
    /// Update definitions file on start
    pub update: bool,
    /// Path to definitions file
    pub location: PathBuf,
}

impl Default for DefinitionsFile {
    fn default() -> DefinitionsFile {
        DefinitionsFile {
            update: true,
            location: PathBuf::from_str("NetScriptDefinitions.d.ts").unwrap(),
        }
    }
}
