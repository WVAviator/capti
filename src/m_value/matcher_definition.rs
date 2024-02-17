use std::fmt;

use colored::Colorize;
use serde::Serialize;

use crate::{
    errors::CaptiError,
    variables::{variable_map::VariableMap, SuiteVariables},
};

use super::{
    m_match::MMatch, m_value::MValue, match_context::MatchContext, matcher_map::MatcherMap,
};

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MatcherDefinition {
    match_key: String,
    args: MValue,
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
    fn matches(&self, other: &MValue) -> bool {
        if let Some(matcher) = MatcherMap::get_matcher(&self.match_key) {
            return matcher.is_match(&self.args, other);
        }

        false
    }

    fn get_context(&self, other: &MValue) -> MatchContext {
        let mut context = MatchContext::new();
        if let Some(matcher) = MatcherMap::get_matcher(&self.match_key) {
            if !matcher.is_match(&self.args, other) {
                context.push(format!(
                    "Match failed at {} matches {}",
                    &self.to_string().yellow(),
                    &other.to_string().red()
                ));
            }
        }
        context
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
                let args = serde_yaml::from_str::<MValue>(&args).unwrap_or(MValue::Null);
                return Ok(MatcherDefinition {
                    match_key: key_candidate.to_string(),
                    args,
                });
            }
        }

        return Err(());
    }
}
