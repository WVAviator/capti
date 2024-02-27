use std::fmt;

use colored::Colorize;
use serde::Deserialize;

use crate::errors::CaptiError;

use super::{m_match::MMatch, match_context::MatchContext};

/// A special matcher specifically for statuses only. Statuses have different matching rules than
/// MValues.
#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(untagged)]
pub enum StatusMatcher {
    Exact(u16),
    Class(String),
}

impl MMatch for StatusMatcher {
    fn matches(&self, other: &Self) -> Result<bool, CaptiError> {
        match (self, other) {
            (StatusMatcher::Exact(a), StatusMatcher::Exact(b)) => Ok(a.eq(b)),
            (StatusMatcher::Class(a), StatusMatcher::Class(b)) => Ok(a.eq(b)),
            (StatusMatcher::Class(c), StatusMatcher::Exact(n)) => match c.as_str() {
                "2xx" => Ok((200..300).contains(n)),
                "3xx" => Ok((300..400).contains(n)),
                "4xx" => Ok((400..500).contains(n)),
                "5xx" => Ok((500..600).contains(n)),
                _ => Err(CaptiError::matcher_error(format!("Invalid status matcher pattern {}.\nMust be one of '2xx', '3xx', '4xx', or '5xx', or must be the exact status as a number (404, 201, 303, etc)", c.red()))),
            },
            _ => Ok(false),
        }
    }

    fn get_context(&self, other: &Self) -> MatchContext {
        let mut context = MatchContext::new();
        match self.matches(other) {
            Ok(true) => {},
            Ok(false) => {
                context.push(format!(
                    "Mismatch at response status:\n    expected: {}\n    found: {}",
                    &self.to_string().yellow(),
                    &other.to_string().red()
                ));
            }
            Err(e) => context.push(format!(
                "Matching error at response status:\n    expected: {}\n    found: {}\n    error: {}",
                &self.to_string().yellow(),
                &other.to_string(),
                e
            )),
        }
        context
    }
}

impl From<reqwest::StatusCode> for StatusMatcher {
    fn from(status: reqwest::StatusCode) -> Self {
        StatusMatcher::Exact(status.as_u16())
    }
}

impl fmt::Display for StatusMatcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatusMatcher::Exact(n) => {
                write!(f, "{}", n)
            }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn matches_exact() {
        let matcher = StatusMatcher::Exact(200);
        let other = StatusMatcher::Exact(200);
        assert!(matcher.matches(&other).unwrap());
    }

    #[test]
    fn match_fails_exact() {
        let matcher = StatusMatcher::Exact(200);
        let other = StatusMatcher::Exact(201);
        assert!(!matcher.matches(&other).unwrap());
    }

    #[test]
    fn matches_range() {
        let matcher = StatusMatcher::Class("2xx".to_string());
        let other = StatusMatcher::Exact(200);
        let other2 = StatusMatcher::Exact(250);
        let other3 = StatusMatcher::Exact(299);
        assert!(matcher.matches(&other).unwrap());
        assert!(matcher.matches(&other2).unwrap());
        assert!(matcher.matches(&other3).unwrap());
    }

    #[test]
    fn match_fails_range() {
        let matcher = StatusMatcher::Class("2xx".to_string());
        let other = StatusMatcher::Exact(300);
        let other2 = StatusMatcher::Exact(199);
        assert!(!matcher.matches(&other).unwrap());
        assert!(!matcher.matches(&other2).unwrap());
    }
}
