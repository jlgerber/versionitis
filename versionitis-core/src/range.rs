//! range.rs
//!
//! Enum used to define a range
//!

/// Enum wrapping possible inputs to from_src
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PISrc<'a> {
    Single(&'a str),
    HalfOpen(&'a str, &'a str),
    Open(&'a str, &'a str),
}
