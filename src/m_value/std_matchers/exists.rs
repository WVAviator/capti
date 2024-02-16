use crate::m_value::{m_value::MValue, match_processor::MatchProcessor};

#[derive(Default)]
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

    fn is_match(&self, _args: &MValue, value: &MValue) -> bool {
        match value {
            MValue::Null => false,
            _ => true,
        }
    }
}
