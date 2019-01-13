use std::fmt;
//use core::str::FromStr;
use crate::errors::VersionitisError;
use serde_derive::{Deserialize,Serialize};
/// VersionNumber implements Versionable trait. A VersionNumber may be comprised of one or more u16 digits
#[derive(PartialEq,PartialOrd,Eq,Ord,Deserialize,Serialize)]
pub struct VersionNumber { //todo: use tuple struct instead of struct
    value: Vec<u16>,
}
impl fmt::Debug for VersionNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.value.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(".");
        write!(f, "{}", name)
    }
}

impl VersionNumber {

    fn construct_name(value: &Vec<u16>) -> String {
        let version = value.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(".");
        format!("{}", version)
    }

    /// Construct a VersionNumber from a vector of u16
    pub fn new(input: Vec<u16>) -> Self {
        Self {
            value: input
        }
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

    /// Not the FromString trait because of lifetime requirements
    pub fn  from_string(s: &str) -> Result<Self, VersionitisError> {
        // todo support variants

        let mut result: Vec<u16> = Vec::new();
        for x in s.split(".").map(|x| x.parse::<u16>()) {
            let x = x?;
            result.push(x);
        }

        Ok(VersionNumber::new(result))
    }
}

impl ToString for VersionNumber {
    fn to_string(&self) -> String {
        self.value.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(".")
    }
}

#[macro_export]
macro_rules! vernum {
    ($e:expr) => {
        VersionNumber::from_string(
        stringify!($e).chars().filter(|x| *x != ' ').collect::<String>().as_str()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macrotest() {
        let sv1 = vernum!( 0.1.0 ).unwrap() ;
        let sv2 = VersionNumber::from_string("0.1.0").unwrap();
        assert_eq!(sv1, sv2);
    }

    #[test]
    fn simple_equality() {
        let sv1 = VersionNumber::semver(0,1,0);
        let sv2 = VersionNumber::semver( 0,1,0);
        assert_eq!(sv1, sv2);
    }

    #[test]
    fn simple_inequality_lt() {
        let sv1 = VersionNumber::semver(0,0,1);
        let sv2 = VersionNumber::semver( 0,1,0);
        assert!(sv1 < sv2);
    }

    #[test]
    fn simple_inequality_gt() {
        let sv1 = VersionNumber::semver( 1,0,1);
        let sv2 = VersionNumber::semver( 0,1,0);
        assert!(sv1 > sv2);
    }

    #[test]
    fn complex_inequality_lt() {
        let sv1 = VersionNumber::semver( 0,1,0);
        let sv2 = VersionNumber::semver4(0,1,0,0);
        assert!(sv1 < sv2);
    }

    #[test]
    fn complex_inequality_lt2() {
        let sv1 = VersionNumber::semver(0,1,0);
        let sv2 = VersionNumber::semver4( 0,1,0,1);
        assert!(sv1 < sv2);
    }

    #[test]
    fn complex_inequality_gt() {
        let sv1 = VersionNumber::semver(0,1,1);
        let sv2 = VersionNumber::semver4( 0,1,0,1);
        assert!(sv1 > sv2);
    }

    #[test]
    fn version() {
        let sv2 = VersionNumber::semver4( 0,1,0,1);
        assert_eq!(sv2.to_string().as_str(), "0.1.0.1" );
    }

    #[test]
    fn to_str() {
        let package = String::from("0.1.0.1");
        let sv = VersionNumber::semver4(0,1,0,1);
        let result = sv.to_string();
        assert_eq!(result, package );
    }

    #[test]
    fn debug() {
        let package = String::from("0.1.0.1");
        let sv = VersionNumber::semver4(0,1,0,1);
        let result = format!("{:?}", sv);
        assert_eq!(result, package );
    }

    #[test]
    fn from_str() {
        let package = String::from("0.1.0.1");
        let sv1 = VersionNumber::from_string(&package).unwrap();
        let sv2 = VersionNumber::semver4( 0,1,0,1);
        assert_eq!(sv1, sv2 );
    }
}
