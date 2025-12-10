mod app;
mod persistence;
mod platform;

use {{project-name}}_cli as cli;
use {{project-name}}_core::error::{Result, other_error};
use {{project-name}}_core::logger;

fn main() -> Result<()> {
    let args = cli::parse();
    logger::setup_logger(args.verbosity, None)?;
    app::run().map_err(|err| other_error(err.to_string(), "app::run".to_owned()))
}
