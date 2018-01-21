extern crate std;
extern crate xml;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Element not found: '{}'", t)] ElementNotFound { t: String },
    #[fail(display = "Value could not be parsed: '{}'", t)] ValueFromStr { t: String },
    #[fail(display = "Parse error: '{}'", what)] ParseError { what: String },
    #[fail(display = "Write error: '{}'", what)] WriteError { what: String },
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
