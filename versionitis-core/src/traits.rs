use std::{
    fmt::Debug,
    error::Error,
};
use core::str::FromStr;

/// Trait for defining a version scheme
pub trait Versionable: Eq + Ord + Debug + ToString + FromStr {}

/// Trait defines a package database interface. A package database is responsible
/// for tracking all of the distinct vers
pub trait TrackPackages {
    type AddReturns;
    type GetReturns: Debug+Eq+Ord;
    type Errors: failure::Fail;

    /// Add a package version
    fn add_version(&mut self, package_name: &str, version: &str)
    -> Result<Self::AddReturns, Self::Errors>;

    /// Given a package name (sans version), fetch a vector of Packages wrapped in a
    /// Result. If no package with the supplied name exits, return an UnknownPackageError,
    /// wrapped in a Result.
    fn get<'a>(&'a self, package: &str) -> Result<&'a Vec<Self::GetReturns>, Self::Errors>;
 }