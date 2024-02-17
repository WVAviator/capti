use std::ops::Deref;

use regex::Regex;

// Matches continuous ${words} wrapped like ${this}
pub static VARIABLE_MATCHER: &str = r"\$\{(\w+)\}";

#[derive(Debug, Clone)]
pub struct VarRegex(Regex);

impl Deref for VarRegex {
    type Target = Regex;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for VarRegex {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Default for VarRegex {
    fn default() -> Self {
        VarRegex(Regex::new(VARIABLE_MATCHER).expect("Invalid regex matcher."))
    }
}
