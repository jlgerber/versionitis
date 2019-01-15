//! manifest.rs
//!
//! stores package dependencies
//!
use crate::{
    errors::VersionitisError,
    interval::Interval,
    package::owned::Package,
    //version_number::VersionNumber,
};
use std::collections::HashSet;

pub type PackageInterval = Interval<Package>;
pub type IntervalSet     = HashSet<PackageInterval>;

/// Enum wrapping possible inputs to from_src
pub enum PISrc<'a> {
    Single(&'a str),
    HalfOpen(&'a str, &'a str),
    Open(&'a str, &'a str)
}

impl PackageInterval {

    /// Retrieve the package name for the PackageInterval as a &str.
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

    /// Constructs a PackageInterval from a Src enum reference.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let package_interval = PackageInterval::from_src(&PISrc::Open("foo-0.1.0", "foo-1.0.0"))?;
    /// ```
    ///
    /// One may wish to make this more ergonomic though:
    ///
    /// ```ignore
    /// type PI = PackageInterval;
    /// use self::PISrc::Open;
    /// let package_interval = PI::from_src(&Open("foo-0.1.0", "foo-1.0.0"))?;
    /// ```
    pub fn from_src(input: &PISrc) -> Result<PackageInterval, VersionitisError> {
        match *input {
            PISrc::Single(ref name) => {
                Ok(Interval::Single(Package::from_string(name)?))
            }

            PISrc::HalfOpen(ref p1, ref p2) => {
                Ok(Interval::HalfOpen{
                    start: Package::from_string(p1)?,
                    end: Package::from_string(p2)?
                })
            }

            PISrc::Open(ref p1, ref p2) => {
                Ok(Interval::Open{
                    start: Package::from_string(p1)?,
                    end: Package::from_string(p2)?
                })
            }
        }
    }
}

/// A manifest stores a set of dependencies for a named package.
/// The dependencies are modeled as a HashSet<Interval<Package>>.
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
                Interval::HalfOpen{ref start, ..} => {
                    name == start.package()
                }

                Interval::Open{ref start, ..} => {
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
            type PI=PackageInterval;
            use self::PISrc::*;
            let mut manifest = Manifest::new("fred-1.0.0");
            let interval1 = PI::from_src(&Single("foo-0.1.0")).unwrap();
            let interval2 = PI::from_src(&HalfOpen("bar-0.1.0", "bar-1.0.0")).unwrap();
            manifest.add_dependency(interval1).unwrap();
            manifest.add_dependency(interval2).unwrap();
            assert_eq!(manifest.dependencies.len(), 2);
        }

        #[test]
        fn add_dependencies_src() {
            // make this a bit more ergonomic to type
            type PI=PackageInterval;
            use self::PISrc::*;
            let mut manifest = Manifest::new("fred-1.0.0");
            let interval1 = PI::from_src(&Single("foo-0.1.0")).unwrap();
            let interval2 = PI::from_src(&HalfOpen("bar-0.1.0", "bar-1.0.0")).unwrap();
            manifest.add_dependency(interval1).unwrap();
            manifest.add_dependency(interval2).unwrap();
            assert_eq!(manifest.dependencies.len(), 2);
        }

        #[test]
        fn cannot_add_duplicate_dependency() {
            type PI=PackageInterval;
            use self::PISrc::*;
            let mut manifest = Manifest::new("fred-1.0.0");
            let interval1 = PI::from_src(&Single("foo-0.1.0")).unwrap();
            let interval2 = PI::from_src(&HalfOpen("bar-0.1.0", "bar-1.0.0")).unwrap();
            let interval3 = PI::from_src(&Open("bar-1.1.0", "bar-2.0.0")).unwrap();

            manifest.add_dependency(interval1).unwrap();
            manifest.add_dependency(interval2).unwrap();
            let result = manifest.add_dependency(interval3);
            assert!(result.is_err(), "should return err when attempting to add duplicate package");
        }

        #[test]
        fn depends_on() {
            type PI=PackageInterval;
            use self::PISrc::*;
            let mut manifest = Manifest::new("fred-1.0.0");
            let interval1 = PI::from_src(&Single("foo-0.1.0")).unwrap();
            let interval2 = PI::from_src(&HalfOpen("bar-0.1.0", "bar-1.0.0")).unwrap();
            let interval3 = PI::from_src(&Open("bla-0.1.0", "bla-1.0.0")).unwrap();
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
            type PI=PackageInterval;
            use self::PISrc::*;
            let mut manifest = Manifest::new("fred-1.0.0");
            let interval1 = PI::from_src(&Single("foo-0.1.0")).unwrap();
            let interval2 = PI::from_src(&HalfOpen("bar-0.1.0", "bar-1.0.0")).unwrap();
            let interval3 = PI::from_src(&Open("bla-0.1.0", "bla-1.0.0")).unwrap();
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