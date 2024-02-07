use std::{collections::HashMap, ops::Deref};

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::{
    errors::config_error::ConfigurationError,
    variables::{variable_map::VariableMap, SuiteVariables},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RequestHeaders(HashMap<String, String>);

impl SuiteVariables for RequestHeaders {
    fn populate_variables(
        &mut self,
        variables: &mut VariableMap,
    ) -> Result<(), ConfigurationError> {
        for (_, value) in self.0.iter_mut() {
            *value = variables.replace_variables(value.as_str())?;
        }

        Ok(())
    }
}

impl Deref for RequestHeaders {
    type Target = HashMap<String, String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<reqwest::header::HeaderMap> for RequestHeaders {
    fn into(self) -> reqwest::header::HeaderMap {
        let mut headers = HeaderMap::new();
        for (key, value) in self.iter() {
            headers.insert(
                HeaderName::from_bytes(key.as_bytes()).unwrap(),
                HeaderValue::from_bytes(value.as_bytes()).unwrap(),
            );
        }
        headers
    }
}
