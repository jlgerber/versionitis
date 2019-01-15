//! repo.rs
//!
//! Store and retrieve package versions. The repo is intended to
//! hold the available packages in memory.
use crate::{
    errors::VersionitisError,
    package::owned::Package,
    traits::TrackPackages,
};
use serde_derive::{Deserialize,Serialize};
use std::{
    collections::HashMap,
    iter::Iterator,
};

// type alias
type PackageMap = HashMap<String, Vec<Package>>;

/// The Repo stores package versions for each package
#[derive(Debug,PartialEq,Eq,Deserialize,Serialize)]
pub struct Repo {
    pub packages: PackageMap,
    unchecked: bool, // have we called add_version_nocheck
}

impl Repo {
    /// create a new package repository.
    pub fn new() -> Self {
        Self {
            packages: PackageMap::new(),
            unchecked: false,
        }
    }

    /// Walk through each of the keys and sort
    pub fn dedup_sort(&mut self) {
        self.packages.iter_mut().for_each(|(key,v)| {
            v.sort();
            v.dedup();
            // TODO: do we need this here? only after deserialization. perhaps
            // i will put it there.
            v.iter_mut().filter(|x| x.package() == key ).for_each(|_|{});
        });
        self.unchecked = false;
    }

    /// Add version but do not bother to check for duplicates / monotonic
    /// increase. IFF you are going to add a bunch of versions in arbitary
    /// order then use add_version_nocheck and call dedup_sort afterwards
    pub fn add_version_unchecked(&mut self, package_name: &str, version: &str)
    -> Result<(), VersionitisError> {
        self.add_version_imp(package_name, version, false)
    }

    /// Implementation for add_version and add_version_nocheck
    fn add_version_imp(&mut self, package_name: &str, version: &str, check: bool)
    -> Result<(), VersionitisError> {
        let pack = Package::from_strs(package_name, version)?;
        // retrieve the vector of package versions for the supplied
        // package name. If it exists, verify that the new package's
        // version is greater than the version of the last package in
        // the aforementioned vector. Otherwise, return an InvalidPackageVersion
        // error.
        match self.packages.get_mut(package_name) {
            Some(ref mut lst) => {
                if check {
                    if let Some(last_elem) = lst.last()  {
                        if *last_elem >= pack {
                            return Err(
                                VersionitisError::InvalidPackageVersion(pack.to_string())
                            );
                        }
                    }
                } else {
                    self.unchecked = true;
                }
                // now we add the new versioned package to the mutable reference
                // to the vector
                lst.push(pack);
                Ok(())
            },
            _ => {
                // the package key does not exist. Create it and add a vec
                // value which has the new versioned package.
                self.packages.insert(package_name.to_string(), vec![pack]);
                Ok(())
            },
        }
    }

    /// Is the repo guaranteed to have deduplicated and ordered package
    /// versions?
    pub fn is_clean(&self) -> bool {
        !self.unchecked
    }
}

impl TrackPackages for Repo {
    type AddReturns = ();
    type GetReturns = Package;
    type Errors = VersionitisError;
    /// Add a package version to the repository. Supply a package
    /// name and version, as &strs. The add_version method will
    /// construt a Package instance and add it to the repo, if it
    /// isn't already present.
    fn add_version(&mut self, package_name: &str, version: &str)
    -> Result<Self::AddReturns, Self::Errors> {
        self.add_version_imp(package_name, version, true)

    }

