use core::str::FromStr;

/// VersionNumber implements Versionable trait. A VersionNumber may be comprised of one or more u16 digits
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct VersionNumber {
    value: Vec<u16>,
}

impl VersionNumber {
    fn construct_name(value: &Vec<u16>) -> String {
        value.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(".")
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
}

impl ToString for VersionNumber {
    fn to_string(&self) -> String {
        VersionNumber::construct_name(&self.value)
    }
}

impl FromStr for VersionNumber {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result: Vec<u16> = Vec::new();
        for x in s.split(".").map(|x| x.parse::<u16>()) {
            let x = x?;
            result.push(x);
        }

        Ok( VersionNumber::new(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_equality() {
        let sv1 = VersionNumber::semver(0,1,0);
        let sv2 = VersionNumber::semver(0,1,0);
        assert_eq!(sv1, sv2);
    }

    #[test]
    fn simple_inequality_lt() {
        let sv1 = VersionNumber::semver(0,0,1);
        let sv2 = VersionNumber::semver(0,1,0);
        assert!(sv1 < sv2);
    }

    #[test]
    fn simple_inequality_gt() {
        let sv1 = VersionNumber::semver(1,0,1);
        let sv2 = VersionNumber::semver(0,1,0);
        assert!(sv1 > sv2);
    }

    #[test]
    fn complex_inequality_lt() {
        let sv1 = VersionNumber::semver(0,1,0);
        let sv2 = VersionNumber::semver4(0,1,0,0);
        assert!(sv1 < sv2);
    }

    #[test]
    fn complex_inequality_lt2() {
        let sv1 = VersionNumber::semver(0,1,0);
        let sv2 = VersionNumber::semver4(0,1,0,1);
        assert!(sv1 < sv2);
    }

    #[test]
    fn complex_inequality_gt() {
        let sv1 = VersionNumber::semver(0,1,1);
        let sv2 = VersionNumber::semver4(0,1,0,1);
        assert!(sv1 > sv2);
    }

    #[test]
    fn version() {
        let sv2 = VersionNumber::semver4(0,1,0,1);
        assert_eq!(sv2.to_string().as_str(), "0.1.0.1" );
    }

    #[test]
    fn from_str() {
        let sv1 = VersionNumber::from_str("0.1.0.1").unwrap();
        let sv2 = VersionNumber::semver4(0,1,0,1);
        assert_eq!(sv1, sv2 );
    }
}
