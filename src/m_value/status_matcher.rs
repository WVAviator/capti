use std::fmt;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum StatusMatcher {
    Exact(u16),
    Class(String),
}

impl PartialEq for StatusMatcher {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (StatusMatcher::Exact(a), StatusMatcher::Exact(b)) => a.eq(b),
            (StatusMatcher::Class(a), StatusMatcher::Class(b)) => a.eq(b),
            (StatusMatcher::Class(c), StatusMatcher::Exact(n)) => match c.as_str() {
                "2xx" => (200..300).contains(n),
                "3xx" => (300..400).contains(n),
                "4xx" => (400..500).contains(n),
                "5xx" => (500..600).contains(n),
                _ => false,
            },
            _ => false,
        }
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
                writeln!(f, "Status: {}", n)
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
