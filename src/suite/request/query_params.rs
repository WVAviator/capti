use std::{collections::HashMap, fmt, ops::Deref};

use serde::{Deserialize, Serialize};

use crate::{
    errors::CaptiError,
    m_value::m_value::MValue,
    variables::{variable_map::VariableMap, SuiteVariables},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryParams(HashMap<String, String>);

impl Default for QueryParams {
    fn default() -> Self {
        QueryParams(HashMap::new())
    }
}

impl SuiteVariables for QueryParams {
    fn populate_variables(&mut self, variables: &mut VariableMap) -> Result<(), CaptiError> {
        for (key, value) in self.0.iter_mut() {
            match variables.replace_variables(value.clone()) {
                Ok(MValue::String(s)) => *value = s,
                _ => {
                    return Err(CaptiError::VariableError(format!(
                        "Unable to interpret query param {} as string.",
                        key
                    )))
                }
            }
        }
        Ok(())
    }
}

impl Deref for QueryParams {
    type Target = HashMap<String, String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl QueryParams {
    pub fn as_query_string(&self) -> String {
        if self.is_empty() {
            return String::new();
        }

        let mut query_string = String::from("?");
        for (key, value) in self.iter() {
            query_string.push_str(&format!("{}={}&", key, value));
        }

        query_string.pop(); // remove last '&'

        query_string
    }
}

impl fmt::Display for QueryParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        writeln!(f, "Params:")?;

        for (key, value) in self.iter() {
            writeln!(f, "  {}: {}", key, value)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn properly_formats_appended() {
        let url = "http://example.com";

        let mut query_params = QueryParams::default();
        query_params
            .0
            .insert("key".to_string(), "value".to_string());
        query_params
            .0
            .insert("key2".to_string(), "value2".to_string());

        let expected_possibilities = vec![
            "http://example.com?key=value&key2=value2",
            "http://example.com?key2=value2&key=value",
        ];

        let actual = format!("{}{}", url, &query_params.as_query_string());
        let actual = actual.as_str();

        assert!(expected_possibilities.contains(&actual));
    }
}
