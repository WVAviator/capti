use crate::{
    errors::CaptiError,
    matcher::{MatchCmp, MatchResult},
    variables::{variable_map::VariableMap, SuiteVariables},
};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, ops::Deref};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ResponseHeaders(HashMap<String, String>);

impl SuiteVariables for ResponseHeaders {
    fn populate_variables(&mut self, variables: &mut VariableMap) -> Result<(), CaptiError> {
        for (_, value) in self.0.iter_mut() {
            *value = variables.replace_variables(value.as_str())?;
        }

        Ok(())
    }
}

impl MatchCmp for ResponseHeaders {
    fn match_cmp(&self, other: &Self) -> MatchResult {
        let lowercase_headers = self
            .0
            .iter()
            .map(|(key, value)| (key.to_lowercase(), value.clone()))
            .collect::<HashMap<String, String>>();

        lowercase_headers.match_cmp(&other.0)
    }
}

impl From<&HeaderMap> for ResponseHeaders {
    fn from(header_map: &HeaderMap) -> Self {
        let header_map = header_map.clone();
        let headers = header_map
            .into_iter()
            .filter_map(|(header, value)| match header {
                Some(header) => Some((header, value)),
                None => None,
            })
            .filter_map(|(header, value)| {
                let header = header.to_string();
                let value = match value.to_str() {
                    Ok(value) => value,
                    Err(_) => {
                        eprintln!("Failed to convert header value to string.");
                        return None;
                    }
                };

                Some((header, value.to_string()))
            })
            .collect::<HashMap<String, String>>();

        return ResponseHeaders(headers);
    }
}

impl Deref for ResponseHeaders {
    type Target = HashMap<String, String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for ResponseHeaders {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Headers:")?;
        for (key, value) in self.0.iter() {
            writeln!(f, "    ▹ {}: {}", key, value)?;
        }
        Ok(())
    }
}
