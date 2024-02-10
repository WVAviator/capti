use std::fmt;

use serde::{Deserialize, Serialize};

use super::{MatchCmp, MatchResult};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatusMatcher {
    Exact(u16),
    Class(String),
}

impl MatchCmp for StatusMatcher {
    fn match_cmp(&self, other: &Self) -> super::MatchResult {
        let other = match other {
            StatusMatcher::Exact(n) => *n,
            _ => {
                return super::MatchResult::Missing {
                    key: "Exact".to_string(),
                    context: None,
                }
            }
        };

        match self {
            StatusMatcher::Exact(expected) if expected.eq(&other) => MatchResult::Matches,
            StatusMatcher::Class(matcher)
                if match matcher.as_str() {
                    "2xx" => (200..300).contains(&other),
                    "3xx" => (300..400).contains(&other),
                    "4xx" => (400..500).contains(&other),
                    "5xx" => (500..600).contains(&other),
                    _ => false,
                } =>
            {
                MatchResult::Matches
            }
            _ => MatchResult::ValueMismatch {
                expected: format!("{}", self),
                actual: format!("{}", other),
                context: None,
            },
        }
    }
}

impl fmt::Display for StatusMatcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatusMatcher::Exact(n) => write!(f, "{}", n),
            StatusMatcher::Class(s) => match s.as_str() {
                "2xx" => write!(f, "200-299"),
                "3xx" => write!(f, "300-399"),
                "4xx" => write!(f, "400-499"),
                "5xx" => write!(f, "500-599"),
                _ => write!(f, "Invalid status range"),
            },
        }
    }
}
