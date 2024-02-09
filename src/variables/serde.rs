use crate::errors::config_error::ConfigurationError;

use super::{variable_map::VariableMap, SuiteVariables};

impl SuiteVariables for serde_json::Value {
    fn populate_variables(
        &mut self,
        variables: &mut VariableMap,
    ) -> Result<(), ConfigurationError> {
        match self {
            serde_json::Value::Object(map) => {
                for (_, value) in map.iter_mut() {
                    value.populate_variables(variables)?;
                }
            }
            serde_json::Value::Array(vec) => {
                for value in vec.iter_mut() {
                    value.populate_variables(variables)?;
                }
            }
            serde_json::Value::String(s) => {
                let replaced = variables.replace_variables(s)?;
                *self = serde_json::Value::String(replaced);
            }
            _ => {}
        }

        Ok(())
    }
}
