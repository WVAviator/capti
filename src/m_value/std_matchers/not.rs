use crate::{
    errors::CaptiError,
    formatting::indent::Indent,
    m_value::{m_match::MMatch, m_value::MValue, match_processor::MatchProcessor},
};

// The $not matcher matches the opposite of the given arguments
pub struct Not;

impl Not {
    pub fn new() -> Box<Self> {
        Box::new(Not)
    }
}

impl MatchProcessor for Not {
    fn key(&self) -> String {
        String::from("$not")
    }

    fn is_match(&self, args: &MValue, value: &MValue) -> Result<bool, CaptiError> {
        match args.matches(value) {
            Ok(true) => Ok(false),
            Ok(false) => Ok(true),
            Err(e) => Err(CaptiError::matcher_error(format!(
                "Cannot process $not matcher due to argument error:\n{}",
                e.to_string().indent()
            ))),
        }
    }
}
