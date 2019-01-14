use std::fmt;
//use core::str::FromStr;
use crate::version_number::VersionNumber;
use serde_derive::{Deserialize,Serialize};
use std::hash::Hash;

/// Package implements Versionable trait. A VersionNumber may be comprised of one or more u16 digits
#[derive(PartialEq, PartialOrd, Eq, Ord, Deserialize, Serialize,Hash)]
pub struct Package {
    pub name: String,
    version: VersionNumber,
}

impl fmt::Debug for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.name, self.version.to_string())
    }
}

impl Package {
    /// extract the package name as a &str
    pub fn package(&self) -> &str {
        self.name.as_str()
    }

    pub fn name(&self) -> String {
        format!("{}-{}", self.name, self.version.to_string())
    }

    /// Construct a Package from a vector of u16
    pub fn new<T: Into<String>>(name: T, version: VersionNumber) -> Self {
        Self {
            name: name.into(),
            version
        }
    }

    /// construct a Package with 3 u16 values
    pub fn semver(name: &str, major: u16, minor: u16, micro: u16) -> Self {
        let value =  VersionNumber::new(vec![major, minor, micro]);
        Self::new(name, value)
    }

    /// construct a semver4 from a value
    pub fn semver4(name: &str, major: u16, minor: u16, micro: u16, patch: u16) -> Self {
        let value = VersionNumber::new(vec![major, minor, micro, patch]);
        Self::new(name, value)
    }

    /// Not the FromString trait because of lifetime requirements
    pub fn  from_string(s: &str) -> Result<Self, crate::errors::VersionitisError> {
        // todo support variants
        let pieces: Vec<&str> = s.split("-").collect();
        let mut result: Vec<u16> = Vec::new();
        for x in pieces[1].split(".").map(|x| x.parse::<u16>()) {
            let x = x?;
            result.push(x);
        }

        Ok( Package::new(pieces[0], VersionNumber::new(result)))
    }


    /// Not the FromString trait because of lifetime requirements
    pub fn  from_strs(name: &str, version: &str) -> Result<Self, crate::errors::VersionitisError> {
        // todo support variants
        let mut result: Vec<u16> = Vec::new();
        for x in version.split(".").map(|x| x.parse::<u16>()) {
            let x = x?;
            result.push(x);
        }

        Ok( Package::new(name, VersionNumber::new(result)))
    }
}

impl ToString for Package {
    fn to_string(&self) -> String {
       self.name()
    }
}

#[macro_export]
macro_rules! version {
    ($e:expr) => {
        Package::from_string(
        stringify!($e).chars().filter(|x| *x != ' ').collect::<String>().as_str()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macrotest() {
        let sv1 = version!( foo-0.1.0 ) ;
        let sv2 = Package::from_string("foo-0.1.0");
        assert_eq!(sv1, sv2);
    }

    #[test]
    fn base() {
        let package = String::from("fred-0.1.0.1");
        let sv1 = Package::from_string(&package).unwrap();
        assert_eq!(sv1.package(), "fred");
    }

    #[test]
    fn simple_new() {
        let name = String::from("fred");
        let sv1 = Package::semver(&name, 0,1,0);
        let sv2 = Package::new(name.as_str(), VersionNumber::new(vec![0,1,0]) );
        assert_eq!(sv1, sv2);
    }

    #[test]
    fn simple_equality() {
        let name = String::from("fred");
        let sv1 = Package::semver(&name, 0,1,0);
        let sv2 = Package::semver(&name, 0,1,0);
        assert_eq!(sv1, sv2);
    }

    #[test]
    fn simple_inequality_lt() {
        let name = String::from("fred");
        let sv1 = Package::semver(&name, 0,0,1);
        let sv2 = Package::semver(&name, 0,1,0);
        assert!(sv1 < sv2);
    }

    #[test]
    fn simple_inequality_gt() {
        let name = String::from("fred");
        let sv1 = Package::semver(&name, 1,0,1);
        let sv2 = Package::semver(&name, 0,1,0);
        assert!(sv1 > sv2);
    }

    #[test]
    fn complex_inequality_lt() {
        let name = String::from("fred");
        let sv1 = Package::semver(&name, 0,1,0);
        let sv2 = Package::semver4(&name, 0,1,0,0);
        assert!(sv1 < sv2);
    }

    #[test]
    fn complex_inequality_lt2() {
        let name = String::from("fred");
        let sv1 = Package::semver(&name, 0,1,0);
        let sv2 = Package::semver4(&name, 0,1,0,1);
        assert!(sv1 < sv2);
    }

    #[test]
    fn complex_inequality_gt() {
        let name = String::from("fred");
        let sv1 = Package::semver(&name,0,1,1);
        let sv2 = Package::semver4(&name, 0,1,0,1);
        assert!(sv1 > sv2);
    }

    #[test]
    fn version() {
        let name = String::from("fred");
        let sv2 = Package::semver4(&name, 0,1,0,1);
        assert_eq!(sv2.to_string().as_str(), "fred-0.1.0.1" );
    }

    #[test]
    fn to_str() {
        let name = String::from("fred");
        let package = String::from("fred-0.1.0.1");
        let sv = Package::semver4(&name, 0,1,0,1);
        let result = sv.to_string();
        assert_eq!(result, package );
    }

    #[test]
    fn debug() {
        let name = String::from("fred");
        let package = String::from("fred-0.1.0.1");
        let sv = Package::semver4(&name, 0,1,0,1);
        let result = format!("{:?}", sv);
        assert_eq!(result, package );
    }

    #[test]
    fn from_str() {
        let name = String::from("fred");
        let package = String::from("fred-0.1.0.1");
        let sv1 = Package::from_string(&package).unwrap();
        let sv2 = Package::semver4(&name, 0,1,0,1);
        assert_eq!(sv1, sv2 );
    }
}
