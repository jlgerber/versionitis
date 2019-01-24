//! manifest_repo.rs
//!
//! create a struct which owns package manifests.
//!
use crate::manifest::Manifest;
use crate::errors::VersionitisError;
use std::collections::{HashMap, hash_map::Keys};
use typed_arena::Arena;
use std::collections::HashSet;
use std::path::PathBuf;
use std::fs;

pub type PackageName = str;
pub type ManifestArena = Arena<Manifest>;
pub type _ManifestMap<'a> = HashMap<&'a PackageName, &'a Manifest>;

pub struct ManifestRepo<'a, 'b: 'a> {
    arena: &'b ManifestArena,
    map: _ManifestMap<'a>,
}

impl<'a, 'b> std::fmt::Debug for ManifestRepo<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Manifest Repo\n{:?}", self.map)
    }
}

impl<'a, 'b> ManifestRepo<'a, 'b> {
    /// New up an empty ManifestRepo
    pub fn new(arena: &'b ManifestArena) -> Self {
        Self {
            arena,
            map: _ManifestMap::new(),
        }
    }

    /// construct a ManifestRepo from a directory full of manifests
    pub fn from_disk<P: Into<PathBuf>>(path: P, arena: &'b ManifestArena) -> Result<Self, VersionitisError> {
        // get path to directory
        let path = path.into();
        if !path.is_dir() {
            return Err(VersionitisError::IoError(format!("path: {:?} does not exist", path)));
        }

        let mut repo = ManifestRepo::new(arena);

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let file = fs::read_to_string(path)?;
                let manifest = serde_yaml::from_str(file.as_str())?;
                repo.add(manifest);
            }
        }

        Ok(repo)
    }

    /// Add a manifest into the manifest_repo
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

    /// retrieve a hashset of packages. If versioned is true, we get
    /// all versioned packages. Otherwise, we simply get the set of
    /// package basenames (eg foo as opposed to foo-0.1.0)
    pub fn packages(&self, versioned: bool) -> HashSet<&'a str> {
        let mut hashset: HashSet<&'a str> = HashSet::new();
        if versioned == true {
            for key in self.keys() {
                hashset.insert(key);
            }
        } else {
            for key in self.keys() {
                let retstr = key.split("-").next().unwrap();
                hashset.insert(retstr);
            }
        }
        hashset
    }

     /// retrieve a hashset of packages. If versioned is true, we get all of the
     /// package versions. Otherwise, we get the unique set of package names, sorted.
    pub fn packages_sorted(&self, versioned: bool) -> Vec<&'a str> {
        let hashset: HashSet<&'a str> = self.packages(versioned);
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
    /// and add it into the ManifestRepo
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
        let mut repo = ManifestRepo::new(&arena);
        repo.add(Manifest::new("foo-0.1.0"));
        repo.add_str("foo-0.2.0");
        repo.add_str("foo-0.2.1");

        assert_eq!(repo.len(), 3);
        assert_eq!(repo.get("foo-0.1.0"), Some(&Manifest::new("foo-0.1.0")));
        assert_eq!(repo.get("foo-0.2.0"), Some(&Manifest::new("foo-0.2.0")));
        assert_eq!(repo.get("foo-0.2.1"), Some(&Manifest::new("foo-0.2.1")));
        assert_eq!(repo.get("foo-bar"), None);
    }


    #[test]
    fn can_get_package_hashset_without_versions() {
        let arena = ManifestArena::new();
        let mut repo = ManifestRepo::new(&arena);
        repo.add(Manifest::new("foo-0.1.0"));
        repo.add_str("foo-0.2.0");
        repo.add_str("foo-0.2.1");
        repo.add_str("bar-0.2.0");
        repo.add_str("bar-0.2.1");

        let packages = repo.packages(false);
        let mut vpackages = packages.iter().map(|x| *x).collect::<Vec<& str>>();
        vpackages.sort();
        let val = vec!["bar", "foo"];
        assert_eq!(vpackages, val);
        assert_eq!(packages.len(), 2);
    }

    #[test]
    fn can_get_ordered_packages_without_versions() {
        let arena = ManifestArena::new();
        let mut repo = ManifestRepo::new(&arena);
        repo.add(Manifest::new("foo-0.1.0"));
        repo.add_str("foo-0.2.0");
        repo.add_str("foo-0.2.1");
        repo.add_str("bar-0.2.0");
        repo.add_str("bar-0.2.1");

        let packages = repo.packages_sorted(false);

        let val = vec!["bar", "foo"];
        assert_eq!(packages, val);
    }


    #[test]
    fn can_add_multiple_times() {
        let arena = ManifestArena::new();
        let mut repo = ManifestRepo::new(&arena);
        repo.add(Manifest::new("foo-0.1.0"));
        repo.add_str("foo-0.2.0");
        repo.add_str("foo-0.2.1");
        let idx = repo.get("foo-0.2.1");
        repo.add_str("foo-0.2.1");
        let idx_after = repo.get("foo-0.2.1");
        assert_eq!(repo.len(), 3);
        assert_eq!(idx, idx_after);
    }

    #[test]
    fn can_load_from_disk() {
        let version = env!("CARGO_MANIFEST_DIR");
        let mut path = PathBuf::from(version);
        path.push("test_resources");
        path.push("manifest_repo");

        let arena = ManifestArena::new();
        let repo = ManifestRepo::from_disk(path, &arena).unwrap();
        assert_eq!(repo.len(), 6);

        let packages  = vec!["abc", "bar", "bla", "foo"];
        assert_eq!(repo.packages_sorted(false), packages);

        let versioned = vec![ "abc-0.1.0","bar-0.1.0",
        "bla-0.2.0", "bla-0.3.0", "foo-0.1.0", "foo-1.0.0"];
        assert_eq!(repo.packages_sorted(true), versioned);
    }

}
