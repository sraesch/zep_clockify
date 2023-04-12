use hyper::{http::uri::InvalidUri, Error as HyperError};
use quick_error::quick_error;
use serde_json::Error as JSONError;
use std::io;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        IO(err: std::string::String) {
            display("{}", err)
        }
        Internal(err: std::string::String) {
            display("{}", err)
        }
        InvalidURI(err: std::string::String) {
            display("{}", err)
        }
        RestAPI(err: std::string::String) {
            display("{}", err)
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IO(format!("{}", error))
    }
}

impl From<InvalidUri> for Error {
    fn from(error: InvalidUri) -> Self {
        Error::IO(format!("{}", error))
    }
}

impl From<HyperError> for Error {
    fn from(error: HyperError) -> Self {
        Error::RestAPI(format!("{}", error))
    }
}

impl From<JSONError> for Error {
    fn from(error: JSONError) -> Self {
        Error::RestAPI(format!("{}", error))
    }
}
