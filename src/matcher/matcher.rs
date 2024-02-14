use colored::Colorize;
use regex::Regex;

use crate::progress_println;

use super::{MatchCmp, MatchResult};

pub enum Matcher {
    Exact(String),
    Exists,
    Regex(Regex),
    Includes(serde_json::Value),
    Empty,
    Absent,
    Length(usize),
}

impl Matcher {
    pub fn matches_value(&self, value: &serde_json::Value) -> bool {
        match self {
            Matcher::Exists => value.ne(&serde_json::Value::Null),
            Matcher::Exact(expected) => match value {
                serde_json::Value::String(s) => expected.eq(s),
                _ => false,
            },
            Matcher::Regex(regex) => match value {
                serde_json::Value::String(s) => regex.is_match(s),
                _ => false,
            },
            Matcher::Includes(expected) => match value {
                serde_json::Value::Array(arr) => arr.iter().any(|v| match expected.match_cmp(&v) {
                    MatchResult::Matches => true,
                    _ => false,
                }),
                _ => false,
            },
            Matcher::Empty => match value {
                serde_json::Value::Array(arr) => arr.is_empty(),
                serde_json::Value::Object(obj) => obj.is_empty(),
                _ => false,
            },
            Matcher::Absent => match value {
                serde_json::Value::Null => true,
                _ => false,
            },
            Matcher::Length(length) => match value {
                serde_json::Value::Array(arr) => arr.len().eq(length),
                serde_json::Value::Object(obj) => obj.len().eq(length),
                serde_json::Value::String(s) => s.len().eq(length),
                _ => false,
            },
        }
    }
}

impl From<&str> for Matcher {
    fn from(value: &str) -> Self {
        match value {
            "$exists" => Matcher::Exists,
            "$empty" => Matcher::Empty,
            "$absent" => Matcher::Absent,
            s if s.starts_with("$length") => {
                let length = value[8..].parse::<usize>().unwrap_or_else(|_| {
                    progress_println!("{}: Invalid length matcher {}.", "Warning".yellow(), value);
                    0
                });

                Matcher::Length(length)
            }
            s if s.starts_with("$regex") => match extract_regex(s) {
                Some(regex) => Matcher::Regex(regex),
                None => Matcher::Exact(s.to_string()),
            },
            s if s.starts_with("$includes") => {
                let value = serde_json::from_str(&s[10..]);
                match value {
                    Ok(value) => Matcher::Includes(value),
                    Err(_) => Matcher::Includes(serde_json::Value::String(s[10..].to_string())),
                }
            }
            _ => Matcher::Exact(value.to_string()),
        }
    }
}

impl From<&String> for Matcher {
    fn from(value: &String) -> Self {
        Matcher::from(value.as_str())
    }
}

impl From<String> for Matcher {
    fn from(value: String) -> Self {
        Matcher::from(value.as_str())
    }
}

fn extract_regex(s: &str) -> Option<Regex> {
    // format: $regex{ /regex/ }
    let mut regex_str = s
        .chars()
        .skip_while(|c| c.ne(&'/'))
        .skip(1)
        .collect::<Vec<char>>();
    while let Some(c) = regex_str.pop() {
        if c.eq(&'/') {
            break;
        }
    }
    let regex_statement = regex_str.into_iter().collect::<String>();

    Regex::new(&regex_statement).ok()
}

#[cfg(test)]
mod test {
    use serde_json::Number;

    use super::*;

    #[test]
    fn exact_values() {
        let matches =
            Matcher::from("123").matches_value(&serde_json::Value::String(String::from("123")));

        assert!(matches);
    }

    #[test]
    fn exists_matches() {
        let matches =
            Matcher::from("$exists").matches_value(&serde_json::Value::Number(Number::from(3)));
        assert!(matches);
    }

    #[test]
    fn exists_nomatch_null() {
        let matches = Matcher::from("$exists").matches_value(&serde_json::Value::Null);
        assert!(!matches);
    }

    #[test]
    fn extract_regex_fn() {
        let regex_str = "$regex /(\\{\\})/";
        let regex = extract_regex(regex_str).unwrap();
        assert_eq!(regex.to_string(), "(\\{\\})");
    }

    #[test]
    fn matches_regex_expression() {
        let regex_str = "$regex /.*[Hh]ello!.*/";
        let match_string = serde_json::Value::String(String::from("Hello! How are you?"));

        let matcher = Matcher::from(regex_str);
        let matches = matcher.matches_value(&match_string);

        assert!(matches);
    }

    #[test]
    fn test_regex() {
        let re = Regex::new(r".*[Hh]ello!.*").unwrap();
        let hay = "Hello! How are you?";
        assert!(re.is_match(hay));
    }

    #[test]
    fn test_length() {
        let length_str = "$length 3";
        let match_arr = serde_json::from_str("[1, 2, 3]").unwrap();
        let matcher = Matcher::from(length_str);
        let matches = matcher.matches_value(&match_arr);

        assert!(matches);
    }
}
