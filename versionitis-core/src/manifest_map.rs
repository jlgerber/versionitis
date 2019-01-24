//! package_map
//!
//! map manifest versions to 1 based id, for use with SAT solver.
//!
use crate::manifest::Manifest;

use std::collections::HashMap;
use typed_arena::Arena;

pub type PackageName = str;
pub type ManifestArena = Arena<Manifest>;
pub type _ManifestMap<'a> = HashMap<&'a PackageName, &'a Manifest>;

pub struct ManifestMap<'a, 'b: 'a> {
    arena: &'b ManifestArena,
    map: _ManifestMap<'a>,
}

//impl<'a, 'b> PackMap<'a, 'b> {
/*
type IdxType = i32;
type PMap = HashMap<String, IdxType>;

/// Store packages in a SAT friendly structure
pub struct ManifestMap {
    arena: Vec<Manifest>,
    map: PMap,
}
*/

impl<'a, 'b> ManifestMap<'a, 'b> {
    /// New up an empty ManifestMap
    pub fn new(arena: &'b ManifestArena) -> Self {
        Self {
            arena,
            map: _ManifestMap::new(),
        }
    }


    pub fn add(&mut self, manifest: Manifest) {
        let manifest: &'b Manifest = self.arena.alloc(manifest);
        let key = manifest.package();
        self.map.insert(key, manifest);
    }

    pub fn get(&self, name: &str) -> Option<&'a Manifest> {
        match self.map.get(name) {
            Some(map) => Some(*map),
            None => None
        }
    }

    /// Given a manifest version str, determine whether the manifest map
    /// contains the manifest version.
    pub fn has(&self, version_str: &str) -> bool {
        self.map.contains_key(version_str)
    }

    /// given a &str representing a valid manifest name, create a Manifest
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
