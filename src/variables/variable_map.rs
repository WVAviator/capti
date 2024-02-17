use std::{collections::HashMap, ops::Deref};

use serde::Deserialize;

use regex::{escape, Captures, Regex};

use crate::{errors::CaptiError, m_value::m_value::MValue, progress_println};

// Matches continuous ${words} wrapped like ${this}
static VARIABLE_MATCHER: &str = r"\$\{(\w+)\}";

#[derive(Debug, Clone, PartialEq, Default, Deserialize)]
pub struct VariableMap(HashMap<String, MValue>);

impl VariableMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<MValue>) {
        self.0.insert(key.into(), value.into());
    }

    pub fn get(&mut self, key: &str) -> Option<MValue> {
        if let Some(value) = self.0.get(key) {
            return Some(value.clone());
        }

        let env_result = std::env::var(key);
        if let Ok(env_value) = env_result {
            let env_value = MValue::String(env_value);
            self.insert(key, env_value.clone());
            Some(env_value)
        } else {
            None
        }

        // TODO: Load env variables from .env file
    }

    pub fn replace_variables(&mut self, value: impl Into<MValue>) -> Result<MValue, CaptiError> {
        match value.into() {
            MValue::String(value) => {
                let total_regex = Regex::new(&format!("^{}$", VARIABLE_MATCHER))?;
                match total_regex.is_match(&value) {
                    true => self.replace_whole_value(&value),
                    false => self.replace_string_value(&value),
                }
            }
            other => Ok(other),
        }
    }

    pub fn replace_string_variables(&mut self, value: &str) -> Result<String, CaptiError> {
        let var_regex = Regex::new(VARIABLE_MATCHER)?;
        let result = var_regex.replace_all(value, |captures: &Captures| {
            if let Some(MValue::String(replacement_val)) = self.get(&captures[1]) {
                replacement_val
            } else {
                captures[0].to_string()
            }
        });

        Ok(result.to_string())
    }

    fn replace_string_value(&mut self, value: &str) -> Result<MValue, CaptiError> {
        let var_regex = Regex::new(VARIABLE_MATCHER)?;
        let result = var_regex.replace_all(value, |captures: &Captures| {
            if let Some(replacement_val) = self.get(&captures[1]) {
                replacement_val.to_string()
            } else {
                captures[0].to_string()
            }
        });

        let result = MValue::String(result.to_string());

        Ok(result)
    }

    fn replace_whole_value(&mut self, value: &str) -> Result<MValue, CaptiError> {
        let var_regex = Regex::new(VARIABLE_MATCHER)?;
        let result = match var_regex.captures(value) {
            Some(captures) => match captures.get(1) {
                Some(var_name) => self.get(var_name.as_str()).unwrap_or(MValue::Null),
                None => MValue::Null,
            },
            None => MValue::Null,
        };

        return Ok(result);
    }

    pub fn extract_variables(&mut self, extractor: &str, actual: &str) -> Result<(), CaptiError> {
        let var_regex = Regex::new(VARIABLE_MATCHER)?;
        let mut regex_pattern = String::from("^");
        let mut last_end = 0;

        for cap in var_regex.captures_iter(extractor) {
            let start = match cap.get(0) {
                Some(start) => start.start(),
                None => continue,
            };

            let end = match cap.get(0) {
                Some(end) => end.end(),
                None => continue,
            };

            let variable_name = match cap.get(1) {
                Some(name) => name.as_str(),
                None => continue,
            };

            regex_pattern.push_str(&escape(&extractor[last_end..start]));
            regex_pattern.push_str(&format!("(?P<{}>.+?)", variable_name));

            last_end = end;
        }

        regex_pattern.push_str(&escape(&extractor[last_end..]));
        regex_pattern.push_str("$");

        let full_regex = Regex::new(&regex_pattern)?;

        if let Some(caps) = full_regex.captures(actual) {
            for name in full_regex.capture_names().flatten() {
                if let Some(value) = caps.name(name).map(|m| m.as_str().to_string()) {
                    progress_println!("Extracted variable {}: {}", name, value);
                    self.insert(name.to_string(), MValue::String(value));
                }
            }
        }

        Ok(())
    }
}

impl Deref for VariableMap {
    type Target = HashMap<String, MValue>;
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

        let value = "Say ${HELLO} to the ${WORLD}!";
        let result = variables.replace_variables(value).unwrap();

        assert_eq!(result, MValue::String("Say hi to the universe!".into()));
    }

    #[test]
    fn extracts_variable_from_str() {
        let mut variables = VariableMap::new();

        let extractor = "The quick ${COLOR} fox jumped over the lazy dog.";
        let actual = "The quick brown fox jumped over the lazy dog.";

        variables.extract_variables(extractor, actual).unwrap();

        assert_eq!(variables["COLOR"], MValue::String("brown".into()));
    }

    #[test]
    fn extracts_multiple_vars_from_str() {
        let mut variables = VariableMap::new();

        let extractor = "The quick ${COLOR} fox jumped over the ${LAZY} dog.";
        let actual = "The quick brown fox jumped over the lazy dog.";

        variables.extract_variables(extractor, actual).unwrap();

        assert_eq!(variables["COLOR"], MValue::String("brown".into()));
        assert_eq!(variables["LAZY"], MValue::String("lazy".into()));
    }

    #[test]
    fn extracts_complex_sequences() {
        let mut variables = VariableMap::new();

        let extractor = "1111111111111${ABC}11111111111${DEF}111111";
        let actual = "1111111111111333111111111111111111111";

        variables.extract_variables(extractor, actual).unwrap();

        assert_eq!(variables["ABC"], MValue::String("333".into()));
        assert_eq!(variables["DEF"], MValue::String("1111".into()));
    }
}
