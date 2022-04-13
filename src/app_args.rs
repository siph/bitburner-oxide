use clap::Parser;

/// Bitburner-oxide will automatically push modified or created script files to a running Bitburner game server.   
/// 
/// If ran from the same directory as the scripts the --directory flag is not needed.   
/// All managed files must exist in the top level directory; bitburner-oxide does not manage nested folders.   
/// 
/// Authentication is done by passing in the bearer-token via --token.   
/// Alternatively, the bearer-token can be placed in a file named 'token' in the chosen directory.   
/// 
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


