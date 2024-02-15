use super::m_value::MValue;

pub trait MatchProcessor {
    fn is_match(&self, args: MValue, value: MValue) -> bool;
}
