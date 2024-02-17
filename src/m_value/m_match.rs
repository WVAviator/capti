use std::fmt::Display;

use super::match_context::MatchContext;

pub trait MMatch<T = Self>: Display
where
    T: Display,
{
    fn matches(&self, other: &T) -> bool;
    fn get_context(&self, other: &T) -> MatchContext;
}
