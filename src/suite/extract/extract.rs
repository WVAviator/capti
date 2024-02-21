use serde::Deserialize;

use crate::{
    errors::CaptiError,
    m_value::m_value::MValue,
    suite::response::{response_headers::ResponseHeaders, ResponseDefinition},
    variables::variable_map::VariableMap,
};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ResponseExtractor {
    body: MValue,
    #[serde(default)]
    headers: ResponseHeaders,
}

impl ResponseExtractor {
    pub async fn extract(
        &self,
        response: &ResponseDefinition,
        variables: &mut VariableMap,
    ) -> Result<(), CaptiError> {
        body_extract(&self.body, &response.body, variables)?;

        for (key, value) in self.headers.iter() {
            let value = match value {
                MValue::String(s) => s,
                _ => {
                    return Err(CaptiError::extract_error(format!(
                        "Invalid value for header {} in response.",
                        &key
                    )))
                }
            };
            match &response.headers.get(key) {
                Some(MValue::String(header_value)) => {
                    variables.extract_variables(value, header_value)?;
                }
                _ => {
                    return Err(CaptiError::extract_error(format!(
                        "Missing header {} in response.",
                        &key
                    )))
                }
            }
        }

        Ok(())
    }
}

fn body_extract(
    left: &MValue,
    right: &MValue,
    variables: &mut VariableMap,
) -> Result<(), CaptiError> {
    match (left, right) {
        (MValue::Null, _) => {}
        (MValue::Mapping(left), MValue::Mapping(right)) => {
            for (key, value) in left.iter() {
                match right.get(key) {
                    Some(right_value) => body_extract(value, right_value, variables)?,
                    None => {
                        return Err(CaptiError::extract_error(format!(
                            "Missing key {} in response body.",
                            &key
                        )))
                    }
                }
            }
        }
        (MValue::Sequence(left), MValue::Sequence(right)) => {
            for (i, value) in left.iter().enumerate() {
                body_extract(value, &right[i], variables)?;
            }
        }
        (MValue::String(left), MValue::String(right)) => {
            variables.extract_variables(left, right).map_err(|_| {
                CaptiError::extract_error(format!(
                    "Failed to extract variables from '{}' using matcher '{}'.",
                    &right, &left
                ))
            })?;
        }
        (left, right) => {
            return Err(CaptiError::extract_error(format!(
                "Variable extraction failed - cannot compare '{}' with '{}' - invalid type.",
                &left, &right
            )))
        }
    }
    Ok(())
}
