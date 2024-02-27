use crate::m_value::{
    m_value::MValue, match_processor::MatchProcessor, matcher_error::MatcherError,
};

/// The $exists matcher returns true if something exists at the expected value.
/// Returns false if no value is found or if the value is null.
pub struct Exists;

impl Exists {
    pub fn new() -> Box<Self> {
        Box::new(Exists)
    }
}

impl MatchProcessor for Exists {
    fn key(&self) -> String {
        String::from("$exists")
    }

    fn is_match(&self, _args: &MValue, value: &MValue) -> Result<bool, MatcherError> {
        match value {
            MValue::Null => Ok(false),
            _ => Ok(true),
        }
    }
}