    /// Given a package name (sans version), fetch a vector of Packages wrapped in a
    /// Result. If no package with the supplied name exits, return an UnknownPackageError,
    /// wrapped in a Result.
    fn get<'a>(&'a self, package: &str) -> Result<&'a Vec<Package>, VersionitisError> {
        match self.packages.get(package) {
            Some(ref pv) => Ok(pv),
            None => Err(VersionitisError::UnknownPackage(package.to_string()))
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    const REPO: &'static str = r#"---
packages:
  fred:
    - name: fred
      version:
        value:
          - 0
          - 1
          - 0
    - name: fred
      version:
        value:
          - 0
          - 2
          - 0
    - name: fred
      version:
        value:
          - 0
          - 2
          - 1
    - name: fred
      version:
        value:
          - 0
          - 2
          - 3
    - name: fred
      version:
        value:
          - 0
          - 3
          - 0
unchecked: false"#;

    fn setup_repo(package_name: &str) -> Repo {
        let mut repo = Repo::new();

        repo.add_version(package_name, "0.1.0").unwrap();
        repo.add_version(package_name, "0.2.0").unwrap();
        repo.add_version(package_name, "0.2.1").unwrap();
        repo.add_version(package_name, "0.2.3").unwrap();
        repo.add_version(package_name, "0.3.0").unwrap();

        repo
    }

    #[test]
    fn can_get_a_package() {
        let package_version = Package::from_strs("foo", "0.1.0");
        let repo = setup_repo("foo");
        let package = repo.get("foo");
        assert_eq!(package.unwrap()[0], package_version.unwrap());
    }

    #[test]
    fn setup_nocheck_allows_dups_and_unordered_inserts() {
        let mut repo = Repo::new();
        let package_name = "fred";
        repo.add_version_unchecked(package_name, "0.2.0").unwrap();
        repo.add_version_unchecked(package_name, "0.1.0").unwrap();
        repo.add_version_unchecked(package_name, "0.1.0").unwrap();
        repo.add_version_unchecked(package_name, "0.2.1").unwrap();
        repo.add_version_unchecked(package_name, "0.3.0").unwrap();
        repo.add_version_unchecked(package_name, "0.2.3").unwrap();
        assert_eq!(repo.get(package_name).unwrap().len(), 6);
    }


    #[test]
    fn dedup_sort_cleans_up() {
        let mut repo = Repo::new();
        let package_name = "fred";
        // make a mess
        repo.add_version_unchecked(package_name, "0.2.0").unwrap();
        repo.add_version_unchecked(package_name, "0.1.0").unwrap();
        // duplicate insert
        repo.add_version_unchecked(package_name, "0.1.0").unwrap();
        repo.add_version_unchecked(package_name, "0.2.1").unwrap();
        repo.add_version_unchecked(package_name, "0.3.0").unwrap();
        // out of order insert
        repo.add_version_unchecked(package_name, "0.2.3").unwrap();
        // clean up
        repo.dedup_sort();

        let package = repo.get(package_name).unwrap();
        let versions = vec!["0.1.0", "0.2.0", "0.2.1","0.2.3", "0.3.0"];
        package.iter().enumerate().for_each(|(idx,pack)| {
            assert_eq!(pack, &Package::from_strs(package_name, versions[idx]).unwrap());
        });
    }

    #[test]
    fn serialize() {
        let mut repo = Repo::new();
        let package_name = "fred";
        // make a mess
        repo.add_version_unchecked(package_name, "0.2.0").unwrap();
        repo.add_version_unchecked(package_name, "0.1.0").unwrap();
        // duplicate insert
        repo.add_version_unchecked(package_name, "0.1.0").unwrap();
        repo.add_version_unchecked(package_name, "0.2.1").unwrap();
        repo.add_version_unchecked(package_name, "0.3.0").unwrap();
        // out of order insert
        repo.add_version_unchecked(package_name, "0.2.3").unwrap();
        // clean up
        repo.dedup_sort();
        let s = serde_yaml::to_string(&repo).unwrap();
        assert_eq!(s, REPO);
    }

    #[test]
    fn deserialize_from_yaml() {
        let mut repo = Repo::new();
        let package_name = "fred";
        // make a mess
        repo.add_version_unchecked(package_name, "0.2.0").unwrap();
        repo.add_version_unchecked(package_name, "0.1.0").unwrap();
        // duplicate insert
        repo.add_version_unchecked(package_name, "0.1.0").unwrap();
        repo.add_version_unchecked(package_name, "0.2.1").unwrap();
        repo.add_version_unchecked(package_name, "0.3.0").unwrap();
        // out of order insert
        repo.add_version_unchecked(package_name, "0.2.3").unwrap();
        // clean up
        repo.dedup_sort();

    let deserialized: Repo = serde_yaml::from_str(&REPO).unwrap();
    assert_eq!(deserialized, repo);

    }
}

