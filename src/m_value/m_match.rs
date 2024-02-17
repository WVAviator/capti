use std::fmt::Display;

use super::match_context::MatchContext;

pub trait MMatch<T = Self>: Display
where
    T: Display,
{
    fn matches(&self, other: &T) -> bool;
    fn get_context(&self, other: &T) -> MatchContext {
        let mut context = MatchContext::new();
        context.push(format!("Assertion failed at {} == {}", &self, &other));
        context
    }
}
