use std::fmt::Debug;
use core::str::FromStr;

/// Trait for defining a version scheme
pub trait Versionable: Eq + Ord + Debug + ToString + FromStr {}
