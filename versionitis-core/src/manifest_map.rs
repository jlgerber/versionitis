//! package_map
//!
//! map manifest versions to 1 based id, for use with SAT solver.
//!
use crate::manifest::Manifest;

use std::collections::{HashMap, hash_map::Keys};
use typed_arena::Arena;
use std::collections::HashSet;

pub type PackageName = str;
pub type ManifestArena = Arena<Manifest>;
pub type _ManifestMap<'a> = HashMap<&'a PackageName, &'a Manifest>;

pub struct ManifestMap<'a, 'b: 'a> {
    arena: &'b ManifestArena,
    map: _ManifestMap<'a>,
}

impl<'a, 'b> ManifestMap<'a, 'b> {
    /// New up an empty ManifestMap
    pub fn new(arena: &'b ManifestArena) -> Self {
        Self {
            arena,
            map: _ManifestMap::new(),
        }
    }

    /// Add a manifest into the manifest_map
    pub fn add(&mut self, manifest: Manifest) {
        let manifest: &'b Manifest = self.arena.alloc(manifest);
        let key = manifest.package();
        self.map.insert(key, manifest);
    }

    /// Retrieve an option wrapped Manifest reference given a package name.
    pub fn get(&self, name: &str) -> Option<&'a Manifest> {
        match self.map.get(name) {
            Some(map) => Some(*map),
            None => None
        }
    }

    /// Retrieve an iterator over keys
    pub fn keys(&self) -> Keys<&'a PackageName, &'a Manifest> {
        self.map.keys()
    }

    /// retrieve a hashset of packages
    pub fn packages(&self) -> HashSet<&'a str> {
        let mut hashset: HashSet<&'a str> = HashSet::new();
        for key in self.keys() {
            let retstr = key.split("-").next().unwrap();
            hashset.insert(retstr);
        }
        hashset
    }

     /// retrieve a hashset of packages
    pub fn packages_sorted(&self) -> Vec<&'a str> {
        let mut hashset: HashSet<&'a str> = HashSet::new();
        for key in self.keys() {
            let retstr = key.split("-").next().unwrap();
            hashset.insert(retstr);
        }
        let mut packages = hashset.iter().map(|x| *x).collect::<Vec<& str>>();
        packages.sort();
        packages
    }
    /// Given a package name, determine whether the manifest map
    /// contains the manifest version or not.
    pub fn has(&self, package: &str) -> bool {
        self.map.contains_key(package)
    }

    /// Given a &str representing a valid manifest name, create a Manifest
    /// and add it into the ManifestMap
    pub fn add_str(&mut self, vs: &str) {
        // todo: deal with error
        let version_num = Manifest::new(vs);
        self.add(version_num);
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
    fn can_add_manifests_into_map() {
        let arena = ManifestArena::new();
        let mut mymap = ManifestMap::new(&arena);
        mymap.add(Manifest::new("foo-0.1.0"));
        mymap.add_str("foo-0.2.0");
        mymap.add_str("foo-0.2.1");

        assert_eq!(mymap.len(), 3);
        assert_eq!(mymap.get("foo-0.1.0"), Some(&Manifest::new("foo-0.1.0")));
        assert_eq!(mymap.get("foo-0.2.0"), Some(&Manifest::new("foo-0.2.0")));
        assert_eq!(mymap.get("foo-0.2.1"), Some(&Manifest::new("foo-0.2.1")));
        assert_eq!(mymap.get("foo-bar"), None);
    }


    #[test]
    fn can_get_package_hashset() {
        let arena = ManifestArena::new();
        let mut mymap = ManifestMap::new(&arena);
        mymap.add(Manifest::new("foo-0.1.0"));
        mymap.add_str("foo-0.2.0");
        mymap.add_str("foo-0.2.1");
        mymap.add_str("bar-0.2.0");
        mymap.add_str("bar-0.2.1");

        let packages = mymap.packages();
        let mut vpackages = packages.iter().map(|x| *x).collect::<Vec<& str>>();
        vpackages.sort();
        let val = vec!["bar", "foo"];
        assert_eq!(vpackages, val);
        assert_eq!(packages.len(), 2);
    }

    #[test]
    fn can_get_ordered_packages() {
        let arena = ManifestArena::new();
        let mut mymap = ManifestMap::new(&arena);
        mymap.add(Manifest::new("foo-0.1.0"));
        mymap.add_str("foo-0.2.0");
        mymap.add_str("foo-0.2.1");
        mymap.add_str("bar-0.2.0");
        mymap.add_str("bar-0.2.1");

        let packages = mymap.packages_sorted();

        let val = vec!["bar", "foo"];
        assert_eq!(packages, val);
    }


    #[test]
    fn can_add_multiple_times() {
        let arena = ManifestArena::new();
        let mut mymap = ManifestMap::new(&arena);
        mymap.add(Manifest::new("foo-0.1.0"));
        mymap.add_str("foo-0.2.0");
        mymap.add_str("foo-0.2.1");
        let idx = mymap.get("foo-0.2.1");
        mymap.add_str("foo-0.2.1");
        let idx_after = mymap.get("foo-0.2.1");
        assert_eq!(mymap.len(), 3);
        assert_eq!(idx, idx_after);
    }

}
