//! package_interval_parser.rs
//!
//! parse package version range strs, converting them to PackageIntervals
//!
use crate::errors::VersionitisError;
use crate::package::owned::interval::PackageInterval;
use crate::interval::Range;
use pest::Parser;
use pest_derive::Parser;


// The pest parser is not exposed directly.
#[derive(Parser)]
#[grammar = "package_interval.pest"]
struct _PackageIntervalParser;

// IndexParser is a convenience struct which provides a parse method that is more suited
// to the api than the raw pest _IndexParser.

/// Parse package intervals from strs.
///
/// # Example
///
/// ```
/// let foo_version_range = PackageIntervalParser::parse("foo=1.2.3<2.0.0");
///
pub struct PackageIntervalParser;

impl PackageIntervalParser {
    /// parse an elasticsearch index, of the form ```name-YYYY.MM.DD``` and return
    /// a Result - either an Ok Index instance, or an Err String.
    pub fn parse(input: &str ) -> Result<PackageInterval, VersionitisError> {
        let ident_list =  _PackageIntervalParser::parse(Rule::ident_list, input)
        .map_err(|e| VersionitisError::ParseError(format!("unable to parse: '{}' error: '{}'",input, e)))?;

        // parsing guarantees that these vars are going to get set. we just choose arbitrary
        // values for now.

        for idx_piece in ident_list {

            // A idx_piece can be converted to an iterator of the tokens which make it up:
           // for inner_idx_piece in idx_piece.into_inner() {
                let inner_span = idx_piece.clone().into_span();

                match idx_piece.as_rule() {
                    Rule::single => {
                        let mut name=None;
                        let mut version=None;
                        for single_piece in idx_piece.into_inner() {
                            let single_span = single_piece.clone().into_span();
                            match single_piece.as_rule() {
                                Rule::name => {
                                    name = Some(single_span.as_str());
                                }
                                Rule::version_a => {
                                    version = Some(single_span.as_str());
                                }
                                _ => {}
                            }
                        }
                        // assemble
                        // we can unwrap these here
                        let name = format!("{}-{}", name.unwrap(), version.unwrap());
                        return PackageInterval::from_range(&Range::Single(&name))
                    }

                    Rule::half_open => {
                        let mut name = None;
                        let mut version_a = None;
                        let mut version_b = None;

                        for single_piece in idx_piece.into_inner() {
                            let single_span = single_piece.clone().into_span();
                            match single_piece.as_rule() {
                                Rule::name => {
                                    name = Some(single_span.as_str());
                                }

                                Rule::version_a => {
                                    version_a = Some(single_span.as_str());
                                }

                                Rule::version_b => {
                                    version_b = Some(single_span.as_str());
                                }

                                _ => {}
                            }
                        }
                        // assemble
                        // we can unwrap these here
                        let v1 = format!("{}-{}", name.unwrap(), version_a.unwrap());
                        let v2 = format!("{}-{}", name.unwrap(), version_b.unwrap());
                        return PackageInterval::from_range(&Range::HalfOpen(&v1, &v2))
                    }

                    Rule::open => {
                        let mut name = None;
                        let mut version_a = None;
                        let mut version_b = None;

                        for single_piece in idx_piece.into_inner() {
                            let single_span = single_piece.clone().into_span();
                            match single_piece.as_rule() {
                                Rule::name => {
                                    name = Some(single_span.as_str());
                                }

                                Rule::version_a => {
                                    version_a = Some(single_span.as_str());
                                }

                                Rule::version_b => {
                                    version_b = Some(single_span.as_str());
                                }

                                _ => {}
                            }
                        }
                        // assemble
                        // we can unwrap these here
                        let v1 = format!("{}-{}", name.unwrap(), version_a.unwrap());
                        let v2 = format!("{}-{}", name.unwrap(), version_b.unwrap());
                        return PackageInterval::from_range(&Range::Open(&v1, &v2))
                    }

                    _ => unreachable!()
                };
            //}
        }
       Err(VersionitisError::ParseError("NotImplemented".to_string()))
    }
}


#[cfg(test)]
mod test {
    use super::*;
    type PI = PackageInterval;
    use self::Range::*;

    #[test]
    fn single_interval() {
        let test = PackageIntervalParser::parse("foo=1.2.3");
        let result = PI::from_range(&Single("foo-1.2.3"));
        assert_eq!(test, result);
    }

    #[test]
    fn single_interval_spaces() {
        let test = PackageIntervalParser::parse("foo = 1.2.3");
        let result = PI::from_range(&Single("foo-1.2.3"));
        assert_eq!(test, result);
    }

    #[test]
    fn half_open_interval() {
        let test = PackageIntervalParser::parse("foo=1.2.3<2.0.0");
        let result = PI::from_range(&HalfOpen("foo-1.2.3","foo-2.0.0"));
        assert_eq!(test, result);
    }

    #[test]
    fn half_open_interval_spaces() {
        let test = PackageIntervalParser::parse("foo = 1.2.3 < 2.0.0");
        let result = PI::from_range(&HalfOpen("foo-1.2.3","foo-2.0.0"));
        assert_eq!(test, result);
    }

    #[test]
    fn open_interval() {
        let test = PackageIntervalParser::parse("foo=1.2.3<=2.0.0");
        let result = PI::from_range(&Open("foo-1.2.3","foo-2.0.0"));
        assert_eq!(test, result);
    }


    #[test]
    fn open_interval_spaces() {
        let test = PackageIntervalParser::parse("foo = 1.2.3 <= 2.0.0");
        let result = PI::from_range(&Open("foo-1.2.3","foo-2.0.0"));
        assert_eq!(test, result);
    }
}