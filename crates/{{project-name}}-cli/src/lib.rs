pub mod args;
use clap::Parser;

pub use args::ArgParser;

pub fn parse() -> ArgParser {
    args::ArgParser::parse()
}
