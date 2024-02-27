use std::fmt;

use colored::Colorize;
use serde::Serialize;

use crate::{
    errors::CaptiError,
    m_value::mvalue_wrapper::MValueWrapper,
    variables::{variable_map::VariableMap, SuiteVariables},
};

use super::{
    m_match::MMatch, m_value::MValue, match_context::MatchContext, matcher_error::MatcherError,
    matcher_map::MatcherMap,
};

/// A wrapper definition for where to find the MatchProcessor necessary to process the match. Built
/// during deserialization and handles processing matches.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MatcherDefinition {
    match_key: String,
    pub args: MValue,
}

impl Serialize for MatcherDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{} {}", self.match_key, self.args))
    }
}

impl MMatch<MValue> for MatcherDefinition {
    fn matches(&self, other: &MValue) -> Result<bool, CaptiError> {
        if let Some(matcher) = MatcherMap::get_matcher(&self.match_key) {
            let result = matcher.is_match(&self.args, other)?;
            return Ok(result);
        }

        Err(MatcherError::MissingMatcher(self.match_key.clone()).into())
    }

    fn get_context(&self, other: &MValue) -> MatchContext {
        let mut context = MatchContext::new();
        if let Some(matcher) = MatcherMap::get_matcher(&self.match_key) {
            match matcher.is_match(&self.args, other) {
                Ok(true) => {}
                Ok(false) => {
                    context.push(format!(
                        "Match failed at {} matches {}",
                        &self.to_string().yellow(),
                        &other.to_string().red()
                    ));
                }
                Err(e) => context.push(format!(
                    "Matching error at {} matches {}\n  {}",
                    &self.to_string().yellow(),
                    &other.to_string(),
                    e
                )),
            }

            if let Ok(false) = matcher.is_match(&self.args, other) {}
        }
        context
    }
}

impl Into<serde_json::Value> for MatcherDefinition {
    fn into(self) -> serde_json::Value {
        serde_json::Value::String(self.to_string())
    }
}

impl SuiteVariables for MatcherDefinition {
    fn populate_variables(&mut self, variables: &mut VariableMap) -> Result<(), CaptiError> {
        self.args.populate_variables(variables)?;
        Ok(())
    }
}

impl fmt::Display for MatcherDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.match_key, self.args)?;
        Ok(())
    }
}

impl TryFrom<&str> for MatcherDefinition {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(" ");
        if let Some(key_candidate) = parts.next() {
            if let Some(_) = MatcherMap::get_matcher(key_candidate) {
                let args = parts.map(|s| s.into()).collect::<Vec<String>>().join(" ");
                let args = MValueWrapper::from_json_value(&args);
                return Ok(MatcherDefinition {
                    match_key: key_candidate.to_string(),
                    args,
                });
            }
        }

        Err(())
    }
}
