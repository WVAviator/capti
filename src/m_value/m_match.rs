use std::fmt::Display;

use colored::Colorize;

use crate::progress_println;

pub trait MMatch<T = Self> {
    fn matches(&self, other: &T) -> bool;
}

impl<T> MMatch for Option<T>
where
    T: MMatch + Display,
{
    fn matches(&self, other: &Option<T>) -> bool {
        match (self, other) {
            (Some(left), Some(right)) => left.matches(right),
            (None, _) => true,
            (Some(left), None) => {
                progress_println!("Assertion failed at {} == {}", left, "None".red());
                false
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl MMatch for &str {
        fn matches(&self, other: &Self) -> bool {
            self == other
        }
    }

    #[test]
    fn matches_all_from_none() {
        let from: Option<&str> = None;
        let to = Some("hello");

        assert!(from.matches(&to));
    }

    #[test]
    fn matches_some_with_some() {
        let from = Some("hello");
        let to = Some("hello");

        assert!(from.matches(&to));
    }

    #[test]
    fn match_fails_both_some() {
        let from = Some("hello");
        let to = Some("world");

        assert!(!from.matches(&to));
    }

    #[test]
    fn match_fails_lhs_some_rhs_none() {
        let from = Some("hello");
        let to: Option<&str> = None;

        assert!(!from.matches(&to));
    }
}
