use std::{collections::VecDeque, fmt, ops::AddAssign};

/// Provides context for a mismatch between two `MValue` instances.
/// Used to print information for the user.
#[derive(Debug, Clone, PartialEq)]
pub struct MatchContext(VecDeque<String>);

impl MatchContext {
    pub fn new() -> Self {
        MatchContext(VecDeque::new())
    }

    pub fn push(&mut self, context: impl Into<String>) {
        self.0.push_back(context.into());
    }
}

impl AddAssign for MatchContext {
    fn add_assign(&mut self, mut other: Self) {
        while let Some(item) = other.0.pop_back() {
            self.0.push_front(item);
        }
    }
}

impl fmt::Display for MatchContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for context in &self.0 {
            writeln!(f, "  {}", context)?;
        }
        Ok(())
    }
}
