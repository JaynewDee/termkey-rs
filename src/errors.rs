use std::fmt;

#[derive(Debug)]
pub struct WriteError {
    message: String,
    _error: anyhow::Error,
}

impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "!Write Error! ::: {}", self.message)
    }
}

impl<T: std::error::Error + Send + Sync + 'static> From<T> for WriteError {
    fn from(e: T) -> Self {
        Self {
            message: e.to_string(),
            _error: anyhow::Error::new(e),
        }
    }
}

#[derive(Debug)]
pub struct ControlError {
    message: String,
    _error: anyhow::Error,
}

impl fmt::Display for ControlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "!Control Error! ::: {}", self.message)
    }
}

impl<T: std::error::Error + Send + Sync + 'static> From<T> for ControlError {
    fn from(e: T) -> Self {
        Self {
            message: e.to_string(),
            _error: anyhow::Error::new(e),
        }
    }
}
