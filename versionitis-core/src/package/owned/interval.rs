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