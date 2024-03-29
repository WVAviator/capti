use crate::formatting::indent::Indent;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CaptiError {
    #[error("Error parsing YAML test suite content: {source:#?}")]
    YamlParseError {
        #[from]
        source: serde_yaml::Error,
    },

    #[error("Error serializing JSON structure: {source:#?}")]
    JsonSerializeError {
        #[from]
        source: serde_json::Error,
    },

    #[error("Error reading test suite directory path: {source:#?}")]
    FilePathError {
        #[from]
        source: std::io::Error,
    },

    #[error("Error occurred parsing regex expression: {source:#?}")]
    RegexError {
        #[from]
        source: regex::Error,
    },

    #[error("Error occurred sending HTTP request: {source:#?}")]
    RequestError {
        #[from]
        source: reqwest::Error,
    },

    #[error("Extraction from response failed: {0}")]
    ExtractError(String),

    #[error("Error occurred attempting to run tests in parallel: {0}")]
    ParallelError(String),

    #[error("Error occurred setting up client for requests. Error: {source}")]
    ClientError { source: reqwest::Error },

    #[error("Error occurred parsing or setting suite variables: {0}")]
    VariableError(String),

    #[error("Unable to parse HTTP headers: {0}")]
    HTTPHeaderError(String),

    #[error("Matcher error occurred:\n{message}\n ")]
    MatcherError { message: String },
}

impl CaptiError {
    pub fn extract_error(message: impl Into<String>) -> Self {
        CaptiError::ExtractError(message.into())
    }

    pub fn parallel_error(message: impl Into<String>) -> Self {
        CaptiError::ParallelError(message.into())
    }

    pub fn matcher_error(message: impl Into<String>) -> Self {
        CaptiError::MatcherError {
            message: message.into().indent(),
        }
    }
}
