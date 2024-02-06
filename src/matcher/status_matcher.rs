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
                expected: format!("{:#?}", self),
                actual: other.to_string(),
                context: None,
            },
        }
    }
}
