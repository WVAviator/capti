use crate::m_value::{
    m_match::MMatch, m_value::MValue, match_processor::MatchProcessor, matcher_error::MatcherError,
};

/// The $includes matcher checks an array to see if the provided value is included.
/// Returns true if a matching (following standard matching rules) value is found in the array.
/// Returns false if no match is found.
pub struct Includes;

impl Includes {
    pub fn new() -> Box<Self> {
        Box::new(Includes)
    }
}

impl MatchProcessor for Includes {
    fn key(&self) -> String {
        String::from("$includes")
    }

    fn is_match(&self, args: &MValue, value: &MValue) -> Result<bool, MatcherError> {
        match value {
            MValue::Sequence(arr) => {
                let matches = arr.iter().any(|i| match args.matches(i) {
                    Ok(true) => true,
                    _ => false,
                });
                Ok(matches)
            }
            _ => Err(MatcherError::InvalidComparison {
                matcher: self.key(),
                value: value.clone(),
            }),
        }
    }
}
