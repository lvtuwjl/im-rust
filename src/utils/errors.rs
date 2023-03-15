use super::server_error::ServerError;
use std::{
    cmp::PartialEq,
    error::Error as StdError,
    fmt::{self, Display},
    io,
};

pub type Result<T> = std::result::Result<T, anyhow::Error>;

#[derive(Debug)]
pub enum Error {
    /// The underlying collection no longer exists.
    CollectionNotFound(i16),
    /// The system has been used in an unsupported way.
    Unsupported(String),
    /// An unexpected bug has happened. Please open an issue on github!
    ReportableBug(String),
    /// A read or write error has happened when interacting with the file
    /// system.
    Io(io::Error),
    Custom(ServerError),
}

impl StdError for Error {}

impl Clone for Error {
    fn clone(&self) -> Self {
        use self::Error::*;

        match self {
            Io(ioe) => Io(io::Error::new(ioe.kind(), format!("{:?}", ioe))),
            CollectionNotFound(name) => CollectionNotFound(name.clone()),
            Unsupported(why) => Unsupported(why.clone()),
            ReportableBug(what) => ReportableBug(what.clone()),
            Custom(ra) => Custom(ra.clone()),
        }
    }
}

// impl From<io::Error> for Error {
//     #[inline]
//     fn from(io_error: io::Error) -> Self {
//         Error::Io(io_error)
//     }
// }
// impl From<i16> for Error {
//     #[inline]
//     fn from(io_error: i16) -> Self {
//         // Error::CollectionNotFound(io_error)
//         // io_error
//         Error::CollectionNotFound(io_error)
//     }
// }

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        use self::Error::*;

        match *self {
            CollectionNotFound(ref name) => {
                write!(f, "Collection {:?} does not exist", name,)
            }
            Unsupported(ref e) => write!(f, "Unsupported: {}", e),
            ReportableBug(ref e) => write!(
                f,
                "Unexpected bug has happened: {}. \
                 PLEASE REPORT THIS BUG!",
                e
            ),
            Io(ref e) => write!(f, "IO error: {}", e),
            Custom(ref ra) => write!(f, "Custom: {}", ra),
        }
    }
}
