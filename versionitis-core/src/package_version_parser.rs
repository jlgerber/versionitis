//! vernum_interval_parser.rs
//!
//! parse package version range strs, converting them to VersionNumberIntervals
//!
use crate::errors::VersionitisError;
use crate::version_number_interval::VersionNumberInterval;
use crate::interval::Range;
use pest::Parser;
use pest_derive::Parser;


// We create a private _PackageVersionParser and a public
// PackageVersionParser. They both have a single function, parse.
// The public method is more ergonomic, hiding implementation details
// exposed in _PackageVersionParser (see _PacakgeIntervalParser::parse
// signature).
#[derive(Parser)]
#[grammar = "package_version.pest"]
struct _PackageVersionParser;

/// Parse package intervals from strs. PackageVersionParser has a single
/// function, ```parse```, which is used to construct VersionNumberIntervals from
/// ```&str```s.
pub struct PackageVersionParser;

impl PackageVersionParser {

    /// Convert a str to a PackageVersion, or a VersionitisError if not successful.
    ///
    /// # Example
    /// ```
    /// use versionitis::package_version_parser::PackageVersionParser;
    /// let (name,version) = PackageVersionParser::parse("foo-1.2.3").unwrap();
    /// ```
    pub fn parse(input: &str ) -> Result<(&str, &str), VersionitisError> {
        // call the private parser struct and iterate through returned values
        let single =  _PackageVersionParser::parse(Rule::single, input)
            .map_err(|e| VersionitisError::ParseError(
                format!("unable to parse: '{}' error: '{}'",input, e)
            ))?;

        let mut version=None;
        let mut name=None;
        for single_piece in single {
            let single_span = single_piece.clone().into_span();
            match single_piece.as_rule() {

                Rule::version => {
                    version = Some(single_span.as_str());
                }
                Rule::name => {
                    name = Some(single_span.as_str())
                }
                _ => {}
            };
        }

        if name.is_none() || version.is_none() {
            return Err(VersionitisError::ParseError(
                format!("unable to parse {} name.is_none:{} version.is_none:{}",
                input, name.is_none(), version.is_none())));
        }

        return Ok((name.unwrap(), version.unwrap()))
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_parse_name_and_version() {
        let result = PackageVersionParser::parse("foo-1.2.3");
        if let Ok((name, version)) =  result {
            assert_eq!(name, "foo");
            assert_eq!(version, "1.2.3");
        } else {
            assert_eq!(result, Err(VersionitisError::SerdeYamlError("NO WAY".to_string())));
        }
    }

    #[test]
    fn can_parse_name_with_underscores() {
        let result = PackageVersionParser::parse("foo_bar-1.2.3");
        if let Ok((name, version)) =  result {
            assert_eq!(name, "foo_bar");
            assert_eq!(version, "1.2.3");
        } else {
            assert_eq!(result, Err(VersionitisError::SerdeYamlError("NO WAY".to_string())));
        }
    }

    #[test]
    fn name_cannot_start_with_digit() {
        let result = PackageVersionParser::parse("1foo_bar-1.2.3");
        assert!(result.is_err());
    }

    #[test]
    fn name_cannot_contain_multiple_contiguous_underscores() {
        let result = PackageVersionParser::parse("foo__bar-1.2.3");
        assert!(result.is_err());
    }

    #[test]
    fn name_cannot_end_with_underscore() {
        let result = PackageVersionParser::parse("foo_bar_-1.2.3");
        assert!(result.is_err());
    }

    #[test]
    fn name_cannot_start_with_space() {
        let result = PackageVersionParser::parse(" foo_bar_1.2.3 ");
        assert!(result.is_err());
    }

    #[test]
    fn can_pass_in_string_reference() {
        let pkg = "foo_bar-1.2.3".to_string();
        let result = PackageVersionParser::parse(&pkg);
        if let Ok((name, version)) =  result {
            assert_eq!(name, "foo_bar");
            assert_eq!(version, "1.2.3");
        } else {
            assert_eq!(result, Err(VersionitisError::SerdeYamlError("NO WAY".to_string())));
        }
    }
}