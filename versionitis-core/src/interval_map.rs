
use crate::{errors::VersionitisError, interval::Interval, package::owned::Package};
use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_derive::{Deserialize, Serialize};
use std::collections::{ HashMap, hash_map::{ Keys, Values, ValuesMut, Iter, IterMut, Entry, Drain, RandomState } };
use crate::package::owned::interval::{PackageInterval};
use std::borrow::Borrow;
use std::fmt;
use std::cmp::{ PartialEq, Eq };

pub type _IntervalMap = HashMap<String, PackageInterval>;

impl PartialEq for IntervalMap {
     fn eq(&self, other: &IntervalMap) -> bool {
         self.0.eq(&other.0)
     }
}

impl Eq for IntervalMap {}

#[derive(Deserialize)]
pub struct IntervalMap(_IntervalMap);

impl fmt::Debug for IntervalMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IntervalMap")
    }
}

impl IntervalMap {
    pub fn new() -> Self {
        Self (
            _IntervalMap::new()
        )
    }

    pub fn hasher(&self) -> &RandomState {
        self.0.hasher()
    }

    pub fn capacity(&self) -> usize	{
        self.0.capacity()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self (_IntervalMap::with_capacity(capacity))
    }

    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional)
    }

    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit()
    }

    pub fn keys(&self) -> Keys<String, PackageInterval> {
        self.0.keys()
    }

    /// An iterator visiting all values in arbitrary order.
    /// The iterator element type is &'a PackageInterval.
    pub fn values(&self) -> Values<String, PackageInterval> {
        self.0.values()
    }

    pub fn values_mut(&mut self) -> ValuesMut<String, PackageInterval> {
        self.0.values_mut()
    }

    pub fn iter(&self) -> Iter<String, PackageInterval> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<String, PackageInterval> {
        self.0.iter_mut()
    }

    pub fn entry(&mut self, key: String) -> Entry<String, PackageInterval> {
        self.0.entry(key)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn drain(&mut self) -> Drain<String, PackageInterval> {
        self.0.drain()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn get(&self, k: &str) -> Option<&PackageInterval> {
        self.0.get(k)
    }

    pub fn contains_key(&self, k: &str) -> bool {
        self.0.contains_key(k)
    }

    pub fn get_mut(&mut self, k: &str) -> Option<&mut PackageInterval> {
        self.0.get_mut(k)
    }

    ///Inserts a key-value pair into the map. If the map did not have
    /// this key present, None is returned.If the map did have this key
    /// present, the value is updated, and the old value is returned.
    /// The key is not updated, though
    pub fn insert<K: Into<String>>(&mut self, k: K, v: PackageInterval) -> Option<PackageInterval> {
        self.0.insert(k.into(), v)
    }

    pub fn remove(&mut self, k: &str) -> Option<PackageInterval> {
        self.0.remove(k)
    }

}

impl Serialize for IntervalMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in &self.0 {
            map.serialize_entry(k, v)?;
        }
        map.end()
        // match *self {
        //     Interval::Single(ref v) => {
        //         serializer.serialize_newtype_variant("Interval", 0, "single", &v.spec())
        //     }

        //     Interval::HalfOpen { ref start, ref end } => {
        //         let mut state =
        //             serializer.serialize_struct_variant("Interval", 0, "half_open", 2)?;
        //         state.serialize_field("start", &start.spec())?;
        //         state.serialize_field("end", &end.spec())?;
        //         state.end()
        //     }

        //     Interval::Open { ref start, ref end } => {
        //         let mut state = serializer.serialize_struct_variant("Interval", 0, "open", 2)?;
        //         state.serialize_field("start", &start.spec())?;
        //         state.serialize_field("end", &end.spec())?;
        //         state.end()
        //     }
        // }
    }
}
