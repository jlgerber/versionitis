
//! interval.rs
//!
//! Define an owned PackageInterval
use crate::{errors::VersionitisError, interval::Interval, package::owned::Package};
use serde::ser::{Serialize, SerializeStructVariant, Serializer};
use serde_derive::{Deserialize, Serialize};
use crate::interval::PISrc;

/// A package interval expresses a range of package versions
/// using  Interval<T>, where T = package
pub type PackageInterval = Interval<Package>;

impl Serialize for PackageInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Interval::Single(ref v) => {
                serializer.serialize_newtype_variant("Interval", 0, "single", &v.spec())
            }

            Interval::HalfOpen { ref start, ref end } => {
                let mut state =
                    serializer.serialize_struct_variant("Interval", 0, "half_open", 2)?;
                state.serialize_field("start", &start.spec())?;
                state.serialize_field("end", &end.spec())?;
                state.end()
            }

            Interval::Open { ref start, ref end } => {
                let mut state = serializer.serialize_struct_variant("Interval", 0, "open", 2)?;
                state.serialize_field("start", &start.spec())?;
                state.serialize_field("end", &end.spec())?;
                state.end()
            }
        }
    }
}

impl PackageInterval {
    /// Retrieve the package name for the PackageInterval as a &str.
    pub fn package_name(&self) -> &str {
        match *self {
            Interval::Single(ref v) => v.name(),

            Interval::HalfOpen { ref start, .. } => start.name(),

            Interval::Open { ref start, .. } => start.name(),
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
            PISrc::Single(ref name) => Ok(Interval::Single(Package::from_str(name)?)),

            PISrc::HalfOpen(ref p1, ref p2) => Ok(Interval::HalfOpen {
                start: Package::from_str(p1)?,
                end: Package::from_str(p2)?,
            }),

            PISrc::Open(ref p1, ref p2) => Ok(Interval::Open {
                start: Package::from_str(p1)?,
                end: Package::from_str(p2)?,
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const SINGLE: &'static str = "---\nsingle: foo-1.2.3";

    #[test]
    fn serialize() {
        let pi: PackageInterval = serde_yaml::from_str(&SINGLE).unwrap();
        let expect = PackageInterval::from_src(&PISrc::Single("foo-1.2.3")).unwrap();
        assert_eq!(pi,expect);

    }
}
