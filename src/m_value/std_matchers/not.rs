use crate::m_value::{
    m_match::MMatch, m_value::MValue, match_processor::MatchProcessor, matcher_error::MatcherError,
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

    fn is_match(&self, args: &MValue, value: &MValue) -> Result<bool, MatcherError> {
        match args.matches(value) {
            Ok(true) => Ok(false),
            Ok(false) => Ok(true),
            Err(e) => Err(MatcherError::NestedMatchError {
                error: e.to_string(),
                matcher: self.key(),
            }),
        }
    }
}
