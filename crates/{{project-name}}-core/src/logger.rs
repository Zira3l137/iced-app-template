use std::path::Path;

use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::error::Result;

fn level_filter_from_u8(value: u8) -> tracing::level_filters::LevelFilter {
    match value {
        0 => LevelFilter::OFF,
        1 => LevelFilter::ERROR,
        2 => LevelFilter::WARN,
        3 => LevelFilter::INFO,
        4 => LevelFilter::DEBUG,
        5 => LevelFilter::TRACE,
        _ => LevelFilter::OFF,
    }
}

pub fn setup_logger(verbosity: Option<u8>, file_path: Option<&Path>) -> Result<()> {
    let level_filter = match verbosity {
        Some(value) => level_filter_from_u8(value),
        None => LevelFilter::ERROR,
    };

    let package_name = env!("CARGO_PKG_NAME").replace('-', "_");

    let console_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_file(true)
        .with_level(true)
        .with_line_number(true)
        .with_ansi(true)
        .without_time()
        .with_filter(
            EnvFilter::new("")
                .add_directive(format!("{}={level_filter}", &package_name).parse()?)
                .add_directive("error".parse()?),
        );

    let layered_registry = tracing_subscriber::registry().with(console_layer);

    if let Some(file_path) = file_path {
        let log_file = std::fs::OpenOptions::new().create(true).append(true).open(file_path)?;
        let file_layer = tracing_subscriber::fmt::layer()
            .with_writer(log_file)
            .with_target(true)
            .with_file(true)
            .with_level(true)
            .with_line_number(true)
            .with_ansi(false)
            .with_timer(tracing_subscriber::fmt::time::time())
            .with_filter(
                EnvFilter::new("")
                    .add_directive(format!("{package_name}={level_filter}").parse()?)
                    .add_directive("error".parse()?),
            );

        layered_registry.with(file_layer).init();
    } else {
        layered_registry.init();
    }

    tracing::debug!("Logger initialized with level: {level_filter}");
    Ok(())
}
