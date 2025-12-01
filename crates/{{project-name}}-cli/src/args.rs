pub use clap::Parser;

#[derive(Parser, Debug)]
pub struct ArgParser {
    /// Logger verbosity
    #[clap(short, long)]
    pub verbosity: Option<u8>,
}
