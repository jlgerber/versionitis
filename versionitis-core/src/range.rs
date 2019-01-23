//! range.rs
//!
//! Enum used to define a range
//!

/// Enum modeling candidate range types wrapping strings
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Range<'a> {
    Single(&'a str),
    HalfOpen(&'a str, &'a str),
    Open(&'a str, &'a str),
}
