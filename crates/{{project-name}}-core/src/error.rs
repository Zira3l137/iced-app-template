//! Error helpers built on top of `anyhow`.
//!
//! The template standardizes on `anyhow::Result` for fallible operations so
//! callers can attach context while keeping error handling lightweight.

/// Convenience alias for fallible operations across the workspace.
pub type Result<T> = anyhow::Result<T>;

/// Build an `anyhow::Error` with a human-friendly source + message.
pub fn other_error<S: Into<String>>(msg: S, source: S) -> anyhow::Error {
    anyhow::anyhow!("{} error: {}", source.into(), msg.into())
}
