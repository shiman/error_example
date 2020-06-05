use std::result;
use std::fmt;
use std::str;
use std::io;

pub type Result<T> = result::Result<T, Error>;

/// Define possible error types.
/// Some are just wrappers over other Errors, like `Io` and `UTF8Error`.
/// Some are completely new error types, like `ValueError` and `FormatError`.
/// Deriving `Debug` is necessary since `Error`, the wrapper of `ErrorKind`, must also
/// implement the trait `std::fmt::Debug`.
#[derive(Debug)]
pub enum ErrorKind {
    /// An I/O error that occurred while reading the input file.
    Io(io::Error),
    /// A value error when some key information is missing.
    ValueError(String),
    /// A format error when the input file's format is invalid.
    FormatError,
    /// A UTF-8 error when the input data is not a valid utf-8 file.
    UTF8Error(str::Utf8Error),
    /// Unknown error.
    Other(String),
}

/// This is the only Error type that our lib will expose to others.
pub struct Error(Box<ErrorKind>);

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error(Box::new(kind))
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }

    pub fn into_kind(self) -> ErrorKind {
        *self.0
    }
}

/// How should we easily create new Error instances?
/// For completely new error types, it is better to write a helper function
/// here to create the instance.
pub fn new_value_error(value: &str) -> Error {
    Error::new(
        ErrorKind::ValueError(format!("failed to get {}", value))
    )
}
pub fn new_format_error() -> Error {
    Error::new(ErrorKind::FormatError)
}
pub fn new_other_error(value: &str) -> Error {
    Error::new(
        ErrorKind::ValueError(String::from(value))
    )
}

/// For wrapper error types, we simply need to
/// implement From<MyError> so that `?` operators can automatically
/// convert our customized error types into the unified Error type.
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::new(ErrorKind::Io(err))
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Error {
        Error::new(ErrorKind::UTF8Error(err))
    }
}

/// Implement methods to help Rust trace back.
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self.0 {
            ErrorKind::Io(ref err) => Some(err),
            ErrorKind::ValueError(_) => None,
            ErrorKind::FormatError => None,
            ErrorKind::UTF8Error(ref err) => Some(err),
            ErrorKind::Other(_) => None,
        }
    }
}

/// Implement Display so Rust can print the error message elegantly.
/// Required by `impl std::error::Error`.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ErrorKind::Io(ref err) => err.fmt(f),
            ErrorKind::ValueError(ref err) => {
                write!(f, "data missing: {}", err)
            },
            ErrorKind::FormatError => write!(f, "Wrong format!"),
            ErrorKind::UTF8Error(ref err) => err.fmt(f),
            ErrorKind::Other(ref err) => {
                write!(f, "Unknown error: {}", err)
            }
        }
    }
}

/// Implement Debug so Rust can print the error message on `?` and `unwrap()`.
/// Required by `impl std::error::Error`.
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}