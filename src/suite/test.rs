use std::{
    collections::HashMap,
    fmt::{self, Debug},
};

use serde::{Deserialize, Serialize};

use crate::{
    client::client::get_client,
    errors::config_error::ConfigurationError,
    matcher::{match_result::MatchResult, status_matcher::StatusMatcher, MatchCmp},
};

use super::{response::ResponseDefinition, request::RequestDefinition};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Test {
    pub test: String,
    pub description: Option<String>,
    #[serde(default)]
    pub should_fail: bool,
    pub request: RequestDefinition,
    pub expect: ResponseDefinition,
}

impl Test {
    pub async fn execute(&self) -> Result<TestResult, ConfigurationError> {
        let client = get_client();

        let request = self.request.build_client_request(&client)?;
        let response = request.send().await?;

        let response = ResponseDefinition::from_response(response).await;

        let test_result = self.expect.compare(&response);

        let test_result = match (test_result, self.should_fail) {
            (TestResult::Passed, true) => TestResult::Failed(FailureReport::new(
                "Expected failure, but test passed.",
                MatchResult::Matches,
            )),
            (TestResult::Failed(_), true) => TestResult::Passed,
            (result, _) => result,
        };

        return Ok(test_result);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestResult {
    Passed,
    Failed(FailureReport),
}

impl TestResult {
    pub fn fail(message: impl Into<String>, match_result: &MatchResult) -> Self {
        TestResult::Failed(FailureReport::new(message, match_result.clone()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailureReport {
    message: String,
    match_result: MatchResult,
}

impl FailureReport {
    pub fn new(message: impl Into<String>, match_result: MatchResult) -> Self {
        FailureReport {
            message: message.into(),
            match_result,
        }
    }
}

impl fmt::Display for FailureReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.message)?;
        writeln!(f, "{}", self.match_result)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_compare_optional() {
        let matcher = ResponseDefinition {
            headers: None,
            body: None,
            status: None,
        };
        let response = ResponseDefinition {
            headers: Some(HashMap::new()),
            body: Some(json!({ "test": "test" })),
            status: Some(StatusMatcher::Exact(200)),
        };

        assert_eq!(matcher.compare(&response), TestResult::Passed);
    }

    #[test]
    fn test_compare_status_matches() {
        let matcher = ResponseDefinition {
            headers: None,
            body: None,
            status: Some(StatusMatcher::Class(String::from("2xx"))),
        };
        let response = ResponseDefinition {
            headers: None,
            body: None,
            status: Some(StatusMatcher::Exact(200)),
        };

        assert_eq!(matcher.compare(&response), TestResult::Passed);
    }
}
