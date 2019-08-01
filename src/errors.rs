use libcommon_rs::errors::Error as BaseError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Base(BaseError),
    AtMaxVecCapacity,
}

impl From<BaseError> for Error {
    #[inline]
    fn from(be: BaseError) -> Error {
        Error::Base(be)
    }
}
