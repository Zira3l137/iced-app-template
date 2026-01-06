/// CLI interface module. All available command line arguments and options are defined here.
pub use clap::Parser;
use tracing::level_filters::LevelFilter;

/// CLI interface model based on `clap` crate.
/// Expand it to include more options, switches or arguments.
#[derive(Parser, Debug)]
pub struct CmdArgs {
    /// Logger verbosity
    #[clap(short, long)]
    pub verbosity: Option<LevelFilter>,
    #[clap(long)]
    pub log_to_file: bool,
}
