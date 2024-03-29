use std::{fmt, ops::Deref};

use serde::Deserialize;

use crate::{
    errors::CaptiError,
    m_value::{m_match::MMatch, match_context::MatchContext, status_matcher::StatusMatcher},
};

#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct Status(Option<StatusMatcher>);

impl Status {
    pub fn none() -> Self {
        Status(None)
    }
}

impl MMatch for Status {
    fn matches(&self, other: &Self) -> Result<bool, CaptiError> {
        match (&self.0, &other.0) {
            (Some(a), Some(b)) => a.matches(b),
            (None, _) => Ok(true),
            (_, None) => Ok(false),
        }
    }
    fn get_context(&self, other: &Self) -> MatchContext {
        match (&self.0, &other.0) {
            (Some(a), Some(b)) => a.get_context(b),
            (None, _) => MatchContext::new(),
            (_, None) => MatchContext::new(),
        }
    }
}

impl Deref for Status {
    type Target = Option<StatusMatcher>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for Status {
    fn eq(&self, other: &Status) -> bool {
        match (&self.0, &other.0) {
            (Some(a), Some(b)) => a.eq(b),
            (None, _) => true,
            (_, None) => false,
        }
    }
}

impl From<&str> for Status {
    fn from(value: &str) -> Self {
        match value {
            "2xx" => Status(Some(StatusMatcher::Class(String::from("2xx")))),
            "3xx" => Status(Some(StatusMatcher::Class(String::from("3xx")))),
            "4xx" => Status(Some(StatusMatcher::Class(String::from("4xx")))),
            "5xx" => Status(Some(StatusMatcher::Class(String::from("5xx")))),
            _ => Status(None),
        }
    }
}

impl From<StatusMatcher> for Status {
    fn from(status: StatusMatcher) -> Self {
        Status(Some(status))
    }
}

impl From<u16> for Status {
    fn from(status: u16) -> Self {
        Status(Some(StatusMatcher::Exact(status)))
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(status) => write!(f, "{}", status),
            None => write!(f, "None"),
        }
    }
}
