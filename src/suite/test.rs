use std::fmt::{self, Debug};

use serde::{Deserialize, Serialize};

use crate::{
    client::Client,
    errors::config_error::ConfigurationError,
    matcher::match_result::MatchResult,
    variables::{variable_map::VariableMap, SuiteVariables},
};

use super::{extract::ResponseExtractor, request::RequestDefinition, response::ResponseDefinition};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Test {
    pub test: String,
    pub description: Option<String>,
    #[serde(default)]
    pub should_fail: bool,
    pub request: RequestDefinition,
    pub expect: ResponseDefinition,
    pub extract: Option<ResponseExtractor>,
}

impl Test {
    pub async fn execute(
        &self,
        client: &Client,
        variables: Option<&mut VariableMap>,
    ) -> Result<TestResult, ConfigurationError> {
        let request = self.request.build_client_request(&client)?;
        let response = request.send().await?;

        let response = ResponseDefinition::from_response(response).await;

        let test_result = self.expect.compare(&response);

        if let Some(extractor) = &self.extract {
            if let Some(variables) = variables {
                extractor.extract(&response, variables).await?;
            } else {
                return Err(ConfigurationError::parallel_error("Cannot extract variables from tests running in parallel. Try setting the suite to 'parallel: false'"));
            }
        }

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

impl SuiteVariables for Test {
    fn populate_variables(
        &mut self,
        variables: &mut VariableMap,
    ) -> Result<(), ConfigurationError> {
        self.request.populate_variables(variables)?;
        self.expect.populate_variables(variables)?;

        Ok(())
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

    use crate::{
        matcher::status_matcher::StatusMatcher, suite::response::response_headers::ResponseHeaders,
    };

    use super::*;

    #[test]
    fn test_compare_optional() {
        let matcher = ResponseDefinition {
            headers: None,
            body: None,
            status: None,
        };
        let response = ResponseDefinition {
            headers: Some(ResponseHeaders::default()),
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
