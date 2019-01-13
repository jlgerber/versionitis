use failure::Fail;
use std::fmt::Debug;

#[derive(Fail, Debug)]
pub enum VersionitisError {
    #[fail(display = "{}", _0)]
    ParseIntError(#[fail(cause)] std::num::ParseIntError),
    #[fail(display="InvalidPackageVersion: {}", _0)]
    InvalidPackageVersion(String),
    #[fail(display="UnknownPackage: {}", _0)]
    UnknownPackage(String),
}

use std::num::ParseIntError;
impl From<ParseIntError> for VersionitisError {
    fn from(err: ParseIntError) -> Self {
        VersionitisError::ParseIntError(err)
    }
}