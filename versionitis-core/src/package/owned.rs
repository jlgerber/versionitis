//! owned.rs
//!
//! Owned Package implementation. In the owned implementation
//! the Package owns its fields (eg String instead of &str)

use crate::version_number::VersionNumber;
use serde::{
    de::{self, Deserializer, Visitor},
    ser::{Serialize, Serializer},
    Deserialize,
};
use std::fmt;


/// A named entity which is ordered, convertable to and from a
/// string, hashable, and may of course be debuged.
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct Package {
    name: String,
    version: VersionNumber,
}

impl Serialize for Package {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // let mut state = serializer.serialize_struct("Package", 1)?;
        // let value = format!("{}-{}", self.name, self.version);
        // state.serialize_field("spec", &value)?;
        // state.end()
        let value = format!("{}-{}", self.name, self.version);
        let result = serializer.serialize_newtype_struct("Package",&value);
        result
        //state.serialize_field("spec", &value)?;
    }
}
// PackageVisitor used for serde deserialization
struct PackageVisitor;
// Visitor implemented as part of custom serde pass
impl<'de> Visitor<'de> for PackageVisitor {
    type Value = Package;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a str of the form name-version (eg fred-0.1.0)")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match Package::from_str(value) {
            Ok(v) => Ok(v),
            Err(e) => panic!("unable to deserialize: {}", e),
        }
    }
}

impl<'de> Deserialize<'de> for Package {
    fn deserialize<D>(deserializer: D) -> Result<Package, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(PackageVisitor)
    }
}

impl fmt::Debug for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.name, self.version)
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.name, self.version)
    }
}

impl Package {
    /// Extract the package name as a &str
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Get the full specification for a package, in the form of "name-version",
    /// as a String
    pub fn spec(&self) -> String {
        format!("{}-{}", self.name, self.version)
    }

    /// Get the version as a &str from the package
    pub fn version(&self) -> String {
        self.version.to_string()
    }
    pub fn version_number(&self) -> &VersionNumber {
        &self.version
    }

    /// Construct a Package from a name and a VersionNumber instance.
    pub fn new<T: Into<String>>(name: T, version: VersionNumber) -> Self {
        Self {
            name: name.into(),
            version,
        }
    }

    /// Construct a Package from a package name and three u16 values,
    /// following the semver spec.
    pub fn semver(name: &str, major: u16, minor: u16, micro: u16) -> Self {
        let value = VersionNumber::new(vec![major, minor, micro]);
        Self::new(name, value)
    }

    /// Construct a Package from a package name, and four u16 values, following
    /// the semver spec, plus a patch version to allow for context and manifest changes.
    pub fn semver4(name: &str, major: u16, minor: u16, micro: u16, patch: u16) -> Self {
        let value = VersionNumber::new(vec![major, minor, micro, patch]);
        Self::new(name, value)
    }

    /// Not the FromString trait because of lifetime requirements
    pub fn from_str(s: &str) -> Result<Self, crate::errors::VersionitisError> {
        // todo support variants
        let pieces: Vec<&str> = s.split("-").collect();
        let mut result: Vec<u16> = Vec::new();
        for x in pieces[1].split(".").map(|x| x.parse::<u16>()) {
            let x = x?;
            result.push(x);
        }

        Ok(Package::new(pieces[0], VersionNumber::new(result)))
    }

    /// Not the FromString trait because of lifetime requirements
    pub fn from_strs(name: &str, version: &str) -> Result<Self, crate::errors::VersionitisError> {
        // todo support variants
        let mut result: Vec<u16> = Vec::new();
        for x in version.split(".").map(|x| x.parse::<u16>()) {
            let x = x?;
            result.push(x);
        }

        Ok(Package::new(name, VersionNumber::new(result)))
    }
}

