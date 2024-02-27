use std::{fmt, ops::Deref};

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::{
    errors::CaptiError,
    m_value::{m_map::MMap, m_match::MMatch, m_value::MValue, match_context::MatchContext},
    variables::{variable_map::VariableMap, SuiteVariables},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MHeaders(MMap);

impl SuiteVariables for MHeaders {
    fn populate_variables(&mut self, variables: &mut VariableMap) -> Result<(), CaptiError> {
        for (_, value) in self.0.iter_mut() {
            value.populate_variables(variables)?;
        }

        Ok(())
    }
}

impl MMatch for MHeaders {
    fn matches(&self, other: &Self) -> Result<bool, CaptiError> {
        let lowercase_headers = self
            .0
            .iter()
            .map(|(key, value)| {
                let key = match key {
                    MValue::String(s) => MValue::String(s.to_lowercase()),
                    other => other.clone(),
                };
                (key, value.clone())
            })
            .collect::<MMap>();

        lowercase_headers.matches(&other.0)
    }

    fn get_context(&self, other: &Self) -> MatchContext {
        let mut context = MatchContext::new();
        match self.matches(other) {
            Ok(true) => {}
            Ok(false) => {
                let lowercase_headers = self
                    .0
                    .iter()
                    .map(|(key, value)| {
                        let key = match key {
                            MValue::String(s) => MValue::String(s.to_lowercase()),
                            other => other.clone(),
                        };
                        (key, value.clone())
                    })
                    .collect::<MMap>();
                context += lowercase_headers.get_context(&other.0);
            }
            Err(e) => {
                context.push(format!("Matching error: {}", e));
            }
        }
        context
    }
}

impl TryInto<HeaderMap> for &MHeaders {
    type Error = CaptiError;
    fn try_into(self) -> Result<HeaderMap, Self::Error> {
        let mut headers = HeaderMap::new();
        for (key, value) in self.iter() {
            let (key, value) = match (key, value) {
                (MValue::String(key), MValue::String(value)) => (key, value),
                _ => {
                    return Err(CaptiError::HTTPHeaderError(format!(
                        "Invalid HTTP header:\n  {}: {}\nHeaders must be string values.\n",
                        &key, &value
                    )))
                }
            };

            let header_bytes = HeaderName::from_bytes(key.as_bytes());
            let value_bytes = HeaderValue::from_bytes(value.as_bytes());
            if let (Ok(key), Ok(value)) = (header_bytes, value_bytes) {
                headers.insert(key, value);
            } else {
                return Err(CaptiError::HTTPHeaderError(format!(
                    "Invalid HTTP header:\n  {}: {}",
                    &key, &value
                )));
            }
        }
        Ok(headers)
    }
}

impl From<&HeaderMap> for MHeaders {
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
            .map(|(key, value)| (MValue::String(key), MValue::String(value)))
            .collect::<MMap>();

        MHeaders(headers)
    }
}

impl Deref for MHeaders {
    type Target = MMap;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromIterator<(MValue, MValue)> for MHeaders {
    fn from_iter<T: IntoIterator<Item = (MValue, MValue)>>(iter: T) -> Self {
        let map = iter.into_iter().collect::<MMap>();
        MHeaders(map)
    }
}

impl fmt::Display for MHeaders {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (key, value) in self.0.iter() {
            writeln!(f, "▹ {}: {}", key, value)?;
        }
        Ok(())
    }
}

impl Default for MHeaders {
    fn default() -> Self {
        MHeaders(MMap::new())
    }
}
