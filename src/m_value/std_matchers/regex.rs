use colored::Colorize;

use crate::{
    errors::CaptiError,
    m_value::{m_value::MValue, match_processor::MatchProcessor},
};

/// The $regex matcher takes in a regex wrapped by '/' characters, and determines whether a match
/// can be found in the provided value.
/// Returns true if a match is found anywhere in the value.
/// Returns false if no match is found.
pub struct Regex;

impl Regex {
    pub fn new() -> Box<Self> {
        Box::new(Regex)
    }
}

impl MatchProcessor for Regex {
    fn key(&self) -> String {
        String::from("$regex")
    }

    fn is_match(&self, args: &MValue, value: &MValue) -> Result<bool, CaptiError> {
        match (args, value) {
            (MValue::String(args), MValue::String(value)) => {
                match regex_match(args.clone(), value.clone()) {
                    Ok(result) => Ok(result),
                    Err(_) => Err(CaptiError::matcher_error(format!(
                        "Invalid argument for $regex matcher: {}\nRegular expression must be valid and wrapped in '{}' characters.",
                        args.red(),
                        "/".yellow()
                    ))),
                }
            }
            (MValue::String(_args), _) => Ok(false),

            _ => Err(CaptiError::matcher_error(format!(
                "Invalid argument for $regex matcher: {}\nRegular expression must be a string.",
                args.to_string().red()
            ))),
        }
    }
}

fn regex_match(args: String, value: String) -> Result<bool, ()> {
    let first_char = args.chars().nth(0).ok_or(())?;
    let last_char = args.chars().last().ok_or(())?;

    let regex_str = match (first_char, last_char) {
        ('/', '/') => &args[1..args.len() - 1],
        _ => return Err(()),
    };

    let regex = regex::Regex::new(regex_str).map_err(|_| ())?;
    Ok(regex.is_match(&value))
}

#[cfg(test)]
mod test {
    use serde_yaml::Number;

    use super::*;

    #[test]
    fn matches_with_valid_regex() {
        let regex = Regex::new();
        let args = MValue::String(String::from("/^abc$/"));
        let value = MValue::String(String::from("abc"));
        assert!(regex.is_match(&args, &value).unwrap());
    }

    #[test]
    fn errors_with_invalid_regex() {
        let regex = Regex::new();
        let args = MValue::String(String::from("^abc$"));
        let value = MValue::String(String::from("abc"));
        assert!(regex.is_match(&args, &value).is_err());
    }

    #[test]
    fn errors_with_invalid_regex_non_str() {
        let regex = Regex::new();
        let args = MValue::Number(Number::from(1));
        let value = MValue::String(String::from("1"));
        assert!(regex.is_match(&args, &value).is_err());
    }
}
