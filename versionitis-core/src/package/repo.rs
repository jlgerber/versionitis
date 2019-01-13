use std::collections::HashMap;
use crate::package::owned::Package;
use std::iter::Iterator;

type PackageMap = HashMap<String, Vec<Package>>;

pub struct Repo {
    packages: PackageMap,
}

impl Repo {
    pub fn new() -> Self {
        Self {
            packages: PackageMap::new()
        }
    }

    pub fn add_version(&mut self, package_name: &str, version: &str)
    -> Result<(), std::num::ParseIntError> {
        let pack = Package::from_strs(package_name, version)?;
        match self.packages.get_mut(package_name) {
            Some(ref mut lst) => {
                lst.push(pack);
                Ok(())
            },
            _ => {
                // create package
                self.packages.insert(package_name.to_string(), vec![pack]);
                Ok(())
            },
        }
    }

    pub fn get<'a>(&'a self, package: &str) -> Result<&'a Vec<Package>, String> {
        match self.packages.get(package) {
            Some(ref pv) => Ok(pv),
            None => Err("unable to get package".to_string())
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let mut repo = Repo::new();
        let package_version = Package::from_strs("foo", "0.1.0");

        repo.add_version("foo", "0.1.0");
        let package = repo.get("foo");
        assert_eq!(package.unwrap()[0], package_version.unwrap());
    }
}