use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
#[non_exhaustive]
pub enum Error {
    #[error("Invalid value: {0}")]
    InvalidValue(&'static str),
}
