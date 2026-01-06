mod app;
mod logger;
mod macros;
mod utils;

use anyhow::{Context, Result};
use app::run;
use logger::setup_logger;

fn main() -> Result<()> {
    let args = app_cli::parse(); // replace with placeholder
    setup_logger(args.verbosity, args.log_to_file).context("Failed to initialize logger.")?;
    run().context("Failed to run application.")
}
