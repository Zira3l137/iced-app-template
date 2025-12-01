pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    TracingSubscriber(#[from] tracing_subscriber::filter::ParseError),
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error>),
}

unsafe impl std::marker::Send for Error {}

#[derive(Debug)]
struct StringError(String);

impl std::fmt::Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for StringError {}

impl Error {
    /// Create an Other error from a string message
    pub fn other<S: Into<String>>(msg: S, source: S) -> Self {
        let err_msg = format!("{} error: {}", source.into(), msg.into());
        Error::Other(Box::new(StringError(err_msg)))
    }
}
