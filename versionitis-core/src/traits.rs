//! traits.rs
//!
//! Define all traits for versionitis-core
use core::str::FromStr;
use std::fmt::Debug;

/// Trait for defines a version scheme, which must be comparable,
/// convertable to and from string, and debugable
pub trait Versionable: Eq + Ord + Debug + ToString + FromStr {}

/// Trait defines a package database interface. A package database is responsible
/// for tracking all of the distinct vers
pub trait TrackPackages {
    type AddReturns;
    type GetReturns: Debug + Eq + Ord;
    type Errors: failure::Fail;

    /// Add a package version, expressed as a &str
    fn add_version(
        &mut self,
        package_name: &str,
        version: &str,
    ) -> Result<Self::AddReturns, Self::Errors>;

    /// Given a package name (sans version), fetch a vector of Packages wrapped in a
    /// Result. If no package with the supplied name exits, return an UnknownPackageError,
    /// wrapped in a Result.
    fn get<'a>(&'a self, package: &str) -> Result<&'a Vec<Self::GetReturns>, Self::Errors>;
}
