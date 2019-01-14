//! manifest.rs
//!
//! stores package dependencies
//!
use crate::interval::Interval;
use std::collections::HashSet;
use crate::version_number::VersionNumber;
use crate::package::owned::Package;
//use crate::package::owned;
use crate::errors::VersionitisError;

/*
pub struct PackageInterval {
    name: String,
    interval: Interval<VersionNumber>,
}

impl PackageInterval {
    pub fn new<I> (name: I, interval: Interval) -> Self where I: Into<String> {
        Self {
            name.into(),
            interval
        }
    }
}
*/

pub type PackageInterval = Interval<Package>;
pub type IntervalSet     = HashSet<PackageInterval>;

impl PackageInterval {
    /// Retrieve the package name for the PackageInterval as a &str
    pub fn package_name(&self) -> &str {
         match *self {
         Interval::Single(ref v) => {
                v.package()
            }

            Interval::HalfOpen{ref start, ..} => {
                start.package()
            }

            Interval::Open{ref start, ..} => {
                start.package()
            }
        }
    }
}


/// Construct a package interval from a &str
///
/// # example
///
/// ```ignore
/// let interval = single_from_str("foo-0.1.0").unwrap();
/// ```
pub fn single_from_str(name: &str) -> Result<Interval<Package>, VersionitisError> {
    Ok(Interval::Single(Package::from_string(name)?))
}

/// Construct a half open package interval from two &str
///
/// # example
///
/// ```ignore
/// let interval = halfopen_from_strs("foo-0.1.0", "foo-1.0.0").unwrap();
/// ```
pub fn halfopen_from_strs(p1: &str, p2: &str) -> Result<Interval<Package>, VersionitisError> {
    Ok(Interval::HalfOpen{
        start: Package::from_string(p1)?,
        end: Package::from_string(p2)?
    })
}

/// Construct an open package interval from two strings
///
/// # example
///
/// ```ignore
/// let interval = open_from_strs("foo-0.1.0", "foo-1.0.0")?;
/// ```
pub fn open_from_strs(p1: &str, p2: &str) -> Result<PackageInterval, VersionitisError> {
    Ok(Interval::Open{
        start: Package::from_string(p1)?,
        end: Package::from_string(p2)?
    })
}


#[derive(Debug,PartialEq,Eq)]
pub struct Manifest {
    name: String,
    dependencies: IntervalSet,
}

impl Manifest {
    /// New up a manifest given a name of type Into<String>
    ///
    /// # example
    ///
    /// ```notest
    /// let manifest = Manifest::new("rustup");
    /// ```
    pub fn new<I>(name: I) -> Self where I: Into<String> {
        Self {
            name: name.into(),
            dependencies: IntervalSet::new(),
        }
    }

    /// Add a dependency to the manifest
    ///
    /// # example
    ///
    /// ```ignore
    /// let manifest = Manifest::new("coolpackage");
    /// let interval = halfopen_from_strs("bar-0.1.0", "bar-1.0.0")?;
    /// manifest.add_dependency(interval)?;
    /// ```
    pub fn add_dependency(&mut self, interval: PackageInterval) -> Result<(), VersionitisError> {

        let package_name = interval.package_name();//package_name_for(&interval);
        if  self.depends_on(package_name ) {
            return Err(VersionitisError::DuplicatePackageDependency(package_name.to_string()));
        }
        self.dependencies.insert(interval);
        Ok(())
    }

    /// Test whether a manifest has a package as a dependency. This method is
    /// only concerned with a package name. It will match any version
    pub fn depends_on(&self, name: &str) -> bool {
        for dep in &self.dependencies {
            let found = match dep {
                Interval::Single(ref v) => {
                    name == v.package()
                }
                // shouldn't need to test both start and end since
                // the package name should be guaranteed to be the same
                Interval::HalfOpen{ref start, ref end} => {
                    name == start.package()
                }

                Interval::Open{ref start, ref end} => {
                    name == start.package()
                }
            };
            if found {
                return true;
            }
        }
        false
    }

