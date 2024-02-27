use colored::Colorize;

use crate::{
    errors::CaptiError,
    m_value::{m_value::MValue, match_processor::MatchProcessor},
};

/// The $length matcher checks the length of a sequence, string, or mapping.
/// Args can be either an exact numeric value, or a string with an operator and a value.
/// The operators are: '==', '<=', '>=', '<', '>'
/// Returns true if the length of the value matches the args.
pub struct Length;

impl Length {
    pub fn new() -> Box<Self> {
        Box::new(Length)
    }
}

impl MatchProcessor for Length {
    fn key(&self) -> String {
        String::from("$length")
    }

    fn is_match(&self, args: &MValue, value: &MValue) -> Result<bool, CaptiError> {
        let matcher = LengthMatcher::try_from(args)?;

        match value {
            MValue::Sequence(arr) => Ok(matcher == arr.len()),
            MValue::String(s) => Ok(matcher == s.len()),
            MValue::Mapping(map) => Ok(matcher == map.len()),
            _ => Err(CaptiError::matcher_error(format!(
                "Invalid comparison for $length: {}\nValue must be an array, object, or string.",
                value.to_string().red()
            ))),
        }
    }
}

enum LengthMatcher {
    Equal(usize),
    LessThan(usize),
    GreaterThan(usize),
    LessEqual(usize),
    GreaterEqual(usize),
}

impl PartialEq<usize> for LengthMatcher {
    fn eq(&self, other: &usize) -> bool {
        match self {
            LengthMatcher::Equal(n) => n == other,
            LengthMatcher::LessThan(n) => other < n,
            LengthMatcher::GreaterThan(n) => other > n,
            LengthMatcher::LessEqual(n) => other <= n,
            LengthMatcher::GreaterEqual(n) => other >= n,
        }
    }
}

impl TryFrom<&MValue> for LengthMatcher {
    type Error = CaptiError;
    fn try_from(value: &MValue) -> Result<Self, CaptiError> {
        match value {
            MValue::Number(n) => {
                let n = n.as_u64().ok_or(CaptiError::MatcherError {
                    message: format!("Invalid number for $length matcher: {}\nMust be a positive integer.", n),
                })? as usize;

                Ok(LengthMatcher::Equal(n))
            }
            MValue::String(s) => {
                match s.as_str() {
                    s if s.starts_with("==") => {
                        let value = s[2..].trim().parse::<usize>().map_err(|_| {
                            CaptiError::MatcherError {
                                message: format!(
                                    "Invalid length matcher {}. Proper format is '{}'",
                                    s.red(),
                                    "== <number>".green()
                                ),
                            }
                        })?;
                        Ok(LengthMatcher::Equal(value))
                    }
                    s if s.starts_with("<=") => {
                        let value = s[2..].trim().parse::<usize>().map_err(|_| {
                            CaptiError::MatcherError {
                                message: format!(
                                    "Invalid length matcher {}. Proper format is '{}'",
                                    s.red(),
                                    "<= <number>".green()
                                ),
                            }
                        })?;

                        Ok(LengthMatcher::LessEqual(value))
                    }
                    s if s.starts_with(">=") => {
                        let value = s[2..].trim().parse::<usize>().map_err(|_| {
                            CaptiError::MatcherError {
                                message: format!(
                                    "Invalid length matcher {}. Proper format is '{}'",
                                    s.red(),
                                    ">= <number>".green()
                                ),
                            }
                        })?;

                        Ok(LengthMatcher::GreaterEqual(value))
                    }
                    s if s.starts_with("<") => {
                        let value = s[1..].trim().parse::<usize>().map_err(|_| {
                            CaptiError::MatcherError {
                                message: format!(
                                    "Invalid length matcher {}. Proper format is '{}'",
                                    s.red(),
                                    "< <number>".green()
                                ),
                            }
                        })?;

                        Ok(LengthMatcher::LessThan(value))
                    }

                    s if s.starts_with(">") => {
                        let value = s[1..].trim().parse::<usize>().map_err(|_| {
                            CaptiError::MatcherError {
                                message: format!(
                                    "Invalid length matcher {}. Proper format is '{}'",
                                    s.red(),
                                    "> <number>".green()
                                ),
                            }
                        })?;

                        Ok(LengthMatcher::GreaterThan(value))
                    }
                    _ => {
                        Err(
                            CaptiError::MatcherError {
                            message: format!(
"Invalid length matcher {}. Comparison operator must be one of '{}', '{}', '{}', '{}', or '{}'.", s.red(), "==".green(), "<=".green(), ">=".green(), "<".green(), ">".green()
                            ),
                        })
                    }
                }
            }

            _ => {
                Err(CaptiError::MatcherError {
                    message: format!(
                        "Invalid value for $length matcher. Must be a number or string.\nExamples: '3', '>= 4', '< 5'"
                    ),
                })
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::m_value::m_sequence::MSequence;

    use super::*;

    #[test]
    fn works_with_numeric_matcher() {
        let matcher = Length::new();
        let args = MValue::Number(5.into());
        let value = MValue::Sequence(MSequence::from(vec![MValue::Null; 5]));
        assert!(matcher.is_match(&args, &value).unwrap());
    }

    #[test]
    fn works_with_string_matcher() {
        let matcher = Length::new();
        let args = MValue::String("<= 6".to_string());
        let value = MValue::String("hello".to_string());
        assert!(matcher.is_match(&args, &value).unwrap());
    }

    #[test]
    fn works_with_lt_matcher() {
        let matcher = Length::new();
        let args = MValue::String("< 5".to_string());
        let value = MValue::String("hello".to_string());
        assert!(!matcher.is_match(&args, &value).unwrap());
    }
}
