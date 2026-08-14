use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RefParseError {
    InvalidShape,
    InvalidKind,
    InvalidId,
}

impl Display for RefParseError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "invalid Ref")
    }
}

impl Error for RefParseError {}