    /// Test whether a manifest has a particular versioned packaage as a
    /// dependency. For intervals, this means that the Package is contained within.
    pub fn depends_on_package(&self, package: &Package ) -> bool {
        for dep in &self.dependencies {
            let found = dep.contains(package);
            if found {
                return true;
            }
        }
        false
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    // mod interval {
    //     use super::*;
    //     #[test]
    //     fn can_construct() {
    //         let pinter = PackageInterval::new("foo", Un);
    //     }
    // }

    mod manifest {
        use super::*;

        #[test]
        fn can_construct() {
            let manifest = Manifest::new("fred-1.0.0");
        }

        #[test]
        fn add_dependencies() {
            let mut manifest = Manifest::new("fred-1.0.0");
            let interval1 = single_from_str("foo-0.1.0").unwrap();
            let interval2 = halfopen_from_strs("bar-0.1.0", "bar-1.0.0").unwrap();
            manifest.add_dependency(interval1).unwrap();
            manifest.add_dependency(interval2).unwrap();
            assert_eq!(manifest.dependencies.len(), 2);
        }


        #[test]
        fn cannot_add_duplicate_dependency() {
            let mut manifest = Manifest::new("fred-1.0.0");
            let interval1 = single_from_str("foo-0.1.0").unwrap();
            let interval2 = halfopen_from_strs("bar-0.1.0", "bar-1.0.0").unwrap();
            let interval3 = open_from_strs("bar-1.1.0", "bar-2.0.0").unwrap();

            manifest.add_dependency(interval1).unwrap();
            manifest.add_dependency(interval2).unwrap();
            let result = manifest.add_dependency(interval3);
            assert!(result.is_err(), "should return err when attempting to add duplicate package");
        }

        #[test]
        fn depends_on() {
            let mut manifest = Manifest::new("fred-1.0.0");
            let interval1 = single_from_str("foo-0.1.0").unwrap();
            let interval2 = halfopen_from_strs("bar-0.1.0", "bar-1.0.0").unwrap();
            let interval3 = open_from_strs("bla-0.1.0", "bla-1.0.0").unwrap();
            manifest.add_dependency(interval1).unwrap();
            manifest.add_dependency(interval2).unwrap();
            manifest.add_dependency(interval3).unwrap();
            assert!(manifest.depends_on("foo"));
            assert!(manifest.depends_on("bar"));
            assert!(manifest.depends_on("bla"));
            assert!(!manifest.depends_on("blargybalargy"));
        }

        #[test]
        fn depends_on_package() {
            let pfs = |n: &str| {
                Package::from_string(n).unwrap()
            };
            let mut manifest = Manifest::new("fred-1.0.0");
            let interval1 = single_from_str("foo-0.1.0").unwrap();
            let interval2 = halfopen_from_strs("bar-0.1.0", "bar-1.0.0").unwrap();
            let interval3 = open_from_strs("bla-0.1.0", "bla-1.0.0").unwrap();
            manifest.add_dependency(interval1).unwrap();
            manifest.add_dependency(interval2).unwrap();
            manifest.add_dependency(interval3).unwrap();
            // single
            assert!(manifest.depends_on_package(&pfs("foo-0.1.0")));
            assert!(!manifest.depends_on_package(&pfs("foo-1.1.0")));

            // half open
            assert!(manifest.depends_on_package(&pfs("bar-0.1.0")));
            assert!(manifest.depends_on_package(&pfs("bar-0.5.0")));
            assert!(!manifest.depends_on_package(&pfs("bar-0.0.1")));
            assert!(!manifest.depends_on_package(&pfs("bar-1.0.0")));
            // open
            assert!(manifest.depends_on_package(&pfs("bla-0.1.0")));
            assert!(manifest.depends_on_package(&pfs("bla-1.0.0")));
            assert!(!manifest.depends_on_package(&pfs("bla-1.1.0")));

            // not a package
            assert!(!manifest.depends_on_package(&pfs("blargybalargy-1.0.0")));
        }
    }
}