use crate::m_value::{m_match::MMatch, m_value::MValue, match_processor::MatchProcessor};

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

    fn is_match(&self, args: &MValue, value: &MValue) -> bool {
        match args.matches(value) {
            true => false,
            false => true,
        }
    }
}
