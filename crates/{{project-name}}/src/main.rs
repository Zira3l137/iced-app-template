mod app;
mod persistence;
mod platform;

use {{crate_name}}_cli as cli;
use {{crate_name}}_core::logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::parse();
    logger::setup_logger(args.verbosity, None)?;
    app::run()
}
