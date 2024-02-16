use super::m_value::MValue;

pub trait MatchProcessor: Send + Sync {
    fn key(&self) -> String;
    fn is_match(&self, args: &MValue, value: &MValue) -> bool;
}
