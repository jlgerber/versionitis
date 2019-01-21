
//! interval.rs
//!
//! Define an owned PackageInterval
use crate::{errors::VersionitisError, interval::Interval, package::owned::Package};
//use serde::ser::{Serialize, SerializeStructVariant, Serializer};
//use serde_derive::{Deserialize, Serialize};
// TODO: chance PackagetInterval to be Interval<VersionNumber>
// TODO: change package_ineterval.pest to remove quotes and package name
use crate::interval::Range;
use std::fmt;
use serde::{
    de::{self, Deserializer, Visitor},
    ser::{Serialize,SerializeStructVariant, Serializer},
    Deserialize,
};
use crate::package_interval_parser::PackageIntervalParser;
use crate::version_number::VersionNumber;

/// A package interval expresses a range of package versions
/// using  Interval<T>, where T = package
pub type PackageInterval = Interval<VersionNumber>;


impl Serialize for PackageInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Interval::Single(ref v) => {
                serializer.serialize_newtype_variant("Interval", 0, "single", &v.to_string())
            }

            Interval::HalfOpen { ref start, ref end } => {
                let mut state =
                    serializer.serialize_struct_variant("Interval", 0, "half_open", 2)?;
                state.serialize_field("start", &start.to_string())?;
                state.serialize_field("end", &end.to_string())?;
                state.end()
            }

            Interval::Open { ref start, ref end } => {
                let mut state = serializer.serialize_struct_variant("Interval", 0, "open", 2)?;
                state.serialize_field("start", &start.to_string())?;
                state.serialize_field("end", &end.to_string())?;
                state.end()
            }
        }
    }
}

// PackageVisitor used for serde deserialization
struct PackageIntervalVisitor;
// Visitor implemented as part of custom serde pass
impl<'de> Visitor<'de> for PackageIntervalVisitor {
    type Value = PackageInterval;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a str of the form name-version (eg fred-0.1.0)")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match PackageIntervalParser::parse(value) {
            Ok(v) => Ok(v),
            Err(e) => panic!("unable to deserialize: {}", e),
        }
    }
}

impl<'de> Deserialize<'de> for PackageInterval {
    fn deserialize<D>(deserializer: D) -> Result<PackageInterval, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(PackageIntervalVisitor)
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

    /// Convert the internal representatino to a compact range format.
    pub fn to_range(&self) -> String {
        match *self {
            Interval::Single(ref v) => {
                format!("{}: '{}'",v.name(), v.version().to_string())
            }

            Interval::HalfOpen { ref start, ref end } => {
                format!("{}: '{}<{}'", start.name(), start.version(), end.version())
            }

            Interval::Open { ref start, ref end } => {
                format!("{}: '{}<={}'", start.name(), start.version(), end.version())
            }
        }


    }
    /// Constructs a PackageInterval from a Src enum reference.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let package_interval = PackageInterval::from_range(&Range::Open("foo-0.1.0", "foo-1.0.0"))?;
    /// ```
    ///
    /// One may wish to make this more ergonomic though:
    ///
    /// ```ignore
    /// type PI = PackageInterval;
    /// use self::Range::Open;
    /// let package_interval = PI::from_range(&Open("foo-0.1.0", "foo-1.0.0"))?;
    /// ```
    pub fn from_range(input: &Range) -> Result<PackageInterval, VersionitisError> {
        match *input {
            Range::Single(ref name) => Ok(Interval::Single(Package::from_str(name)?)),

            Range::HalfOpen(ref p1, ref p2) => Ok(Interval::HalfOpen {
                start: Package::from_str(p1)?,
                end: Package::from_str(p2)?,
            }),

            Range::Open(ref p1, ref p2) => Ok(Interval::Open {
                start: Package::from_str(p1)?,
                end: Package::from_str(p2)?,
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const SINGLE: &'static str = "---\n'1.2.3'";

    #[test]
    fn deserialize() {
        let pi: PackageInterval = serde_yaml::from_str(&SINGLE).unwrap();
        let expect = PackageInterval::from_range(&Range::Single("foo-1.2.3")).unwrap();
        assert_eq!(pi,expect);

    }

    #[test]
    fn convert_single_to_range() {
        let pi = PackageInterval::from_range(&Range::Single("foo-1.2.3")).unwrap();
        let result = pi.to_range();
        assert_eq!(result, "foo: '1.2.3'");
    }

    #[test]
    fn convert_open_to_range() {
        let pi = PackageInterval::from_range(&Range::Open("foo-1.2.3", "foo-2.0.0")).unwrap();
        let result = pi.to_range();
        assert_eq!(result, "foo: '1.2.3<=2.0.0'");
    }

    #[test]
    fn convert_half_open_to_range() {
        let pi = PackageInterval::from_range(&Range::HalfOpen("foo-1.2.3", "foo-2.0.0")).unwrap();
        let result = pi.to_range();
        assert_eq!(result, "foo: '1.2.3<2.0.0'");
    }
}
