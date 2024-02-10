use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigurationError {
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

    #[error("Error reading test suite file: {source:#?}")]
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
}

impl ConfigurationError {
    pub fn extract_error(message: impl Into<String>) -> Self {
        ConfigurationError::ExtractError(message.into())
    }

    pub fn parallel_error(message: impl Into<String>) -> Self {
        ConfigurationError::ParallelError(message.into())
    }
}
