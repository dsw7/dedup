use std::fmt;

#[derive(Debug, Clone)]
pub struct DeduplicationError(pub String);

impl From<String> for DeduplicationError {
    fn from(message: String) -> Self {
        DeduplicationError(message)
    }
}

impl fmt::Display for DeduplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
