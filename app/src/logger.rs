use std::env;
use std::{fs, str::FromStr};

use anyhow::{Context, Result};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    EnvFilter, Layer,
    fmt::{layer, time},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

pub fn setup_logger(verbosity: Option<LevelFilter>, log_to_file: bool) -> Result<()> {
    let workspace_name = env!("CARGO_PKG_NAME");
    let level_filter = match verbosity {
        Some(level) => level,
        None => LevelFilter::from_str(&env::var("RUST_LOG").unwrap_or(String::from("info")))
            .unwrap_or(LevelFilter::INFO),
    };

    let console_layer_env_filter = EnvFilter::new("")
        .add_directive(
            format!("{}={level_filter}", workspace_name)
                .parse()
                .context("Failed to parse log directive")?,
        )
        .add_directive("error".parse().context("Failed to parse log directive")?);

    let console_layer = layer()
        .with_target(true)
        .with_file(true)
        .with_level(true)
        .with_line_number(true)
        .with_ansi(true)
        .without_time()
        .with_filter(console_layer_env_filter);

    let layered_registry = tracing_subscriber::registry().with(console_layer);

    if log_to_file {
        let log_file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{workspace_name}.log"))
            .context("Failed to open log file")?;

        let file_layer_filter = EnvFilter::new("")
            .add_directive(
                format!("{workspace_name}={level_filter}")
                    .parse()
                    .context("Failed to parse log directive")?,
            )
            .add_directive("error".parse().context("Failed to parse log directive")?);

        let file_layer = layer()
            .with_writer(log_file)
            .with_target(true)
            .with_file(true)
            .with_level(true)
            .with_line_number(true)
            .with_ansi(false)
            .with_timer(time())
            .with_filter(file_layer_filter);

        layered_registry.with(file_layer).init();
    } else {
        layered_registry.init();
    }

    tracing::debug!("Logger initialized with level: {level_filter}");
    Ok(())
}
