//! package_map
//! map package versions to 1 based id
//!
use std::collections::HashMap;
type IdxType = i32;
//use crate::traits::Versionable;
type PMap = HashMap<String, IdxType>;

pub struct PackageMap {
    idx: IdxType,
    map: PMap
}

impl PackageMap {

    pub fn new() -> Self {
        Self {
            idx: 1,
            map: PMap::new(),
        }
    }

    pub fn add<T>(&mut self, version: T) where T: Into<String> {
        self.map.insert(version.into(), self.idx);
        self.idx +=1;
    }

    pub fn get(&self, value: &str) -> Option<&IdxType> {
        self.map.get(value)
    }

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
        mymap.add("foo-0.1.0");
        mymap.add("foo-0.2.0");
        mymap.add("foo-0.2.1");

        assert_eq!(mymap.len(), 3);
        assert_eq!(mymap.get("foo-0.1.0"), Some(&1));
        assert_eq!(mymap.get("foo-0.2.0"), Some(&2));
        assert_eq!(mymap.get("foo-0.2.1"), Some(&3));
        assert_eq!(mymap.get("foo-bar"), None);
    }
}