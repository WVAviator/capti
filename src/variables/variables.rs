use crate::errors::CaptiError;

use super::variable_map::VariableMap;

pub trait SuiteVariables {
    fn populate_variables(&mut self, variables: &mut VariableMap) -> Result<(), CaptiError>;
}

impl<T> SuiteVariables for Option<T>
where
    T: SuiteVariables,
{
    fn populate_variables(&mut self, variables: &mut VariableMap) -> Result<(), CaptiError> {
        match self {
            Some(value) => value.populate_variables(variables),
            None => Ok(()),
        }
    }
}
