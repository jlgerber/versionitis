
use crate::{errors::VersionitisError, interval::Interval};
use crate::interval::Range;
use std::fmt;
use serde::{
    de::{self, Deserializer, Visitor},
    ser::{Serialize, Serializer},
    Deserialize,
};
use crate::vernum_interval_parser::VerNumIntervalParser;
use crate::version_number::VersionNumber;
use std::fmt::Display;

/// an Interval of VersionNumbers. Interval is an enum whose variants
/// define various intervals between VersionNumbers.
pub type VersionNumberInterval = Interval<VersionNumber>;

impl Display for VersionNumberInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_range())
    }
}

impl VersionNumberInterval {
    /// Retrieve the package name for the PackageInterval as a &str.

    pub fn from_str(name: &str) -> Result<VersionNumberInterval, VersionitisError> {
        VerNumIntervalParser::parse(name)
    }

    /// Convert the internal representation to a compact range format.
    ///
    /// The mapping is as follows
    ///
    /// Variant | String
    /// --- | ---
    /// Single(1.2.3) | 1.2.3
    /// HalfOpen(1.2.3, 2.0.0) | 1.2.3<2.0.0
    /// Open(1.2.3, 2.0.0) | 1.2.3<=2.0.0
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

    /// Construct a PackageInterval from a Src enum reference.
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
            //TODO: look into returning an error instead of panicing.
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


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_serialize_a_single_interval() {
        let interval = VersionNumberInterval::from_range(&Range::Single("1.2.3")).unwrap();
        let result = serde_yaml::to_string(&interval);
        let expect = "---\n1.2.3".to_string();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expect);
    }

    #[test]
    fn can_deserialize_a_single_interval() {
        let interval_str = "---\n1.2.3";
        let result: VersionNumberInterval = serde_yaml::from_str(interval_str).unwrap();
        let expect = VersionNumberInterval::from_range(&Range::Single("1.2.3")).unwrap();
        assert_eq!(result, expect);
    }

    #[test]
    fn can_serialize_an_halfopen_interval() {
        let interval = VersionNumberInterval::from_range(&Range::HalfOpen("1.2.3", "2.0.0")).unwrap();
        let result = serde_yaml::to_string(&interval);
        let expect = "---\n1.2.3<2.0.0".to_string();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expect);
    }

    #[test]
    fn can_deserialize_an_halfopen_interval() {
        let interval_str = "---\n1.2.3<2.0.0";
        let result: VersionNumberInterval = serde_yaml::from_str(interval_str).unwrap();
        let expect = VersionNumberInterval::from_range(&Range::HalfOpen("1.2.3", "2.0.0")).unwrap();
        assert_eq!(result, expect);
    }
    #[test]
    fn can_serialize_an_open_interval() {
        let interval = VersionNumberInterval::from_range(&Range::Open("1.2.3", "2.0.0")).unwrap();
        let result = serde_yaml::to_string(&interval);
        let expect = "---\n1.2.3<=2.0.0".to_string();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expect);
    }

    #[test]
    fn can_deserialize_an_open_interval() {
        let interval_str = "---\n1.2.3<=2.0.0";
        let result: VersionNumberInterval = serde_yaml::from_str(interval_str).unwrap();
        let expect = VersionNumberInterval::from_range(&Range::Open("1.2.3", "2.0.0")).unwrap();
        assert_eq!(result, expect);
    }

    #[test]
    fn can_convert_a_single_to_range() {
        let pi = VersionNumberInterval::from_range(&Range::Single("1.2.3")).unwrap();
        let result = pi.to_range();
        assert_eq!(result, "1.2.3");
    }

    #[test]
    fn can_convert_an_open_to_range() {
        let pi = VersionNumberInterval::from_range(&Range::Open("1.2.3", "2.0.0")).unwrap();
        let result = pi.to_range();
        assert_eq!(result, "1.2.3<=2.0.0");
    }

    #[test]
    fn can_convert_a_half_open_to_range() {
        let pi = VersionNumberInterval::from_range(&Range::HalfOpen("1.2.3", "2.0.0")).unwrap();
        let result = pi.to_range();
        assert_eq!(result, "1.2.3<2.0.0");
    }
}
