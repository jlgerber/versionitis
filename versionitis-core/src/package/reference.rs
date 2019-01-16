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
    /// Construct a Package from a vector of u16
    pub fn new<'b: 'a>(name: &'b str, input: Vec<u16>) -> Self {
        Self {
            name,
            value: input
        }
    }

    pub fn name(&self) -> &str {
        self.name.split("-").collect::<Vec<&str>>()[0]
    }

    pub fn package(&self) -> &str {
        self.name
    }

    /// Not the FromString trait because of lifetime requirements
    pub fn  from_str<'b: 'a>(s: &'b str) -> Result<Self, VersionitisError> {
        // todo support variants
        let pieces: Vec<&'b str> = s.split("-").collect();
        let mut result: Vec<u16> = Vec::new();
        for x in pieces[1].split(".").map(|x| x.parse::<u16>()) {
            let x = x?;
            result.push(x);
        }

        Ok(Package::new(s, result))
    }
}

impl<'a> ToString for Package<'a> {
    fn to_string(&self) -> String {
        self.name.to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_equality() {
        let sv1 = Package::from_str("fred-0.1.0").unwrap();
        let sv2 = Package::from_str("fred-0.1.0").unwrap();
        assert_eq!(sv1, sv2);
    }

    #[test]
    fn simple_inequality_lt() {
        let sv1 = Package::from_str("fred-0.1.0").unwrap();
        let sv2 = Package::from_str("fred-0.1.1").unwrap();
        assert!(sv1 < sv2);
    }

    #[test]
    fn simple_inequality_gt() {
        let sv1 = Package::from_str("fred-1.1.0").unwrap();
        let sv2 = Package::from_str("fred-0.1.0").unwrap();
        assert!(sv1 > sv2);
    }

    #[test]
    fn complex_inequality_lt() {
        let sv1 = Package::from_str("fred-0.1.0").unwrap();
        let sv2 = Package::from_str("fred-0.1.0.0").unwrap();
        assert!(sv1 < sv2);
    }

    #[test]
    fn complex_inequality_lt2() {
        let sv1 = Package::from_str("fred-0.1.0").unwrap();
        let sv2 = Package::from_str("fred-0.1.0.1").unwrap();
        assert!(sv1 < sv2);
    }

    #[test]
    fn complex_inequality_gt() {
        let sv1 = Package::from_str("fred-0.1.1").unwrap();
        let sv2 = Package::from_str("fred-0.1.0.1").unwrap();
        assert!(sv1 > sv2);
    }

    #[test]
    fn version() {
        let sv2 = Package::from_str("fred-0.1.0.1").unwrap();
        assert_eq!(sv2.to_string().as_str(), "fred-0.1.0.1" );
    }

    #[test]
    fn debug() {
        let package = String::from("fred-0.1.0.1");
        let sv = Package::from_str("fred-0.1.0.1").unwrap();
        let result = format!("{:?}", sv);
        assert_eq!(result, package );
    }

}
