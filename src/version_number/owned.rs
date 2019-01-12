use std::fmt;
//use core::str::FromStr;

/// VersionNumber implements Versionable trait. A VersionNumber may be comprised of one or more u16 digits
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct VersionNumber {
    pub name: String,
    index: u8,
    value: Vec<u16>,
}
impl fmt::Debug for VersionNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.to_string();
        write!(f, "{}", name)
    }
}

impl VersionNumber {
    /// extract the package name as a &str
    pub fn package(&self) -> &str {
        self.name.as_str().split_at(self.index as usize).0
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    fn construct_name(name: &str, value: &Vec<u16>) -> String {
        let version = value.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(".");
        format!("{}-{}", name, version)
    }

    /// Construct a VersionNumber from a vector of u16
    pub fn new(name: &str, input: Vec<u16>) -> Self {
        let fullname = VersionNumber::construct_name(name, &input);
        Self {
            name: fullname,
            index: name.len() as u8,
            value: input
        }
    }

    /// construct a VersionNumber with 3 u16 values
    pub fn semver(name: &str, major: u16, minor: u16, micro: u16) -> Self {
        let value = vec![major, minor, micro];
        Self::new(name, value)
    }

    /// construct a semver4 from a value
    pub fn semver4(name: &str, major: u16, minor: u16, micro: u16, patch: u16) -> Self {
        let value = vec![major, minor, micro, patch];
        Self::new(name, value)
    }

    /// Not the FromString trait because of lifetime requirements
    pub fn  from_string(s: &str) -> Result<Self, std::num::ParseIntError> {
        // todo support variants
        let pieces: Vec<&str> = s.split("-").collect();
        let mut result: Vec<u16> = Vec::new();
        for x in pieces[1].split(".").map(|x| x.parse::<u16>()) {
            let x = x?;
            result.push(x);
        }

        Ok( VersionNumber::new(pieces[0], result))
    }
}

impl ToString for VersionNumber {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

#[macro_export]
macro_rules! version {
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
        let sv1 = version!( foo-0.1.0 ) ;
        let sv2 = VersionNumber::from_string("foo-0.1.0");
        assert_eq!(sv1, sv2);
    }

    #[test]
    fn base() {
        let package = String::from("fred-0.1.0.1");
        let sv1 = VersionNumber::from_string(&package).unwrap();
        assert_eq!(sv1.package(), "fred");
    }

    #[test]
    fn simple_equality() {
        let name = String::from("fred");
        let sv1 = VersionNumber::semver(&name, 0,1,0);
        let sv2 = VersionNumber::semver(&name, 0,1,0);
        assert_eq!(sv1, sv2);
    }

    #[test]
    fn simple_inequality_lt() {
        let name = String::from("fred");
        let sv1 = VersionNumber::semver(&name, 0,0,1);
        let sv2 = VersionNumber::semver(&name, 0,1,0);
        assert!(sv1 < sv2);
    }

    #[test]
    fn simple_inequality_gt() {
        let name = String::from("fred");
        let sv1 = VersionNumber::semver(&name, 1,0,1);
        let sv2 = VersionNumber::semver(&name, 0,1,0);
        assert!(sv1 > sv2);
    }

    #[test]
    fn complex_inequality_lt() {
        let name = String::from("fred");
        let sv1 = VersionNumber::semver(&name, 0,1,0);
        let sv2 = VersionNumber::semver4(&name, 0,1,0,0);
        assert!(sv1 < sv2);
    }

    #[test]
    fn complex_inequality_lt2() {
        let name = String::from("fred");
        let sv1 = VersionNumber::semver(&name, 0,1,0);
        let sv2 = VersionNumber::semver4(&name, 0,1,0,1);
        assert!(sv1 < sv2);
    }

    #[test]
    fn complex_inequality_gt() {
        let name = String::from("fred");
        let sv1 = VersionNumber::semver(&name,0,1,1);
        let sv2 = VersionNumber::semver4(&name, 0,1,0,1);
        assert!(sv1 > sv2);
    }

    #[test]
    fn version() {
        let name = String::from("fred");
        let sv2 = VersionNumber::semver4(&name, 0,1,0,1);
        assert_eq!(sv2.to_string().as_str(), "fred-0.1.0.1" );
    }

    #[test]
    fn to_str() {
        let name = String::from("fred");
        let package = String::from("fred-0.1.0.1");
        let sv = VersionNumber::semver4(&name, 0,1,0,1);
        let result = sv.to_string();
        assert_eq!(result, package );
    }

    #[test]
    fn debug() {
        let name = String::from("fred");
        let package = String::from("fred-0.1.0.1");
        let sv = VersionNumber::semver4(&name, 0,1,0,1);
        let result = format!("{:?}", sv);
        assert_eq!(result, package );
    }

    #[test]
    fn from_str() {
        let name = String::from("fred");
        let package = String::from("fred-0.1.0.1");
        let sv1 = VersionNumber::from_string(&package).unwrap();
        let sv2 = VersionNumber::semver4(&name, 0,1,0,1);
        assert_eq!(sv1, sv2 );
    }
}
