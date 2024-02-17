use super::m_value::MValue;

/// The MatchProcessor trait must be implemented by a struct to handle custom matching. Every
/// matcher is a MatchProcessor.
pub trait MatchProcessor: Send + Sync {
    fn key(&self) -> String;
    fn is_match(&self, args: &MValue, value: &MValue) -> bool;
}
