use crate::m_value::{m_value::MValue, match_processor::MatchProcessor};

/// The absent matcher returns true if the expected value is missing or null.
/// Returns false if any other kind of value is found.
pub struct Absent;

impl Absent {
    pub fn new() -> Box<Self> {
        Box::new(Absent)
    }
}

impl MatchProcessor for Absent {
    fn key(&self) -> String {
        String::from("$absent")
    }

    fn is_match(&self, _args: &MValue, value: &MValue) -> bool {
        match value {
            MValue::Null => true,
            _ => false,
        }
    }
}
