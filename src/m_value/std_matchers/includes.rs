use crate::m_value::{m_match::MMatch, m_value::MValue, match_processor::MatchProcessor};

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

    fn is_match(&self, args: &MValue, value: &MValue) -> bool {
        match value {
            MValue::Sequence(arr) => arr.iter().any(|i| args.matches(i)),
            _ => false,
        }
    }
}
