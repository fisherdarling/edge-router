use std::fmt;

#[derive(Debug)]
pub struct Error {
    inner: BoxError,
}

impl Error {
    pub fn new(inner: impl Into<BoxError>) -> Self {
        Self {
            inner: inner.into(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl std::error::Error for Error {}

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
