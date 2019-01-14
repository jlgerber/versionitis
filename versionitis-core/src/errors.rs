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
    #[fail(display="{}", _0)]
    SerdeYamlError(#[fail(cause)] serde_yaml::Error),
    #[fail(display="{}", _0)]
    IoError(#[fail(cause)] std::io::Error),
    #[fail(display="NonExtantFile: {}", _0)]
    NonExtantFileError(String),
    #[fail(display="AddVersionError: {}", _0)]
    AddVersionError(String),
}

use std::num::ParseIntError;
impl From<ParseIntError> for VersionitisError {
    fn from(err: ParseIntError) -> Self {
        VersionitisError::ParseIntError(err)
    }
}

impl From<serde_yaml::Error> for VersionitisError {
    fn from(err: serde_yaml::Error) -> Self {
        VersionitisError::SerdeYamlError(err)
    }
}


impl From<std::io::Error> for VersionitisError {
    fn from(err: std::io::Error) -> Self {
        VersionitisError::IoError(err)
    }
}
