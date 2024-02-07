use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigurationError {
    #[error("Error parsing YAML test suite content: {source}")]
    YamlParseError {
        #[from]
        source: serde_yaml::Error,
    },

    #[error("Error reading test suite file: {source}")]
    FilePathError {
        #[from]
        source: std::io::Error,
    },

    #[error("Error occurred parsing regex expression: {source}")]
    RegexError {
        #[from]
        source: regex::Error,
    },
}
