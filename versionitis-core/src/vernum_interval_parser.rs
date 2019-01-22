//! vernum_interval_parser.rs
//!
//! parse package version range strs, converting them to VersionNumberIntervals
//!
use crate::errors::VersionitisError;
use crate::package::owned::interval::VersionNumberInterval;
use crate::interval::Range;
use pest::Parser;
use pest_derive::Parser;


// We create a private _VerNumIntervalParser and a public
// VerNumIntervalParser. They both have a single function, parse.
// The public method is more ergonomic, hiding implementation details
// exposed in _VerNumIntervalParser (see _PacakgeIntervalParser::parse
// signature).
#[derive(Parser)]
#[grammar = "vernum_interval.pest"]
struct _VerNumIntervalParser;

/// Parse package intervals from strs. VerNumIntervalParser has a single
/// function, ```parse```, which is used to construct VersionNumberIntervals from
/// ```&str```s.
pub struct VerNumIntervalParser;

impl VerNumIntervalParser {

    /// Convert a str to a VersionNumberInterval, or a VersionitisError if not successful.
    ///
    /// # Example
    /// ```
    /// use versionitis::vernum_interval_parser::VerNumIntervalParser;
    /// let foo_version_range = VerNumIntervalParser::parse("foo: '1.2.3<2.0.0'");
    /// ```
    pub fn parse(input: &str ) -> Result<VersionNumberInterval, VersionitisError> {
        // call the private parser struct and iterate through returned values
        let ident_list =  _VerNumIntervalParser::parse(Rule::ident_list, input)
            .map_err(|e| VersionitisError::ParseError(
                format!("unable to parse: '{}' error: '{}'",input, e)
            ))?;

        for idx_piece in ident_list {

            match idx_piece.as_rule() {
                Rule::single => {
                    //let mut name=None;
                    let mut version=None;
                    for single_piece in idx_piece.into_inner() {
                        let single_span = single_piece.clone().into_span();
                        match single_piece.as_rule() {

                            Rule::version_a => {
                                version = Some(single_span.as_str());
                            }
                            _ => {}
                        }
                    }
                    // assemble
                    // we can safely unwrap these here because parsing
                    // was successful.
                    let name = format!("{}", version.unwrap());
                    return VersionNumberInterval::from_range(&Range::Single(&name))
                }

                Rule::half_open => {
                    //let mut name = None;
                    let mut version_a = None;
                    let mut version_b = None;

                    for single_piece in idx_piece.into_inner() {
                        let single_span = single_piece.clone().into_span();
                        match single_piece.as_rule() {

                            Rule::version_a => {
                                version_a = Some(single_span.as_str());
                            }

                            Rule::version_b => {
                                version_b = Some(single_span.as_str());
                            }

                            _ => {}
                        }
                    }
                    // assemble. Unwrapping here is safe
                    let v1 = format!("{}", version_a.unwrap());
                    let v2 = format!("{}", version_b.unwrap());
                    return VersionNumberInterval::from_range(&Range::HalfOpen(&v1, &v2))
                }

                Rule::open => {
                    //let mut name = None;
                    let mut version_a = None;
                    let mut version_b = None;

                    for single_piece in idx_piece.into_inner() {
                        let single_span = single_piece.clone().into_span();
                        match single_piece.as_rule() {

                            Rule::version_a => {
                                version_a = Some(single_span.as_str());
                            }

                            Rule::version_b => {
                                version_b = Some(single_span.as_str());
                            }

                            _ => {}
                        }
                    }
                    // assemble. Unwrapping here is safe
                    let v1 = format!("{}",  version_a.unwrap());
                    let v2 = format!("{}", version_b.unwrap());

                    return VersionNumberInterval::from_range(&Range::Open(&v1, &v2))
                }

                _ => unreachable!()

            };
        }

        Err(VersionitisError::ParseError("NotImplemented".to_string()))
    }
}


#[cfg(test)]
mod test {
    use super::*;
    type PI = VersionNumberInterval;
    use self::Range::*;

    #[test]
    fn single_interval() {
        let test = VerNumIntervalParser::parse("1.2.3");
        let result = PI::from_range(&Single("1.2.3"));
        assert_eq!(test, result);
    }

    #[test]
    fn half_open_interval_nospaces() {
        let test = VerNumIntervalParser::parse("1.2.3<2.0.0");
        let result = PI::from_range(&HalfOpen("1.2.3","2.0.0"));
        assert_eq!(test, result);
    }

    #[test]
    fn half_open_interval_spaces() {
        let test = VerNumIntervalParser::parse("1.2.3 < 2.0.0");
        let result = PI::from_range(&HalfOpen("1.2.3","2.0.0"));
        assert_eq!(test, result);
    }

    #[test]
    fn open_interval_nospaces() {
        let test = VerNumIntervalParser::parse("1.2.3<=2.0.0");
        let result = PI::from_range(&Open("1.2.3","2.0.0"));
        assert_eq!(test, result);
    }

    #[test]
    fn open_interval_spaces() {
        let test = VerNumIntervalParser::parse("1.2.3 <= 2.0.0");
        let result = PI::from_range(&Open("1.2.3","2.0.0"));
        assert_eq!(test, result);
    }

}