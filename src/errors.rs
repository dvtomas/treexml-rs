extern crate std;
extern crate xml;

use std::fmt;

#[derive(Debug)]
pub enum Error {
    ElementNotFound { t: String },
    ValueFromStr { t: String },
    ParseError { what: String },
    WriteError { what: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Error::ElementNotFound { t } => write!(f, "Element not found: '{}'", t),
            Error::ValueFromStr { t } => write!(f, "Value could not be parsed: '{}'", t),
            Error::ParseError { what } => write!(f, "Parse error: '{}'", what),
            Error::WriteError { what } => write!(f, "Write error: '{}'", what),
        }
    }
}

impl From<xml::reader::Error> for Error {
    fn from(v: xml::reader::Error) -> Self {
        Error::ParseError {
            what: std::error::Error::description(&v).into(),
        }
    }
}

impl From<xml::writer::Error> for Error {
    fn from(v: xml::writer::Error) -> Self {
        Error::WriteError {
            what: std::error::Error::description(&v).into(),
        }
    }
}
