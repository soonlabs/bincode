use core::error::Error as StdError;
use core2::io;
use core::str::Utf8Error;
use core::fmt;
use alloc::{boxed::Box, string::{String, ToString}};

/// The result of a serialization or deserialization operation.
pub type Result<T> = core::result::Result<T, Error>;

/// An error that can be produced during (de)serializing.
pub type Error = Box<ErrorKind>;

/// The kind of error that can be produced during a serialization or deserialization.
#[derive(Debug)]
pub enum ErrorKind {
    /// If the error stems from the reader/writer that is being used
    /// during (de)serialization, that error will be stored and returned here.
    Io(io::Error),
    /// Returned if the deserializer attempts to deserialize a string that is not valid utf8
    InvalidUtf8Encoding(Utf8Error),
    /// Returned if the deserializer attempts to deserialize a bool that was
    /// not encoded as either a 1 or a 0
    InvalidBoolEncoding(u8),
    /// Returned if the deserializer attempts to deserialize a char that is not in the correct format.
    InvalidCharEncoding,
    /// Returned if the deserializer attempts to deserialize the tag of an enum that is
    /// not in the expected ranges
    InvalidTagEncoding(usize),
    /// Serde has a deserialize_any method that lets the format hint to the
    /// object which route to take in deserializing.
    DeserializeAnyNotSupported,
    /// If (de)serializing a message takes more than the provided size limit, this
    /// error is returned.
    SizeLimit,
    /// Bincode can not encode sequences of unknown length (like iterators).
    SequenceMustHaveLength,
    /// A custom error message from Serde.
    Custom(String),
}

impl StdError for ErrorKind {
    fn description(&self) -> &str {
        match *self {
            ErrorKind::Io(_) => "io error",
            ErrorKind::InvalidUtf8Encoding(_) => "string is not valid utf8",
            ErrorKind::InvalidBoolEncoding(_) => "invalid u8 while decoding bool",
            ErrorKind::InvalidCharEncoding => "char is not valid",
            ErrorKind::InvalidTagEncoding(_) => "tag for enum is not valid",
            ErrorKind::SequenceMustHaveLength => {
                "Bincode can only encode sequences and maps that have a knowable size ahead of time"
            }
            ErrorKind::DeserializeAnyNotSupported => {
                "Bincode doesn't support serde::Deserializer::deserialize_any"
            }
            ErrorKind::SizeLimit => "the size limit has been reached",
            ErrorKind::Custom(ref msg) => msg,
        }
    }

    fn cause(&self) -> Option<&dyn core::error::Error> {
        match *self {
            ErrorKind::Io(_) => None,
            ErrorKind::InvalidUtf8Encoding(_) => None,
            ErrorKind::InvalidBoolEncoding(_) => None,
            ErrorKind::InvalidCharEncoding => None,
            ErrorKind::InvalidTagEncoding(_) => None,
            ErrorKind::SequenceMustHaveLength => None,
            ErrorKind::DeserializeAnyNotSupported => None,
            ErrorKind::SizeLimit => None,
            ErrorKind::Custom(_) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        ErrorKind::Io(err).into()
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::Io(ref ioerr) => write!(fmt, "io error: {}", ioerr),
            ErrorKind::InvalidUtf8Encoding(ref e) => {
                write!(fmt, "string is not valid utf8: {}", e)
            }
            ErrorKind::InvalidBoolEncoding(b) => {
                write!(fmt, "invalid u8 while decoding bool, expected 0 or 1, found {}", b)
            }
            ErrorKind::InvalidCharEncoding => write!(fmt, "char is not valid"),
            ErrorKind::InvalidTagEncoding(tag) => {
                write!(fmt, "tag for enum is not valid: {}", tag)
            }
            ErrorKind::SequenceMustHaveLength => write!(fmt, "sequence must have length"),
            ErrorKind::SizeLimit => write!(fmt, "the size limit has been reached"),
            ErrorKind::DeserializeAnyNotSupported => write!(
                fmt,
                "Bincode does not support the serde::Deserializer::deserialize_any method"
            ),
            ErrorKind::Custom(ref s) => s.fmt(fmt),
        }
    }
}

impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(desc: T) -> Error {
        ErrorKind::Custom(desc.to_string()).into()
    }
}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        ErrorKind::Custom(msg.to_string()).into()
    }
}
