use serde::{Serialize, Deserialize};
use crate::{
    errors::config_error::ConfigurationError,
    variables::{variable_map::VariableMap, SuiteVariables},
};
use reqwest::header::HeaderMap;
use std::{collections::HashMap, ops::Deref};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ResponseHeaders(HashMap<String, String>);

impl SuiteVariables for ResponseHeaders {
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

impl From<&HeaderMap> for ResponseHeaders {
  fn from(header_map: &HeaderMap) -> Self {
      let headers = header_map
          .iter()
          .filter_map(|(header, value)| match header {
              Some(header) => Some((header, value)),
              None => None,
          })
          .map(|(header, value)| {
              (
                  header.as_str().to_string(),
                  value.to_str().unwrap().to_string(), // TODO: Better way?
              )
          })
          .collect::<HashMap<String, String>>()
      
      return ResponseHeaders(headers);
  }
}

impl Deref for RequestHeaders {
    type Target = HashMap<String, String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}