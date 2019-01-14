use std::fmt::Debug;
use std::hash::Hash;
/// Define an Interval
#[derive(Debug,PartialEq,Eq,Hash)]
pub enum Interval<T: Eq+Ord+Debug+Hash> {
    Single(T),
    HalfOpen{start:T, end:T},
    Open{start:T, end:T},
}

impl<T: Eq+Ord+Debug+Hash> Interval<T> {

    pub fn contains(&self, value:&T) -> bool {
        match *self {
            Interval::Single(ref v) => {
                value == v
            }
            Interval::HalfOpen{ref start, ref end} => {
                value >= start && value < end
            }
            Interval::Open{ref start, ref end} => {
                value >= start && value <= end
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod packagetests {
        use crate::package::owned::Package;
        use super::*;

        #[test]
        fn single_contains_true() {
            let ident = Interval::Single(Package::from_string("foo-0.1.0").unwrap());
            let test = Package::from_string("foo-0.1.0").unwrap();
            assert!(ident.contains(&test));
        }

        #[test]
        fn single_contains_false() {
            let ident = Interval::Single(Package::from_string("foo-0.2.0").unwrap());
            let test = Package::from_string("foo-0.1.0").unwrap();
            assert!(!ident.contains(&test));
        }

        #[test]
        fn half_open_contains_true() {
            let ident = Interval::HalfOpen{
                start: Package::from_string("foo-0.1.0").unwrap(),
                end: Package::from_string("foo-1.0.0").unwrap()
            };

            let test = Package::from_string("foo-0.1.1").unwrap();
            assert!(ident.contains(&test));
        }

        #[test]
        fn half_open_contains_to_small() {
            let ident = Interval::HalfOpen{
                start: Package::from_string("foo-0.1.1").unwrap(),
                end: Package::from_string("foo-1.0.0").unwrap()
            };

            let test = Package::from_string("foo-0.1.0").unwrap();
            assert!(!ident.contains(&test));
        }

        #[test]
        fn half_open_contains_to_big() {
            let ident = Interval::HalfOpen{
                start: Package::from_string("foo-0.1.1").unwrap(),
                end: Package::from_string("foo-1.0.0").unwrap()
            };

            let test = Package::from_string("foo-1.0.1").unwrap();
            assert!(!ident.contains(&test));
        }
        #[test]
        fn half_open_contains_end_false() {
            let ident = Interval::HalfOpen{
                start: Package::from_string("foo-0.1.1").unwrap(),
                end: Package::from_string("foo-1.0.0").unwrap()
            };

            let test = Package::from_string("foo-1.0.0").unwrap();
            assert!(!ident.contains(&test));
        }


        #[test]
        fn open_contains_true() {
            let ident = Interval::Open{
                start: Package::from_string("foo-0.1.1").unwrap(),
                end: Package::from_string("foo-1.0.0").unwrap()
            };

            let test = Package::from_string("foo-0.5.0").unwrap();
            assert!(ident.contains(&test));
        }

        #[test]
        fn open_contains_too_small() {
            let ident = Interval::Open{
                start: Package::from_string("foo-0.1.1").unwrap(),
                end: Package::from_string("foo-1.0.0").unwrap()
            };

            let test = Package::from_string("foo-0.1.0").unwrap();
            assert!(!ident.contains(&test));
        }

        #[test]
        fn open_contains_too_big() {
            let ident = Interval::Open{
                start: Package::from_string("foo-0.1.1").unwrap(),
                end: Package::from_string("foo-1.0.0").unwrap()
            };

            let test = Package::from_string("foo-1.0.1").unwrap();
            assert!(!ident.contains(&test));
        }

        #[test]
        fn open_contains_end_true() {
            let ident = Interval::Open {
                start: Package::from_string("foo-0.1.1").unwrap(),
                end: Package::from_string("foo-1.0.0").unwrap()
            };

            let test = Package::from_string("foo-1.0.0").unwrap();
            assert!(ident.contains(&test));
        }

        #[test]
        fn range_filter_half_open_test() {
            let range = vec![
                Package::from_string("foo-0.1.1").unwrap(),
                Package::from_string("foo-0.1.2").unwrap(),
                Package::from_string("foo-0.1.3").unwrap(),
                Package::from_string("foo-0.2.0").unwrap(),
                Package::from_string("foo-0.3.0").unwrap(),
                Package::from_string("foo-0.3.1").unwrap(),
                Package::from_string("foo-0.3.2").unwrap(),
                Package::from_string("foo-0.3.3").unwrap(),
                Package::from_string("foo-0.3.4").unwrap(),
                Package::from_string("foo-0.4.0").unwrap(),
                Package::from_string("foo-0.4.1").unwrap(),
                Package::from_string("foo-0.4.2").unwrap(),
                Package::from_string("foo-0.4.3").unwrap(),
                Package::from_string("foo-0.5.0").unwrap(),
                Package::from_string("foo-0.5.1").unwrap(),
                Package::from_string("foo-0.5.2").unwrap(),
                Package::from_string("foo-0.5.3").unwrap(),
            ];

            let expected = &range[6..11].iter().map(|x| x).collect::<Vec<&Package>>();

            let interval = Interval::HalfOpen {
                start: Package::from_string("foo-0.3.2").unwrap(),
                end: Package::from_string("foo-0.4.2").unwrap(),
            };

            let result = range.iter().filter(|x| interval.contains(x)).collect::<Vec<&Package>>();

            assert_eq!(result.len(), 5);
            assert_eq!(result, *expected);

            let expected = Package::from_string("foo-0.4.1").unwrap();
            assert_eq!(result[result.len()-1], &expected);
        }
    }

    mod version_num_test {
        use crate::version_number::VersionNumber;
        use super::*;
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
            let ident = Interval::HalfOpen{
                start: VersionNumber::from_string("0.1.0").unwrap(),
                end: VersionNumber::from_string("1.0.0").unwrap()
            };

            let test = VersionNumber::from_string("0.1.1").unwrap();
            assert!(ident.contains(&test));
        }

        #[test]
        fn half_open_contains_to_small() {
            let ident = Interval::HalfOpen{
                start: VersionNumber::from_string("0.1.1").unwrap(),
                end: VersionNumber::from_string("1.0.0").unwrap()
            };

            let test = VersionNumber::from_string("0.1.0").unwrap();
            assert!(!ident.contains(&test));
        }



        #[test]
        fn half_open_contains_to_big() {
            let ident = Interval::HalfOpen{
                start: VersionNumber::from_string("0.1.1").unwrap(),
                end: VersionNumber::from_string("1.0.0").unwrap()
            };

            let test = VersionNumber::from_string("1.0.1").unwrap();
            assert!(!ident.contains(&test));
        }

        #[test]
        fn half_open_contains_end_false() {
            let ident = Interval::HalfOpen{
                start: VersionNumber::from_string("0.1.1").unwrap(),
                end: VersionNumber::from_string("1.0.0").unwrap()
            };

            let test = VersionNumber::from_string("1.0.0").unwrap();
            assert!(!ident.contains(&test));
        }

        #[test]
        fn open_contains_true() {
            let ident = Interval::Open{
                start: VersionNumber::from_string("0.1.1").unwrap(),
                end: VersionNumber::from_string("1.0.0").unwrap()
            };

            let test = VersionNumber::from_string("0.5.0").unwrap();
            assert!(ident.contains(&test));
        }

        #[test]
        fn open_contains_too_small() {
            let ident = Interval::Open{
                start: VersionNumber::from_string("0.1.1").unwrap(),
                end: VersionNumber::from_string("1.0.0").unwrap()
            };

            let test = VersionNumber::from_string("0.1.0").unwrap();
            assert!(!ident.contains(&test));
        }


        #[test]
        fn open_contains_too_big() {
            let ident = Interval::Open{
                start: VersionNumber::from_string("0.1.1").unwrap(),
                end: VersionNumber::from_string("1.0.0").unwrap()
            };

            let test = VersionNumber::from_string("1.0.1").unwrap();
            assert!(!ident.contains(&test));
        }

        #[test]
        fn open_contains_end_true() {
            let ident = Interval::Open {
                start: VersionNumber::from_string("0.1.1").unwrap(),
                end: VersionNumber::from_string("1.0.0").unwrap()
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

            let expected = &range[6..11].iter().map(|x| x).collect::<Vec<&VersionNumber>>();

            let interval = Interval::HalfOpen {
                start: VersionNumber::from_string("0.3.2").unwrap(),
                end: VersionNumber::from_string("0.4.2").unwrap(),
            };

            let result = range.iter().filter(|x| interval.contains(x))
            .collect::<Vec<&VersionNumber>>();

            assert_eq!(result.len(), 5);
            assert_eq!(result, *expected);

            let expected = VersionNumber::from_string("0.4.1").unwrap();
            assert_eq!(result[result.len()-1], &expected);
        }
    }
}