use std::fmt;

use colored::Colorize;

use crate::m_value::match_context::MatchContext;

#[derive(Debug, Clone, PartialEq)]
pub struct FailureReport {
    message: String,
    match_context: MatchContext,
}

impl FailureReport {
    pub fn new(message: impl Into<String>, match_context: MatchContext) -> Self {
        FailureReport {
            message: format!("{} {}", "→".red(), message.into()),
            match_context,
        }
    }
}

impl fmt::Display for FailureReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.message)?;
        writeln!(f, "{}", self.match_context)?;

        Ok(())
    }
}
