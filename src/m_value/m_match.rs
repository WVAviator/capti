use std::fmt::Display;

use crate::errors::CaptiError;

use super::match_context::MatchContext;

// The MMatch trait allows values to be compared using the 'matches' method and any context about
// the matching can be provided via the 'get_context' method.
pub trait MMatch<T = Self>: Display
where
    T: Display,
{
    fn matches(&self, other: &T) -> Result<bool, CaptiError>;
    fn get_context(&self, other: &T) -> MatchContext;
}
