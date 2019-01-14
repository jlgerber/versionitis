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
pub type IntervalSet = HashSet<PackageInterval>;

#[derive(Debug,PartialEq,Eq)]
pub struct Manifest {
    name: String,
    interval: IntervalSet,
}

impl Manifest {
    pub fn new<I>(name: I) -> Self where I: Into<String> {
        Self {
            name: name.into(),
            interval: IntervalSet::new(),
        }
    }

    /// Add an interval to the set
    pub fn add_interval(&mut self, interval: PackageInterval) -> Result<(), VersionitisError> {
        self.interval.insert(interval);
        Ok(())
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
        fn add_interval() {
            let pfs = |p:&str| {Interval::Single(Package::from_string(p).unwrap())};
            let pfi = |p1: &str,p2: &str | {
                Interval::HalfOpen{
                    start: Package::from_string(p1).unwrap(),
                    end: Package::from_string(p2).unwrap()
                }
            };

            let mut manifest = Manifest::new("fred-1.0.0");
            //let interval1 = PackageInterval::Single(Package::from_string("foo-0.1.0").unwrap());
            //let interval2 = PackageInterval::HalfOpen{start: owned::version!("bar-0.1.0"), end: owned::version!("bar-1.0.0") };
            let interval1 = pfs("foo-0.1.0");
            let interval2 = pfi("bar-0.1.0", "bar-1.0.0");
            manifest.add_interval(interval1).unwrap();
            manifest.add_interval(interval2).unwrap();
            assert_eq!(manifest.interval.len(), 2);
        }

    }
}