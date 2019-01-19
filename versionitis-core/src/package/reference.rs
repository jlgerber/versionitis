//! NOT IN USE CURRENTLY
//! PARTIALLY PORTED FROM OLD VERSION
use std::fmt;
use crate::errors::VersionitisError;

/// Package implements Versionable trait. A Package may be comprised of one or more u16 digits
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct Package<'a> {
    name: &'a str,
    value: Vec<u16>,
}
impl<'a> fmt::Debug for Package<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.to_string();
        write!(f, "{}", name)
    }
}

impl<'a> Package<'a> {
    fn construct_name(value: &Vec<u16>) -> String {

        value.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(".")
    }

    /// Construct a Package from a vector of u16
    pub fn new<'b: 'a>(name: &'b str, input: Vec<u16>) -> Self {
        Self {
            name,
            value: input
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn spec(&self) -> String {
        self.to_string()
    }
    /// construct a Package with 3 u16 values
    pub fn semver<'b: 'a>(name: &'b str, major: u16, minor: u16, micro: u16) -> Self {
        let value = vec![major, minor, micro];
        Self::new(name, value)
    }

    /// construct a semver4 from a value
    pub fn semver4<'b: 'a>(name: &'b str, major: u16, minor: u16, micro: u16, patch: u16) -> Self {
        let value = vec![major, minor, micro, patch];
        Self::new(name, value)
    }

    /// Not the FromString trait because of lifetime requirements
    pub fn  from_string<'b: 'a>(s: &'b str) -> Result<Self, VersionitisError> {
        // todo support variants
        let pieces: Vec<&'b str> = s.split("-").collect();
        let mut result: Vec<u16> = Vec::new();
        for x in pieces[1].split(".").map(|x| x.parse::<u16>()) {
            let x = x?;
            result.push(x);
        }

        Ok( Package::new(pieces[0], result))
    }
}

impl<'a> ToString for Package<'a> {
    fn to_string(&self) -> String {
        let version = Package::construct_name(&self.value);
        format!("{}-{}", self.name, version)
    }
}

// cannot use because we need a lifetime which exceeds 'a
// impl<'a> FromStr for Package<'a> {
//     type Err = std::num::ParseIntError;

//     fn from_str<'b>(s: &'b str) -> Result<Self, Self::Err> {

//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
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
