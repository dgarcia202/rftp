use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct RftpError(pub String);

impl Error for RftpError {
}

impl fmt::Display for RftpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}