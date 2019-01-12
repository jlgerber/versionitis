/// Define an Interval
pub enum Interval<T: Eq+Ord> {
    Single(T),
    HalfOpen{start:T, end:T},
    Open{start:T, end:T},
}

impl<T: Eq+Ord> Interval<T> {

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
    use crate::package::owned;

    #[test]
    fn single_contains_true() {
        let ident = Interval::Single(owned::VersionNumber::from_string("foo-0.1.0").unwrap());
        let test = owned::VersionNumber::from_string("foo-0.1.0").unwrap();
        assert!(ident.contains(&test));
    }


    #[test]
    fn single_contains_false() {
        let ident = Interval::Single(owned::VersionNumber::from_string("foo-0.2.0").unwrap());
        let test = owned::VersionNumber::from_string("foo-0.1.0").unwrap();
        assert!(!ident.contains(&test));
    }


    #[test]
    fn half_open_contains_true() {
        let ident = Interval::HalfOpen{
            start: owned::VersionNumber::from_string("foo-0.1.0").unwrap(),
            end: owned::VersionNumber::from_string("foo-1.0.0").unwrap()
        };

        let test = owned::VersionNumber::from_string("foo-0.1.1").unwrap();
        assert!(ident.contains(&test));
    }


    #[test]
    fn half_open_contains_to_small() {
let ident = Interval::HalfOpen{
            start: owned::VersionNumber::from_string("foo-0.1.1").unwrap(),
            end: owned::VersionNumber::from_string("foo-1.0.0").unwrap()
        };

        let test = owned::VersionNumber::from_string("foo-0.1.0").unwrap();
        assert!(!ident.contains(&test));
    }



    #[test]
    fn half_open_contains_to_big() {
let ident = Interval::HalfOpen{
            start: owned::VersionNumber::from_string("foo-0.1.1").unwrap(),
            end: owned::VersionNumber::from_string("foo-1.0.0").unwrap()
        };

        let test = owned::VersionNumber::from_string("foo-1.0.1").unwrap();
        assert!(!ident.contains(&test));
    }
    #[test]
    fn half_open_contains_end_false() {
let ident = Interval::HalfOpen{
            start: owned::VersionNumber::from_string("foo-0.1.1").unwrap(),
            end: owned::VersionNumber::from_string("foo-1.0.0").unwrap()
        };

        let test = owned::VersionNumber::from_string("foo-1.0.0").unwrap();
        assert!(!ident.contains(&test));
    }


    #[test]
    fn open_contains_true() {
let ident = Interval::Open{
            start: owned::VersionNumber::from_string("foo-0.1.1").unwrap(),
            end: owned::VersionNumber::from_string("foo-1.0.0").unwrap()
        };

        let test = owned::VersionNumber::from_string("foo-0.5.0").unwrap();
        assert!(ident.contains(&test));
    }

    #[test]
    fn open_contains_too_small() {
let ident = Interval::Open{
            start: owned::VersionNumber::from_string("foo-0.1.1").unwrap(),
            end: owned::VersionNumber::from_string("foo-1.0.0").unwrap()
        };

        let test = owned::VersionNumber::from_string("foo-0.1.0").unwrap();
        assert!(!ident.contains(&test));
    }


    #[test]
    fn open_contains_too_big() {
let ident = Interval::Open{
            start: owned::VersionNumber::from_string("foo-0.1.1").unwrap(),
            end: owned::VersionNumber::from_string("foo-1.0.0").unwrap()
        };

        let test = owned::VersionNumber::from_string("foo-1.0.1").unwrap();
        assert!(!ident.contains(&test));
    }

    #[test]
    fn open_contains_end_true() {
        let ident = Interval::Open {
            start: owned::VersionNumber::from_string("foo-0.1.1").unwrap(),
            end: owned::VersionNumber::from_string("foo-1.0.0").unwrap()
        };

        let test = owned::VersionNumber::from_string("foo-1.0.0").unwrap();
        assert!(ident.contains(&test));
    }

    #[test]
    fn range_filter_half_open_test() {
        let range = vec![
            owned::VersionNumber::from_string("foo-0.1.1").unwrap(),
            owned::VersionNumber::from_string("foo-0.1.2").unwrap(),
            owned::VersionNumber::from_string("foo-0.1.3").unwrap(),
            owned::VersionNumber::from_string("foo-0.2.0").unwrap(),
            owned::VersionNumber::from_string("foo-0.3.0").unwrap(),
            owned::VersionNumber::from_string("foo-0.3.1").unwrap(),
            owned::VersionNumber::from_string("foo-0.3.2").unwrap(),
            owned::VersionNumber::from_string("foo-0.3.3").unwrap(),
            owned::VersionNumber::from_string("foo-0.3.4").unwrap(),
            owned::VersionNumber::from_string("foo-0.4.0").unwrap(),
            owned::VersionNumber::from_string("foo-0.4.1").unwrap(),
            owned::VersionNumber::from_string("foo-0.4.2").unwrap(),
            owned::VersionNumber::from_string("foo-0.4.3").unwrap(),
            owned::VersionNumber::from_string("foo-0.5.0").unwrap(),
            owned::VersionNumber::from_string("foo-0.5.1").unwrap(),
            owned::VersionNumber::from_string("foo-0.5.2").unwrap(),
            owned::VersionNumber::from_string("foo-0.5.3").unwrap(),
        ];

        let expected = &range[6..11].iter().map(|x| x).collect::<Vec<&owned::VersionNumber>>();

        let interval = Interval::HalfOpen {
            start: owned::VersionNumber::from_string("foo-0.3.2").unwrap(),
            end: owned::VersionNumber::from_string("foo-0.4.2").unwrap(),
        };

        let result = range.iter().filter(|x| interval.contains(x)).collect::<Vec<&owned::VersionNumber>>();

        assert_eq!(result.len(), 5);
        assert_eq!(result, *expected);

        let expected = owned::VersionNumber::from_string("foo-0.4.1").unwrap();
        assert_eq!(result[result.len()-1], &expected);
    }
}