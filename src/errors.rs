/// # Fantom Libtransport/errors
///
/// This file simply defines a set of errors and error handling functionality for the Transport
/// trait. THis simply allows us to convert any std::Error to a variant as described in libcommon.rs.
use libcommon_rs::errors::Error as BaseError;
use std::error::Error as StdError;
use std::net::AddrParseError;
use std::sync::{MutexGuard, PoisonError};

/// Standard Error type as defiend by the std library.
pub type Result<T> = std::result::Result<T, Error>;

/// A set of enums to differentiate between different types of errors.
#[derive(Debug)]
pub enum Error {
    Base(BaseError),
    /// Indicating a vector reached max capacity and can not receive new element
    AtMaxVecCapacity,
    Bincode(bincode::Error),
    Io(std::io::Error),
    /// Indicating read/write operation was unable to read/write complete size of data
    Incomplete,
    PoisonError(String),
    /// Socket address parse error.
    AddrParse(AddrParseError),
}
/// Allow errors to be converted from a standard error to a BaseError type.
impl From<BaseError> for Error {
    #[inline]
    fn from(be: BaseError) -> Error {
        Error::Base(be)
    }
}

/// Allow errors to be converted from a standard error to a bincode type.
impl From<bincode::Error> for Error {
    #[inline]
    fn from(bincode_error: bincode::Error) -> Error {
        Error::Bincode(bincode_error)
    }
}

/// Allow errors to be converted from a standard error to a io_error type.
impl From<std::io::Error> for Error {
    #[inline]
    fn from(io_error: std::io::Error) -> Error {
        Error::Io(io_error)
    }
}

/// Allow errors to be converted from a standard error to a PoisonError.
impl<'a, T> From<PoisonError<MutexGuard<'a, T>>> for Error {
    fn from(e: PoisonError<MutexGuard<'a, T>>) -> Error {
        Error::PoisonError(e.description().to_string())
    }
}

/// Macro for when there is no error: equivalent of a 'None' for errors.
#[macro_export]
macro_rules! none_error {
    () => {
        libtransport::errors::Error::Base(libcommon_rs::errors::Error::NoneError)
    };
}
