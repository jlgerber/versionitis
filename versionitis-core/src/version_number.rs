use crate::errors::VersionitisError;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::string::ToString;

/// VersionNumber implements Versionable trait. A VersionNumber may be comprised of one or more u16 digits
#[derive(PartialEq, PartialOrd, Eq, Ord, Deserialize, Serialize, Hash, Clone)]
pub struct VersionNumber {
    value: Vec<u16>,
    name: String,
}

impl fmt::Debug for VersionNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

//self.0.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(".")
impl fmt::Display for VersionNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = write!(f, "{}", self.name);
        r
    }
}

impl VersionNumber {
    /// Construct a VersionNumber from a vector of u16
    pub fn new(value: Vec<u16>) -> Self {
        let name = value.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(".");
        VersionNumber{value, name}
    }

    pub fn value(&self) -> Vec<u16> {
        self.value.clone()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// construct a VersionNumber with 3 u16 values
    pub fn semver(major: u16, minor: u16, micro: u16) -> Self {
        let value = vec![major, minor, micro];
        Self::new(value)
    }

    /// construct a semver4 from a value
    pub fn semver4(major: u16, minor: u16, micro: u16, patch: u16) -> Self {
        let value = vec![major, minor, micro, patch];
        Self::new(value)
    }

    /// Deprecated. Prefer from_str()
    pub fn from_string(s: &str) -> Result<Self, VersionitisError> {
        Self::from_str(s)
    }

    /// Not the FromString trait because of lifetime requirements
    pub fn from_str(s: &str) -> Result<Self, VersionitisError> {
        // todo support variants

        let mut result: Vec<u16> = Vec::new();
        for x in s.split(".").map(|x| x.parse::<u16>()) {
            let x = x?;
            result.push(x);
        }

        Ok(VersionNumber::new(result))
    }
}

#[macro_export]
macro_rules! vernum {
    ($e:expr) => {
        VersionNumber::from_string(
            stringify!($e)
                .chars()
                .filter(|x| *x != ' ')
                .collect::<String>()
                .as_str(),
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn implements_display_trait() {
        let v = VersionNumber::semver(0, 1, 0);
        let vs = format!("{}", v);
        assert_eq!(vs, "0.1.0".to_string());
    }

    #[test]
    fn can_generate_versnion_nubmer_via_vernum_macro() {
        let sv1 = vernum!(0.1.0).unwrap();
        let sv2 = VersionNumber::from_string("0.1.0").unwrap();
        assert_eq!(sv1, sv2);
    }

    #[test]
    fn two_instances_with_same_inputs_are_equal() {
        let sv1 = VersionNumber::semver(0, 1, 0);
        let sv2 = VersionNumber::semver(0, 1, 0);
        assert_eq!(sv1, sv2);
    }

    #[test]
    fn smaller_vernum_is_less_than_larger_vernum() {
        let sv1 = VersionNumber::semver(0, 0, 1);
        let sv2 = VersionNumber::semver(0, 1, 0);
        assert!(sv1 < sv2);
    }

    #[test]
    fn larger_vernum_is_greater_than_smaller_vernum() {
        let sv1 = VersionNumber::semver(1, 0, 1);
        let sv2 = VersionNumber::semver(0, 1, 0);
        assert!(sv1 > sv2);
    }

    #[test]
    fn complex_inequality_lt() {
        let sv1 = VersionNumber::semver(0, 1, 0);
        let sv2 = VersionNumber::semver4(0, 1, 0, 0);
        assert!(sv1 < sv2);
    }

    #[test]
    fn complex_inequality_lt2() {
        let sv1 = VersionNumber::semver(0, 1, 0);
        let sv2 = VersionNumber::semver4(0, 1, 0, 1);
        assert!(sv1 < sv2);
    }

    #[test]
    fn complex_inequality_gt() {
        let sv1 = VersionNumber::semver(0, 1, 1);
        let sv2 = VersionNumber::semver4(0, 1, 0, 1);
        assert!(sv1 > sv2);
    }

    #[test]
    fn implements_to_string() {
        let package = String::from("0.1.0.1");
        let sv = VersionNumber::semver4(0, 1, 0, 1);
        let result = sv.to_string();
        assert_eq!(result, package);
    }

    #[test]
    fn implements_debug() {
        let package = String::from("0.1.0.1");
        let sv = VersionNumber::semver4(0, 1, 0, 1);
        let result = format!("{:?}", sv);
        assert_eq!(result, package);
    }

    #[test]
    fn can_generate_versionnumber_from_str() {
        let package = String::from("0.1.0.1");
        let sv1 = VersionNumber::from_string(&package).unwrap();
        let sv2 = VersionNumber::semver4(0, 1, 0, 1);
        assert_eq!(sv1, sv2);
    }
}
