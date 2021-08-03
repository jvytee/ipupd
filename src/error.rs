use std::{convert::From, fmt};

#[derive(Debug)]
pub struct Error {
    msg: String,
    error: Box<dyn std::error::Error>
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}: {}", self.msg, self.error)
    }
}

impl std::error::Error for Error {}

impl From<getopts::Fail> for Error {
    fn from(error: getopts::Fail) -> Self {
        Self {
            msg: "Could not parse arguments".to_string(),
            error: Box::new(error)
        }
    }
}

