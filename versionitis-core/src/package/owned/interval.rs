
//! interval.rs
//!
//! Define an owned PackageInterval
// NOT USED CURRENTLY

use crate::{errors::VersionitisError, interval::Interval, package::owned::Package};
use crate::interval::Range;
use std::fmt;
use serde::{
    de::{self, Deserializer, Visitor},
    ser::{Serialize, Serializer},
    Deserialize,
};
use crate::vernum_interval_parser::VerNumIntervalParser;
use crate::version_number::VersionNumber;

/// A package interval expresses a range of package versions
/// using  Interval<T>, where T = package

pub struct PackageInterval {
    name: String,
    interval: VersionNumberInterval
}

impl PackageInterval {
    pub fn new<I: Into<String>>(name: I, interval: VersionNumberInterval) -> Self {
        Self {
            name: name.into(),
            interval
        }
    }

    pub fn from_str(name: &str) -> Result<PackageInterval, VersionitisError> {
        let pieces: Vec<&str> = name.split("-").collect();
        if pieces.len() != 2 {
            return Err(VersionitisError::ParseError(format!("unable to split {}", name)));
        }

        Ok(PackageInterval::new(pieces[0], VersionNumberInterval::from_str(pieces[1])? ))
    }

    pub fn interval(&self) -> VersionNumberInterval {
        self.interval.clone()
    }

    pub fn package_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn contains(&self, package: &Package) -> bool {
        self.interval.contains(package.version_number())
    }

    // pub fn from_range(input: &Range) -> Result<PackageInterval, VersionitisError> {
    //     match *input {
    //         Range::Single(ref name) => {
    //             PackageInterval::from_str(name)
    //         },

    //         Range::HalfOpen(ref p1, ref p2) => Ok(Interval::HalfOpen {
    //             start: VersionNumber::from_str(p1)?,
    //             end: VersionNumber::from_str(p2)?,
    //         }),

    //         Range::Open(ref p1, ref p2) => Ok(Interval::Open {
    //             start: VersionNumber::from_str(p1)?,
    //             end: VersionNumber::from_str(p2)?,
    //         }),
    //     }
    // }
}

pub type VersionNumberInterval = Interval<VersionNumber>;

impl Serialize for VersionNumberInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_range())
    }
}

// PackageVisitor used for serde deserialization
struct VersionNumberIntervalVisitor;

// Visitor implemented as part of custom serde pass
impl<'de> Visitor<'de> for VersionNumberIntervalVisitor {
    type Value = VersionNumberInterval;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a str of the form name-version (eg fred-0.1.0)")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match VerNumIntervalParser::parse(value) {
            Ok(v) => Ok(v),
            Err(e) => panic!("unable to deserialize: {}", e),
        }
    }
}

impl<'de> Deserialize<'de> for VersionNumberInterval {
    fn deserialize<D>(deserializer: D) -> Result<VersionNumberInterval, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(VersionNumberIntervalVisitor)
    }
}

impl VersionNumberInterval {
    /// Retrieve the package name for the PackageInterval as a &str.

    pub fn from_str(name: &str) -> Result<VersionNumberInterval, VersionitisError> {
        VerNumIntervalParser::parse(name)
    }

    /// Convert the internal representatino to a compact range format.
    pub fn to_range(&self) -> String {
        match *self {
            Interval::Single(ref v) => {
                v.to_string()
            }

            Interval::HalfOpen { ref start, ref end } => {
                format!("{}<{}", start.to_string(), end.to_string())
            }

            Interval::Open { ref start, ref end } => {
                format!("{}<={}",start.to_string(), end.to_string())
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
    pub fn from_range(input: &Range) -> Result<VersionNumberInterval, VersionitisError> {
        match *input {
            Range::Single(ref name) => {
                //VerNumIntervalParser::parse(name)
                Ok(Interval::Single(VersionNumber::from_str(name)?))
            },

            Range::HalfOpen(ref p1, ref p2) => Ok(Interval::HalfOpen {
                start: VersionNumber::from_str(p1)?,
                end: VersionNumber::from_str(p2)?,
            }),

            Range::Open(ref p1, ref p2) => Ok(Interval::Open {
                start: VersionNumber::from_str(p1)?,
                end: VersionNumber::from_str(p2)?,
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const SINGLE: &'static str = "---\n1.2.3";

    #[test]
    fn deserialize() {
        let pi: VersionNumberInterval = serde_yaml::from_str(&SINGLE).unwrap();
        let expect = VersionNumberInterval::from_range(&Range::Single("1.2.3")).unwrap();
        assert_eq!(pi,expect);
    }

    #[test]
    fn serialize_single_interval() {
        let interval = VersionNumberInterval::from_range(&Range::Single("1.2.3")).unwrap();
        let result = serde_yaml::to_string(&interval);
        let expect = "---\n1.2.3".to_string();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expect);
    }

    #[test]
    fn deserialize_single_interval() {
        let interval_str = "---\n1.2.3";
        let result: VersionNumberInterval = serde_yaml::from_str(interval_str).unwrap();
        let expect = VersionNumberInterval::from_range(&Range::Single("1.2.3")).unwrap();
        assert_eq!(result, expect);
    }

    #[test]
    fn serialize_halfopen_interval() {
        let interval = VersionNumberInterval::from_range(&Range::HalfOpen("1.2.3", "2.0.0")).unwrap();
        let result = serde_yaml::to_string(&interval);
        let expect = "---\n1.2.3<2.0.0".to_string();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expect);
    }

    #[test]
    fn deserialize_halfopen_interval() {
        let interval_str = "---\n1.2.3<2.0.0";
        let result: VersionNumberInterval = serde_yaml::from_str(interval_str).unwrap();
        let expect = VersionNumberInterval::from_range(&Range::HalfOpen("1.2.3", "2.0.0")).unwrap();
        assert_eq!(result, expect);
    }
    #[test]
    fn serialize_open_interval() {
        let interval = VersionNumberInterval::from_range(&Range::Open("1.2.3", "2.0.0")).unwrap();
        let result = serde_yaml::to_string(&interval);
        let expect = "---\n1.2.3<=2.0.0".to_string();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expect);
    }

    #[test]
    fn deserialize_open_interval() {
        let interval_str = "---\n1.2.3<=2.0.0";
        let result: VersionNumberInterval = serde_yaml::from_str(interval_str).unwrap();
        let expect = VersionNumberInterval::from_range(&Range::Open("1.2.3", "2.0.0")).unwrap();
        assert_eq!(result, expect);
    }

    #[test]
    fn convert_single_to_range() {
        let pi = VersionNumberInterval::from_range(&Range::Single("1.2.3")).unwrap();
        let result = pi.to_range();
        assert_eq!(result, "1.2.3");
    }

    #[test]
    fn convert_open_to_range() {
        let pi = VersionNumberInterval::from_range(&Range::Open("1.2.3", "2.0.0")).unwrap();
        let result = pi.to_range();
        assert_eq!(result, "1.2.3<=2.0.0");
    }

    #[test]
    fn convert_half_open_to_range() {
        let pi = VersionNumberInterval::from_range(&Range::HalfOpen("1.2.3", "2.0.0")).unwrap();
        let result = pi.to_range();
        assert_eq!(result, "1.2.3<2.0.0");
    }
}