#[macro_export]
macro_rules! version {
    ($e:expr) => {
        Package::from_str(
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
    fn package_implements_display_trait() {
        let p = Package::from_str("foo-0.1.0").unwrap();
        let pd = format!("{}", p);
        assert_eq!(pd, "foo-0.1.0".to_string());
    }

    #[test]
    fn can_create_package_from_name_with_spaces_using_verison_macro() {
        let sv1 = version!(foo - 0.1.0);
        let sv2 = Package::from_str("foo-0.1.0");
        assert_eq!(sv1, sv2);
    }

    #[test]
    fn can_create_package_using_from_str_function() {
        let package = String::from("fred-0.1.0.1");
        let sv1 = Package::from_str(&package).unwrap();
        assert_eq!(sv1.name(), "fred");
    }

    #[test]
    fn can_create_package_using_new_function_given_name_str_and_versionnumber() {
        let name = String::from("fred");
        let sv1 = Package::semver(&name, 0, 1, 0);
        let sv2 = Package::new(name.as_str(), VersionNumber::new(vec![0, 1, 0]));
        assert_eq!(sv1, sv2);
    }

    #[test]
    fn packages_with_same_name_and_version_are_equal() {
        let name = String::from("fred");
        let sv1 = Package::semver(&name, 0, 1, 0);
        let sv2 = Package::semver(&name, 0, 1, 0);
        assert_eq!(sv1, sv2);
    }

    #[test]
    fn package_with_same_name_and_small_version_less_than_other_package() {
        let name = String::from("fred");
        let sv1 = Package::semver(&name, 0, 0, 1);
        let sv2 = Package::semver(&name, 0, 1, 0);
        assert!(sv1 < sv2);
    }

    #[test]
    fn package_with_higher_version_greater_than_package_with_smaller_version() {
        let name = String::from("fred");
        let sv1 = Package::semver(&name, 1, 0, 1);
        let sv2 = Package::semver(&name, 0, 1, 0);
        assert!(sv1 > sv2);
    }

    #[test]
    fn package_with_more_digits_in_version_ord_test_but_zero() {
        let name = String::from("fred");
        let sv1 = Package::semver(&name, 0, 1, 0);
        let sv2 = Package::semver4(&name, 0, 1, 0, 0);
        assert!(sv1 < sv2);
    }

    #[test]
    fn package_with_more_digits_in_version_ord_test_more_digits_gt() {
        let name = String::from("fred");
        let sv1 = Package::semver(&name, 0, 1, 0);
        let sv2 = Package::semver4(&name, 0, 1, 0, 1);
        assert!(sv1 < sv2);
    }

    #[test]
    fn package_with_more_digits_in_version_ord_test_2() {
        let name = String::from("fred");
        let sv1 = Package::semver(&name, 0, 1, 1);
        let sv2 = Package::semver4(&name, 0, 1, 0, 1);
        assert!(sv1 > sv2);
    }

    #[test]
    fn can_convert_to_string() {
        let name = String::from("fred");
        let sv2 = Package::semver4(&name, 0, 1, 0, 1);
        assert_eq!(sv2.to_string().as_str(), "fred-0.1.0.1");
    }

    #[test]
    fn pakage_implements_debug_trait() {
        let name = String::from("fred");
        let package = String::from("fred-0.1.0.1");
        let sv = Package::semver4(&name, 0, 1, 0, 1);
        let result = format!("{:?}", sv);
        assert_eq!(result, package);
    }

    #[test]
    fn can_convert_from_str() {
        let name = String::from("fred");
        let package = String::from("fred-0.1.0.1");
        let sv1 = Package::from_str(&package).unwrap();
        let sv2 = Package::semver4(&name, 0, 1, 0, 1);
        assert_eq!(sv1, sv2);
    }

    const YAML_PKG: &'static str = "---\nfred-0.1.2";
    #[test]
    fn can_serialize_to_yaml() {
        let package = Package::from_str("fred-0.1.2").unwrap();
        let yaml = serde_yaml::to_string(&package).unwrap();
        assert_eq!(yaml, YAML_PKG);
    }

    #[test]
    fn can_deserialize_from_yaml() {
        let package: serde_yaml::Result<Package> = serde_yaml::from_str("fred-0.1.2");
        assert!(package.is_ok());
    }
}
