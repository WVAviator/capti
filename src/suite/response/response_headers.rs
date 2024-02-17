use crate::{
    errors::CaptiError,
    m_value::{m_map::Mapping, m_match::MMatch, m_value::MValue, match_context::MatchContext},
    variables::{variable_map::VariableMap, SuiteVariables},
};
use reqwest::header::HeaderMap;
use serde::Deserialize;
use std::{fmt, ops::Deref};

#[derive(Debug, PartialEq, Default, Clone, Deserialize)]
#[serde(transparent)]
pub struct ResponseHeaders(Mapping);

impl SuiteVariables for ResponseHeaders {
    fn populate_variables(&mut self, variables: &mut VariableMap) -> Result<(), CaptiError> {
        for (_, value) in self.0.iter_mut() {
            value.populate_variables(variables)?;
        }

        Ok(())
    }
}

impl MMatch for ResponseHeaders {
    fn matches(&self, other: &Self) -> bool {
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
            .collect::<Mapping>();

        lowercase_headers.matches(&other.0)
    }

    fn get_context(&self, other: &Self) -> MatchContext {
        let mut context = MatchContext::new();
        if !self.matches(other) {
            context.push("Headers do not match.".to_string());
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
                .collect::<Mapping>();
            context += lowercase_headers.get_context(&other.0);
        }
        context
    }
}

impl FromIterator<(MValue, MValue)> for ResponseHeaders {
    fn from_iter<T: IntoIterator<Item = (MValue, MValue)>>(iter: T) -> Self {
        let map = iter.into_iter().collect::<Mapping>();
        ResponseHeaders(map)
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
            .map(|(key, value)| (MValue::String(key), MValue::String(value)))
            .collect::<Mapping>();

        return ResponseHeaders(headers);
    }
}

impl Deref for ResponseHeaders {
    type Target = Mapping;
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
