//! interval.rs
//!
//! Define an enum which represents an interval of
//! generic type T.
use std::{fmt::Debug, hash::Hash};
use serde_derive::{Deserialize, Serialize};

/// Enum wrapping possible inputs to PackageInterval::from_src
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Range<'a> {
    Single(&'a str),
    HalfOpen(&'a str, &'a str),
    Open(&'a str, &'a str),
}

/// Define an Interval enum which may be a Single value, HalfOpen, or Open.
/// A HalfOpen value's lower bound is inclusive, whereas an Open bound's lower
/// and upper bounds are inclusive.
#[derive(Debug, PartialEq, Eq, Hash, Clone /*, Deserialize*/)]
//#[serde(rename_all = "snake_case")]
pub enum Interval<T: Eq + Ord + Debug + Hash + Clone> {
    Single(T),
    HalfOpen { start: T, end: T },
    Open { start: T, end: T },
}

impl<T: Eq + Ord + Debug + Hash + Clone> Interval<T> {
    /// Test whether a the Interval contains a specific
    /// value T.
    pub fn contains(&self, value: &T) -> bool {
        match *self {
            Interval::Single(ref v) => value == v,
            Interval::HalfOpen { ref start, ref end } => value >= start && value < end,
            Interval::Open { ref start, ref end } => value >= start && value <= end,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod packagetests {
        use super::*;
        use crate::package::owned::Package;

        #[test]
        fn single_contains_true() {
            let ident = Interval::Single(Package::from_str("foo-0.1.0").unwrap());
            let test = Package::from_str("foo-0.1.0").unwrap();
            assert!(ident.contains(&test));
        }

        #[test]
        fn single_contains_false() {
            let ident = Interval::Single(Package::from_str("foo-0.2.0").unwrap());
            let test = Package::from_str("foo-0.1.0").unwrap();
            assert!(!ident.contains(&test));
        }

        #[test]
        fn half_open_contains_true() {
            let ident = Interval::HalfOpen {
                start: Package::from_str("foo-0.1.0").unwrap(),
                end: Package::from_str("foo-1.0.0").unwrap(),
            };

            let test = Package::from_str("foo-0.1.1").unwrap();
            assert!(ident.contains(&test));
        }

        #[test]
        fn half_open_contains_to_small() {
            let ident = Interval::HalfOpen {
                start: Package::from_str("foo-0.1.1").unwrap(),
                end: Package::from_str("foo-1.0.0").unwrap(),
            };

            let test = Package::from_str("foo-0.1.0").unwrap();
            assert!(!ident.contains(&test));
        }

        #[test]
        fn half_open_contains_to_big() {
            let ident = Interval::HalfOpen {
                start: Package::from_str("foo-0.1.1").unwrap(),
                end: Package::from_str("foo-1.0.0").unwrap(),
            };

            let test = Package::from_str("foo-1.0.1").unwrap();
            assert!(!ident.contains(&test));
        }
        #[test]
        fn half_open_contains_end_false() {
            let ident = Interval::HalfOpen {
                start: Package::from_str("foo-0.1.1").unwrap(),
                end: Package::from_str("foo-1.0.0").unwrap(),
            };

            let test = Package::from_str("foo-1.0.0").unwrap();
            assert!(!ident.contains(&test));
        }

        #[test]
        fn open_contains_true() {
            let ident = Interval::Open {
                start: Package::from_str("foo-0.1.1").unwrap(),
                end: Package::from_str("foo-1.0.0").unwrap(),
            };

            let test = Package::from_str("foo-0.5.0").unwrap();
            assert!(ident.contains(&test));
        }

        #[test]
        fn open_contains_too_small() {
            let ident = Interval::Open {
                start: Package::from_str("foo-0.1.1").unwrap(),
                end: Package::from_str("foo-1.0.0").unwrap(),
            };

            let test = Package::from_str("foo-0.1.0").unwrap();
            assert!(!ident.contains(&test));
        }

        #[test]
        fn open_contains_too_big() {
            let ident = Interval::Open {
                start: Package::from_str("foo-0.1.1").unwrap(),
                end: Package::from_str("foo-1.0.0").unwrap(),
            };

            let test = Package::from_str("foo-1.0.1").unwrap();
            assert!(!ident.contains(&test));
        }

        #[test]
        fn open_contains_end_true() {
            let ident = Interval::Open {
                start: Package::from_str("foo-0.1.1").unwrap(),
                end: Package::from_str("foo-1.0.0").unwrap(),
            };

            let test = Package::from_str("foo-1.0.0").unwrap();
            assert!(ident.contains(&test));
        }

        #[test]
        fn range_filter_half_open_test() {
            let range = vec![
                Package::from_str("foo-0.1.1").unwrap(),
                Package::from_str("foo-0.1.2").unwrap(),
                Package::from_str("foo-0.1.3").unwrap(),
                Package::from_str("foo-0.2.0").unwrap(),
                Package::from_str("foo-0.3.0").unwrap(),
                Package::from_str("foo-0.3.1").unwrap(),
                Package::from_str("foo-0.3.2").unwrap(),
                Package::from_str("foo-0.3.3").unwrap(),
                Package::from_str("foo-0.3.4").unwrap(),
                Package::from_str("foo-0.4.0").unwrap(),
                Package::from_str("foo-0.4.1").unwrap(),
                Package::from_str("foo-0.4.2").unwrap(),
                Package::from_str("foo-0.4.3").unwrap(),
                Package::from_str("foo-0.5.0").unwrap(),
                Package::from_str("foo-0.5.1").unwrap(),
                Package::from_str("foo-0.5.2").unwrap(),
                Package::from_str("foo-0.5.3").unwrap(),
            ];

            let expected = &range[6..11].iter().map(|x| x).collect::<Vec<&Package>>();

            let interval = Interval::HalfOpen {
                start: Package::from_str("foo-0.3.2").unwrap(),
                end: Package::from_str("foo-0.4.2").unwrap(),
            };

            let result = range
                .iter()
                .filter(|x| interval.contains(x))
                .collect::<Vec<&Package>>();

            assert_eq!(result.len(), 5);
            assert_eq!(result, *expected);

            let expected = Package::from_str("foo-0.4.1").unwrap();
            assert_eq!(result[result.len() - 1], &expected);
        }
    }

    mod version_num_test {
        use super::*;
        use crate::version_number::VersionNumber;
        #[test]
        fn single_contains_true() {
            let ident = Interval::Single(VersionNumber::from_string("0.1.0").unwrap());
            let test = VersionNumber::from_string("0.1.0").unwrap();
            assert!(ident.contains(&test));
        }

        #[test]
        fn single_contains_false() {
            let ident = Interval::Single(VersionNumber::from_string("0.2.0").unwrap());
            let test = VersionNumber::from_string("0.1.0").unwrap();
            assert!(!ident.contains(&test));
        }

        #[test]
        fn half_open_contains_true() {
            let ident = Interval::HalfOpen {
                start: VersionNumber::from_string("0.1.0").unwrap(),
                end: VersionNumber::from_string("1.0.0").unwrap(),
            };

            let test = VersionNumber::from_string("0.1.1").unwrap();
            assert!(ident.contains(&test));
        }

        #[test]
        fn half_open_contains_to_small() {
            let ident = Interval::HalfOpen {
                start: VersionNumber::from_string("0.1.1").unwrap(),
                end: VersionNumber::from_string("1.0.0").unwrap(),
            };

            let test = VersionNumber::from_string("0.1.0").unwrap();
            assert!(!ident.contains(&test));
        }

        #[test]
        fn half_open_contains_to_big() {
            let ident = Interval::HalfOpen {
                start: VersionNumber::from_string("0.1.1").unwrap(),
                end: VersionNumber::from_string("1.0.0").unwrap(),
            };

            let test = VersionNumber::from_string("1.0.1").unwrap();
            assert!(!ident.contains(&test));
        }

        #[test]
        fn half_open_contains_end_false() {
            let ident = Interval::HalfOpen {
                start: VersionNumber::from_string("0.1.1").unwrap(),
                end: VersionNumber::from_string("1.0.0").unwrap(),
            };

            let test = VersionNumber::from_string("1.0.0").unwrap();
            assert!(!ident.contains(&test));
        }

        #[test]
        fn open_contains_true() {
            let ident = Interval::Open {
                start: VersionNumber::from_string("0.1.1").unwrap(),
                end: VersionNumber::from_string("1.0.0").unwrap(),
            };

            let test = VersionNumber::from_string("0.5.0").unwrap();
            assert!(ident.contains(&test));
        }

        #[test]
        fn open_contains_too_small() {
            let ident = Interval::Open {
                start: VersionNumber::from_string("0.1.1").unwrap(),
                end: VersionNumber::from_string("1.0.0").unwrap(),
            };

            let test = VersionNumber::from_string("0.1.0").unwrap();
            assert!(!ident.contains(&test));
        }

        #[test]
        fn open_contains_too_big() {
            let ident = Interval::Open {
                start: VersionNumber::from_string("0.1.1").unwrap(),
                end: VersionNumber::from_string("1.0.0").unwrap(),
            };

            let test = VersionNumber::from_string("1.0.1").unwrap();
            assert!(!ident.contains(&test));
        }

        #[test]
        fn open_contains_end_true() {
            let ident = Interval::Open {
                start: VersionNumber::from_string("0.1.1").unwrap(),
                end: VersionNumber::from_string("1.0.0").unwrap(),
            };

            let test = VersionNumber::from_string("1.0.0").unwrap();
            assert!(ident.contains(&test));
        }

        #[test]
        fn range_filter_half_open_test() {
            let range = vec![
                VersionNumber::from_string("0.1.1").unwrap(),
                VersionNumber::from_string("0.1.2").unwrap(),
                VersionNumber::from_string("0.1.3").unwrap(),
                VersionNumber::from_string("0.2.0").unwrap(),
                VersionNumber::from_string("0.3.0").unwrap(),
                VersionNumber::from_string("0.3.1").unwrap(),
                VersionNumber::from_string("0.3.2").unwrap(),
                VersionNumber::from_string("0.3.3").unwrap(),
                VersionNumber::from_string("0.3.4").unwrap(),
                VersionNumber::from_string("0.4.0").unwrap(),
                VersionNumber::from_string("0.4.1").unwrap(),
                VersionNumber::from_string("0.4.2").unwrap(),
                VersionNumber::from_string("0.4.3").unwrap(),
                VersionNumber::from_string("0.5.0").unwrap(),
                VersionNumber::from_string("0.5.1").unwrap(),
                VersionNumber::from_string("0.5.2").unwrap(),
                VersionNumber::from_string("0.5.3").unwrap(),
            ];

            let expected = &range[6..11]
                .iter()
                .map(|x| x)
                .collect::<Vec<&VersionNumber>>();

            let interval = Interval::HalfOpen {
                start: VersionNumber::from_string("0.3.2").unwrap(),
                end: VersionNumber::from_string("0.4.2").unwrap(),
            };

            let result = range
                .iter()
                .filter(|x| interval.contains(x))
                .collect::<Vec<&VersionNumber>>();

            assert_eq!(result.len(), 5);
            assert_eq!(result, *expected);

            let expected = VersionNumber::from_string("0.4.1").unwrap();
            assert_eq!(result[result.len() - 1], &expected);
        }
    }
}
