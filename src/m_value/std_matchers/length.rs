use colored::Colorize;

use crate::{
    m_value::{m_value::MValue, match_processor::MatchProcessor},
    progress_println,
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

    fn is_match(&self, args: &MValue, value: &MValue) -> bool {
        let matcher = LengthMatcher::from(args);

        match value {
            MValue::Sequence(arr) => matcher == arr.len(),
            MValue::String(s) => matcher == s.len(),
            MValue::Mapping(map) => matcher == map.len(),
            _ => false,
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

impl From<&MValue> for LengthMatcher {
    fn from(value: &MValue) -> Self {
        match value {
            MValue::Number(n) => LengthMatcher::Equal(n.as_u64().unwrap_or(0) as usize),
            MValue::String(s) => match s.as_str() {
                s if s.starts_with("==") => {
                    let value = s[2..].trim().parse::<usize>().unwrap_or_else(|_| {
                        progress_println!(
                            "Invalid length matcher {}. Proper format is {}",
                            s.red(),
                            "'== <number>'".green()
                        );
                        0
                    });
                    LengthMatcher::Equal(value)
                }
                s if s.starts_with("<=") => {
                    let value = s[2..].trim().parse::<usize>().unwrap_or_else(|_| {
                        progress_println!(
                            "Invalid length matcher {}. Proper format is {}",
                            s.red(),
                            "'<= <number>'".green()
                        );
                        0
                    });
                    LengthMatcher::LessEqual(value)
                }
                s if s.starts_with(">=") => {
                    let value = s[2..].trim().parse::<usize>().unwrap_or_else(|_| {
                        progress_println!(
                            "Invalid length matcher {}. Proper format is {}",
                            s.red(),
                            "'>= <number>'".green()
                        );
                        0
                    });
                    LengthMatcher::GreaterEqual(value)
                }
                s if s.starts_with("<") => {
                    let value = s[1..].trim().parse::<usize>().unwrap_or_else(|_| {
                        progress_println!(
                            "Invalid length matcher {}. Proper format is {}",
                            s.red(),
                            "'< <number>'".green()
                        );
                        0
                    });
                    LengthMatcher::LessThan(value)
                }
                s if s.starts_with(">") => {
                    let value = s[1..].trim().parse::<usize>().unwrap_or_else(|_| {
                        progress_println!(
                            "Invalid length matcher {}. Proper format is {}",
                            s.red(),
                            "'> <number>'".green()
                        );
                        0
                    });
                    LengthMatcher::GreaterThan(value)
                }
                _ => {
                    progress_println!("Invalid length matcher {}. Comparison operator must be one of {}, {}, {}, {}, or {}.", s.red(), "'=='".green(), "'<='".green(), "'>='".green(), "'<'".green(), "'>'".green());
                    LengthMatcher::Equal(0)
                }
            },

            _ => {
                progress_println!("Invalid value for $length matcher. Must be a number or string in the format '>= 4' or '< 5'");
                LengthMatcher::Equal(0)
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
        assert!(matcher.is_match(&args, &value));
    }

    #[test]
    fn works_with_string_matcher() {
        let matcher = Length::new();
        let args = MValue::String("<= 6".to_string());
        let value = MValue::String("hello".to_string());
        assert!(matcher.is_match(&args, &value));
    }

    #[test]
    fn works_with_lt_matcher() {
        let matcher = Length::new();
        let args = MValue::String("< 5".to_string());
        let value = MValue::String("hello".to_string());
        assert!(!matcher.is_match(&args, &value));
    }
}
