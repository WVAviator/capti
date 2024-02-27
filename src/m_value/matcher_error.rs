use thiserror::Error;

use super::m_value::MValue;

#[derive(Debug, Error)]
pub enum MatcherError {
    #[error("Invalid matcher argument: {0}")]
    InvalidArgument(String),

    #[error("Missing matcher: {0}")]
    MissingMatcher(String),

    #[error("Invalid comparison for matcher {matcher}: {value}")]
    InvalidComparison { matcher: String, value: MValue },

    #[error("Invalid value for matcher: {value}\n  {message}")]
    InvalidValue { value: MValue, message: String },

    #[error("  at {matcher}: {error}")]
    NestedMatchError { error: String, matcher: String },
}
