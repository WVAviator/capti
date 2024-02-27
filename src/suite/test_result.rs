use std::fmt;

use colored::Colorize;

use crate::m_value::match_context::MatchContext;

use super::failure_report::FailureReport;

#[derive(Debug, Clone, PartialEq)]
pub enum TestResult {
    Passed,
    Failed(FailureReport),
}

impl TestResult {
    pub fn fail(message: impl Into<String>, context: MatchContext) -> Self {
        TestResult::Failed(FailureReport::new(message, context))
    }
}

impl fmt::Display for TestResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestResult::Passed => write!(f, "{}", "[OK]".green()),
            TestResult::Failed(_) => write!(f, "{}", "[FAILED]".red()),
        }
    }
}

impl Into<String> for &TestResult {
    fn into(self) -> String {
        format!("{}", self)
    }
}
