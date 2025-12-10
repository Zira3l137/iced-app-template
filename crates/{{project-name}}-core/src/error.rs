pub type Result<T> = anyhow::Result<T>;

// Helper function to create errors with context
pub fn other_error<S: Into<String>>(msg: S, source: S) -> anyhow::Error {
    anyhow::anyhow!("{} error: {}", source.into(), msg.into())
}
