use std::fmt;

use xml;

#[derive(Debug)]
pub enum Error {
    ElementNotFound { t: String },
    ValueFromStr { t: String },
    ParseError { what: xml::reader::Error },
    WriteError { what: xml::writer::Error },
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
        Error::ParseError { what: v.into() }
    }
}

impl From<xml::writer::Error> for Error {
    fn from(v: xml::writer::Error) -> Self {
        Error::WriteError { what: v.into() }
    }
}
