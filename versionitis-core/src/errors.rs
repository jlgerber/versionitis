//! errors.rs
//!
//! Define an implementation of the Fail trait defined
//! in the Failure crate for the project.

use failure::Fail;
use std::fmt::Debug;

/// The primary error class for Versionitis, it leverages the
/// ever popular Failure crate.
#[derive(Fail, Debug,Eq,PartialEq)]
pub enum VersionitisError {
    #[fail(display = "{}", _0)]
    ParseIntError(#[fail(cause)] std::num::ParseIntError),
    #[fail(display="InvalidPackageVersion: {}", _0)]
    InvalidPackageVersion(String),
    #[fail(display="UnknownPackage: {}", _0)]
    UnknownPackage(String),
    #[fail(display="{}", _0)]
    SerdeYamlError(String),
    #[fail(display="IoError: {}", _0)]
    IoError(String),
    #[fail(display="NonExtantFile: {}", _0)]
    NonExtantFileError(String),
    #[fail(display="AddVersionError: {}", _0)]
    AddVersionError(String),
    #[fail(display="DuplicatePackageDependency: {}", _0)]
    DuplicatePackageDependency(String),
}

use std::num::ParseIntError;
impl From<ParseIntError> for VersionitisError {
    fn from(err: ParseIntError) -> Self {
        VersionitisError::ParseIntError(err)
    }
}

impl From<serde_yaml::Error> for VersionitisError {
    fn from(err: serde_yaml::Error) -> Self {
        VersionitisError::SerdeYamlError(err.to_string())
    }
}

impl From<std::io::Error> for VersionitisError {
    fn from(err: std::io::Error) -> Self {
        VersionitisError::IoError(err.to_string())
    }
}
