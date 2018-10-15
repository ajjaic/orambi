use std::{
    fmt::{
        Display,
        Formatter,
        Error as FmtError,
    },
    error::Error as TError,
    io::Error as IOError,
    convert::From,
};
use exif::Error as ExifError;

#[derive(Debug)]
pub enum Error {
    FileNotFound,
    FileNotSupported,
    CreationDateUnavailable,
    InvalidPath,
    ModError(Box<dyn TError>),
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Error::FileNotFound, Error::FileNotFound) => true,
            (Error::FileNotSupported, Error::FileNotSupported) => true,
            (Error::CreationDateUnavailable, Error::CreationDateUnavailable) => true,
            (Error::InvalidPath, Error::InvalidPath) => true,
            (Error::ModError(_), Error::ModError(_)) => true,
            _ => false,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Error::FileNotFound => write!(f, "{}", "File not found"),
            Error::FileNotSupported => write!(f, "{}", "File not supported"),
            Error::CreationDateUnavailable => write!(f, "{}", "Image metadata does not contain creation date"),
            Error::InvalidPath => write!(f, "{}", "Either the filename or the parent path is invalid"),
            Error::ModError(err) => write!(f, "{}", err),
        }
    }
}

impl TError for Error {
    fn cause(&self) -> Option<&dyn TError> {
        match self {
            Error::ModError(err) => Some(err.as_ref()),
            _ => Some(self),
        }
    }
}

impl From<IOError> for Error {
    fn from(err: IOError) -> Self {
        Error::ModError(Box::new(err))
    }
}

impl From<ExifError> for Error {
    fn from(err: ExifError)  -> Self {
        Error::ModError(Box::new(err))
    }
}