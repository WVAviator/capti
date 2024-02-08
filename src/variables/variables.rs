use crate::errors::config_error::ConfigurationError;

use super::variable_map::VariableMap;

pub trait SuiteVariables {
    fn populate_variables(&mut self, variables: &mut VariableMap)
        -> Result<(), ConfigurationError>;
}

impl SuiteVariables for Option<T> where T: SuiteVariables {
    fn populate_variables(&mut self, variables: &mut VariableMap) -> Result<(), ConfigurationError> {
        match self {
            Some(value) => value.populate_variables(variables),
            None => Ok(())
        }
    }
}
