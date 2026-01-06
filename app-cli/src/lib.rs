pub mod args;
use clap::Parser;

use args::CmdArgs;

pub fn parse() -> CmdArgs {
    CmdArgs::parse()
}
