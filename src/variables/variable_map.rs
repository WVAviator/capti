use std::{collections::HashMap, ops::Deref};

use serde::Deserialize;

use regex::Captures;

use crate::errors::config_error::ConfigurationError;

#[derive(Debug, Clone, PartialEq, Default, Deserialize)]
pub struct VariableMap(HashMap<String, String>);

impl VariableMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.0.insert(key.into(), value.into());
    }

    pub fn get(&mut self, key: &str) -> Option<String> {
        if let Some(value) = self.0.get(key) {
            return Some(value.clone());
        }

        let env_result = std::env::var(key);
        if let Ok(env_value) = env_result {
            self.insert(key, env_value.clone());
            Some(env_value)
        } else {
            None
        }

        // TODO: Load env variables from .env file
    }

    // pub fn get(&self, key: &str) -> Option<&String> {
    //     self.0.get(key)
    // }

    pub fn replace_variables(&mut self, value: &str) -> Result<String, ConfigurationError> {
        // Matches continuous word characters inside curly braces
        let var_regex = regex::Regex::new(r"\{(\w+)\}")?;

        let result = var_regex.replace_all(value, |captures: &Captures| {
            if let Some(replacement_val) = self.get(&captures[1]) {
                replacement_val.to_string()
            } else {
                captures[0].to_string()
            }
        });

        Ok(result.to_string())
    }
}

impl Deref for VariableMap {
    type Target = HashMap<String, String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn replaces_variables_in_str() {
        let mut variables = VariableMap::new();
        variables.insert("HELLO", "hi");
        variables.insert("WORLD", "universe");

        let value = "Say {HELLO} to the {WORLD}!";
        let result = variables.replace_variables(value).unwrap();

        assert_eq!(result, "Say hi to the universe!");
    }
}
