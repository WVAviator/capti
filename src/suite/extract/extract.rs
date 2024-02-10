use serde::{Deserialize, Serialize};

use crate::{
    errors::config_error::ConfigurationError,
    suite::response::{response_headers::ResponseHeaders, ResponseDefinition},
    variables::variable_map::VariableMap,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseExtractor {
    body: serde_json::Value,
    headers: Option<ResponseHeaders>,
}

impl ResponseExtractor {
    pub async fn extract(
        &self,
        response: &ResponseDefinition,
        variables: &mut VariableMap,
    ) -> Result<(), ConfigurationError> {
        let response_body = match &response.body {
            Some(body) => body,
            None => &serde_json::Value::Null,
        };

        body_extract(&self.body, response_body, variables)?;

        if let Some(headers) = &self.headers {
            for (key, value) in headers.iter() {
                match &response.headers {
                    Some(response_headers) => match response_headers.get(key) {
                        Some(header_value) => {
                            variables.extract_variables(value, header_value)?;
                        }
                        None => {
                            return Err(ConfigurationError::extract_error(format!(
                                "Missing header {} in response.",
                                &key
                            )))
                        }
                    },
                    None => {
                        return Err(ConfigurationError::extract_error(format!(
                            "Missing header {} in response.",
                            &key
                        )))
                    }
                }
            }
        }

        Ok(())
    }
}

fn body_extract(
    left: &serde_json::Value,
    right: &serde_json::Value,
    variables: &mut VariableMap,
) -> Result<(), ConfigurationError> {
    match (left, right) {
        (serde_json::Value::Null, _) => {}
        (serde_json::Value::Object(left), serde_json::Value::Object(right)) => {
            for (key, value) in left {
                match right.get(key) {
                    Some(right_value) => body_extract(value, right_value, variables)?,
                    None => {
                        return Err(ConfigurationError::extract_error(format!(
                            "Missing key {} in response body.",
                            &key
                        )))
                    }
                }
            }
        }
        (serde_json::Value::Array(left), serde_json::Value::Array(right)) => {
            for (i, value) in left.iter().enumerate() {
                body_extract(value, &right[i], variables)?;
            }
        }
        (serde_json::Value::String(left), serde_json::Value::String(right)) => {
            variables.extract_variables(left, right).map_err(|_| {
                ConfigurationError::extract_error(format!(
                    "Failed to extract variables from '{}' using matcher '{}'.",
                    &right, &left
                ))
            })?;
        }
        (left, right) => {
            return Err(ConfigurationError::extract_error(format!(
                "Variable extraction failed - cannot compare '{}' with '{}' - invalid type.",
                &left, &right
            )))
        }
    }
    Ok(())
}
