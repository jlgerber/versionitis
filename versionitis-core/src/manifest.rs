//! manifest.rs
//!
//! stores package dependencies
//!
use crate::{errors::VersionitisError, interval::Interval, package::owned::Package};
use serde::ser::{Serialize, SerializeStructVariant, Serializer};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashSet;
use crate::package::owned::interval::{PackageInterval};
use crate::interval::Range;


pub type IntervalSet = HashSet<PackageInterval>;

/// A manifest stores a set of dependencies for a named package.
/// The dependencies are modeled as a HashSet<Interval<Package>>.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn new<I>(name: I) -> Self
    where
        I: Into<String>,
    {
        Self {
            name: name.into(),
            dependencies: IntervalSet::new(),
        }
    }

    /// return the name the package
    pub fn package(&self) -> &str {
        return self.name.as_str();
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
        let package_name = interval.package_name(); //package_name_for(&interval);
        if self.depends_on(package_name) {
            return Err(VersionitisError::DuplicatePackageDependency(
                package_name.to_string(),
            ));
        }
        self.dependencies.insert(interval);
        Ok(())
    }

    /// Test whether a manifest has a package as a dependency. This method is
    /// only concerned with a package name. It will match any version
    pub fn depends_on(&self, name: &str) -> bool {
        for dep in &self.dependencies {
            let found = match dep {
                Interval::Single(ref v) => name == v.name(),
                // shouldn't need to test both start and end since
                // the package name should be guaranteed to be the same
                Interval::HalfOpen { ref start, .. } => name == start.name(),

                Interval::Open { ref start, .. } => name == start.name(),
            };
            if found {
                return true;
            }
        }
        false
    }

    /// Test whether a manifest has a particular versioned packaage as a
    /// dependency. For intervals, this means that the Package is contained within.
    pub fn depends_on_package(&self, package: &Package) -> bool {
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

    mod manifest {
        use super::*;

        #[test]
        fn get_package_spec() {
            type PI = PackageInterval;
            use self::Range::*;
            let mut manifest = Manifest::new("fred-1.0.0");
            let interval1 = PI::from_range(&Single("foo-0.1.0")).unwrap();
            let interval2 = PI::from_range(&HalfOpen("bar-0.1.0", "bar-1.0.0")).unwrap();
            manifest.add_dependency(interval1).unwrap();
            manifest.add_dependency(interval2).unwrap();

            let name = manifest.package();
            assert_eq!(name, "fred-1.0.0");
        }

        #[test]
        fn add_dependencies() {
            type PI = PackageInterval;
            use self::Range::*;
            let mut manifest = Manifest::new("fred-1.0.0");
            let interval1 = PI::from_range(&Single("foo-0.1.0")).unwrap();
            let interval2 = PI::from_range(&HalfOpen("bar-0.1.0", "bar-1.0.0")).unwrap();
            manifest.add_dependency(interval1).unwrap();
            manifest.add_dependency(interval2).unwrap();
            assert_eq!(manifest.dependencies.len(), 2);
        }

        #[test]
        fn add_dependencies_src() {
            // make this a bit more ergonomic to type
            type PI = PackageInterval;
            use self::Range::*;
            let mut manifest = Manifest::new("fred-1.0.0");
            let interval1 = PI::from_range(&Single("foo-0.1.0")).unwrap();
            let interval2 = PI::from_range(&HalfOpen("bar-0.1.0", "bar-1.0.0")).unwrap();
            manifest.add_dependency(interval1).unwrap();
            manifest.add_dependency(interval2).unwrap();
            assert_eq!(manifest.dependencies.len(), 2);
        }

        #[test]
        fn cannot_add_duplicate_dependency() {
            type PI = PackageInterval;
            use self::Range::*;
            let mut manifest = Manifest::new("fred-1.0.0");
            let interval1 = PI::from_range(&Single("foo-0.1.0")).unwrap();
            let interval2 = PI::from_range(&HalfOpen("bar-0.1.0", "bar-1.0.0")).unwrap();
            let interval3 = PI::from_range(&Open("bar-1.1.0", "bar-2.0.0")).unwrap();

            manifest.add_dependency(interval1).unwrap();
            manifest.add_dependency(interval2).unwrap();
            let result = manifest.add_dependency(interval3);
            assert!(
                result.is_err(),
                "should return err when attempting to add duplicate package"
            );
        }

        #[test]
        fn depends_on() {
            type PI = PackageInterval;
            use self::Range::*;
            let mut manifest = Manifest::new("fred-1.0.0");
            let interval1 = PI::from_range(&Single("foo-0.1.0")).unwrap();
            let interval2 = PI::from_range(&HalfOpen("bar-0.1.0", "bar-1.0.0")).unwrap();
            let interval3 = PI::from_range(&Open("bla-0.1.0", "bla-1.0.0")).unwrap();
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
            let pfs = |n: &str| Package::from_str(n).unwrap();
            type PI = PackageInterval;
            use self::Range::*;
            let mut manifest = Manifest::new("fred-1.0.0");
            let interval1 = PI::from_range(&Single("foo-0.1.0")).unwrap();
            let interval2 = PI::from_range(&HalfOpen("bar-0.1.0", "bar-1.0.0")).unwrap();
            let interval3 = PI::from_range(&Open("bla-0.1.0", "bla-1.0.0")).unwrap();
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

        const MANIFEST: &'static str = r#"---
name: fred-1.0.0
dependencies:
  - open:
      start: bla-0.1.0
      end: bla-1.0.0
  - single: foo-0.1.0
  - half_open:
      start: bar-0.1.0
      end: bar-1.0.0"#;

        #[test]
        fn serialize_manifest() {
            let result: serde_yaml::Result<Manifest> = serde_yaml::from_str(MANIFEST);
            assert!(result.is_ok() );
        }


        #[test]
        fn deserialize_manifest() {
            // create a manifest
            let pfs = |n: &str| Package::from_str(n).unwrap();
            type PI = PackageInterval;
            use self::Range::*;
            let mut manifest = Manifest::new("fred-1.0.0");
            let interval1 = PI::from_range(&Single("foo-0.1.0")).unwrap();
            let interval2 = PI::from_range(&HalfOpen("bar-0.1.0", "bar-1.0.0")).unwrap();
            let interval3 = PI::from_range(&Open("bla-0.1.0", "bla-1.0.0")).unwrap();
            manifest.add_dependency(interval1).unwrap();
            manifest.add_dependency(interval2).unwrap();
            manifest.add_dependency(interval3).unwrap();
            //serialize the manifest to a string
            let result = serde_yaml::to_string(&manifest);
            assert!(result.is_ok());
            //convert the string back to a manifest via serde
            let result = result.unwrap();
            let expected: Manifest = serde_yaml::from_str(&result).unwrap();
            // verify that the original manifest looks like the round trip.
            // we are doing this because we cannot guarantee the order of the
            // serialized fields, so doing a string compare doesnt work consistently
            assert_eq!(manifest, expected);
        }
    }
}
