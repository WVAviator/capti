use crate::m_value::{m_value::MValue, match_processor::MatchProcessor};

/// the $empty matcher checks to see if the provided value is empty.
/// Works with arrays, objects, and strings. Does not match null (use $absent for null checks).
/// Returns true if the given value effectively has a length of 0, false if not.
pub struct Empty;

impl Empty {
    pub fn new() -> Box<Self> {
        Box::new(Empty)
    }
}

impl MatchProcessor for Empty {
    fn key(&self) -> String {
        String::from("$empty")
    }

    fn is_match(&self, _args: &MValue, value: &MValue) -> bool {
        match value {
            MValue::Sequence(arr) => arr.is_empty(),
            MValue::String(s) => s.is_empty(),
            MValue::Mapping(map) => map.is_empty(),
            _ => false,
        }
    }
}
