use std::{fs, ops::Deref, path::PathBuf};

use colored::Colorize;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{errors::CaptiError, progress_println};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct Environment {
    path: Option<PathBuf>,
    #[serde(skip)]
    env: IndexMap<String, String>,
}

impl Environment {
    pub fn load(&mut self) -> Result<(), CaptiError> {
        match &self.path {
            Some(path) => {
                let env_contents = fs::read_to_string(path)
                    .map_err(|e| CaptiError::FilePathError { source: e })?;

                progress_println!("Loading environment variables from {:?}", &path);

                self.parse_contents(env_contents);

                Ok(())
            }
            None => Ok(()),
        }
    }

    fn parse_contents(&mut self, contents: String) {
        self.env = contents
            .split('\n')
            .filter_map(|kv_pair| {
                if kv_pair.is_empty() {
                    return None;
                }

                let kv = kv_pair.split("=").collect::<Vec<&str>>();
                if kv.len() != 2 {
                    progress_println!(
                        "{}: Invalid key/value pair in env file:\n  {}",
                        "ERROR".red(),
                        kv_pair
                    );
                    return None;
                }

                let key = kv[0].trim().to_string();
                let value = kv[1].trim().to_string();

                let value = match value {
                    v if v.starts_with("\"") && v.ends_with("\"") => v[1..v.len() - 1].to_string(),
                    v if v.starts_with("'") && v.ends_with("'") => v[1..v.len() - 1].to_string(),
                    _ => value,
                };

                Some((key, value))
            })
            .collect::<IndexMap<String, String>>();
    }
}

impl Default for Environment {
    fn default() -> Environment {
        Environment {
            path: None,
            env: IndexMap::new(),
        }
    }
}

impl Deref for Environment {
    type Target = IndexMap<String, String>;
    fn deref(&self) -> &Self::Target {
        &self.env
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn properly_parses_env_file_string() {
        let mut env = Environment::default();

        let contents = "ABC=123\nXYZ=456".to_string();
        env.parse_contents(contents);

        assert_eq!(env.get("ABC"), Some(&"123".to_string()));
        assert_eq!(env.get("XYZ"), Some(&"456".to_string()));
    }

    #[test]
    fn ignores_quotes_in_values() {
        let mut env = Environment::default();
        let contents = "ABC=\"123\"\nXYZ='456'".to_string();
        env.parse_contents(contents);
        assert_eq!(env.get("ABC"), Some(&"123".to_string()));
        assert_eq!(env.get("XYZ"), Some(&"456".to_string()));
    }
}
