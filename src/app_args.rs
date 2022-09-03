use clap::Parser;

/// Bitburner-oxide will watch for the creation, modification, or deletion of files within the chosen directory and its
/// child directories. Upon detection of these events, Bitburner-oxide will update the Bitburner game files to reflect
/// the changes made to the files and directories within the chosen directory.   
///
/// Authentication is done by passing in the bearer-token via --token. 
/// Alternatively, the bearer-token can be placed in a file named 'token' in the chosen directory.
/// When ran from the same directory as the scripts you wish to manage, the --directory flag is not needed.   
/// Source for bitburner-oxide can be found at https://www.gitlab.com/xsiph/bitburner-oxide
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct AppArgs {
    /// auth token from game menu bar
    #[clap(short, long)]
    pub bearer_token: Option<String>,
    /// base directory to synchronize files
    #[clap(short, long)]
    pub directory: Option<String>,
}


