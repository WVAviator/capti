use crate::errors::config_error::ConfigurationError;

use super::variable_map::VariableMap;

pub trait SuiteVariables {
    fn populate_variables(&mut self, variables: &mut VariableMap)
        -> Result<(), ConfigurationError>;
}
