//! package_map
//!
//! map package versions to 1 based id, for use with SAT solver.
//!
use crate::package::owned::Package;
use std::collections::HashMap;

type IdxType = i32;
type PMap = HashMap<String, IdxType>;

/// Store packages in a SAT friendly structure
pub struct PackageMap {
    arena: Vec<Package>,
    map: PMap
}

impl PackageMap {
    /// New up an empty PackageMap
    pub fn new() -> Self {
        Self {
            arena: Vec::new(),
            map: PMap::new(),
        }
    }

    /// add a Package to the map if it doesnt exist
    pub fn add(&mut self, version: Package)  {
        let name = version.name().to_string();
        if !self.has(name.as_str()) {
            self.arena.push(version);
            self.map.insert(name, self.arena.len() as IdxType);
        }
    }

    /// Given a package version str, determine whether the package map
    /// contains the package version.
    pub fn has(&self, version_str: &str) -> bool {
        self.map.contains_key(version_str)
    }

    /// given a &str representing a valid package name, create a Package
    /// and add it into the PackageMap
    pub fn add_str(&mut self, vs: &str) {
        // todo: deal with error
        let version_num = Package::from_string(vs).unwrap();
        self.add(version_num);
    }

    /// Retrieve an Option wrapping a reference to an IdxType
    pub fn get(&self, value: &str) -> Option<IdxType> {
        match self.map.get(value) {
            Some(value) => Some(*value),
            None => None
        }
    }

    /// Retrueve the Package associated with a particular literal. The literal
    /// is a positive integer (ie it is stored in 1-based list to be compatible with
    /// SAT solver semantics )
    pub fn at_lit(&self, lit: IdxType) -> Option<&Package> {
        self.arena.get( (lit - 1) as usize)
    }

    /// Return an option wrapped mutable reference to a Package
    pub fn at_lit_mut(&mut self, lit: IdxType) -> Option<&mut Package> {
        self.arena.get_mut( (lit - 1 )as usize)
    }

    /// Retrieve the number of elements stored
    pub fn len(&self) -> usize {
        self.map.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testone() {
        let mut mymap = PackageMap::new();
        mymap.add(Package::from_string("foo-0.1.0").unwrap());
        mymap.add_str("foo-0.2.0");
        mymap.add_str("foo-0.2.1");

        assert_eq!(mymap.len(), 3);
        assert_eq!(mymap.get("foo-0.1.0"), Some(1));
        assert_eq!(mymap.get("foo-0.2.0"), Some(2));
        assert_eq!(mymap.get("foo-0.2.1"), Some(3));
        assert_eq!(mymap.get("foo-bar"), None);
    }

    #[test]
    fn can_add_multiple_times() {
        let mut mymap = PackageMap::new();
        mymap.add(Package::from_string("foo-0.1.0").unwrap());
        mymap.add_str("foo-0.2.0");
        mymap.add_str("foo-0.2.1");
        let idx = mymap.get("foo-0.2.1");
        mymap.add_str("foo-0.2.1");
        let idx_after = mymap.get("foo-0.2.1");
        assert_eq!(mymap.len(), 3);
        assert_eq!(idx, idx_after);
    }

}