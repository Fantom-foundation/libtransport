use libcommon_rs::errors::Error as BaseError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Base(BaseError),
    AtMaxVecCapacity,
    Bincode(bincode::Error),
    Io(std::io::Error),
}

impl From<BaseError> for Error {
    #[inline]
    fn from(be: BaseError) -> Error {
        Error::Base(be)
    }
}

impl From<bincode::Error> for Error {
    #[inline]
    fn from(bincode_error: bincode::Error) -> Error {
        Error::Bincode(bincode_error)
    }
}

impl From<std::io::Error> for Error {
    #[inline]
    fn from(io_error: std::io::Error) -> Error {
        Error::Io(io_error)
    }
}

#[macro_export]
macro_rules! none_error {
    () => {
        libtransport::errors::Error::Base(fantom_common_rs::errors::Error::NoneError)
    };
}
