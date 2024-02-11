use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    errors::CaptiError,
    matcher::{match_result::MatchResult, status_matcher::StatusMatcher, MatchCmp},
    suite::test::TestResult,
    variables::{variable_map::VariableMap, SuiteVariables},
};

use super::response_headers::ResponseHeaders;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseDefinition {
    pub status: Option<StatusMatcher>,
    pub headers: Option<ResponseHeaders>,
    pub body: Option<serde_json::Value>,
}

impl ResponseDefinition {
    pub async fn from_response(response: reqwest::Response) -> Self {
        let status = Some(StatusMatcher::Exact(response.status().as_u16()));

        let headers = ResponseHeaders::from(response.headers());
        let headers = match headers.len() {
            0 => None,
            _ => Some(headers),
        };

        let body = match response.json::<serde_json::Value>().await {
            Ok(body) => Some(body),
            Err(_) => None,
        };

        ResponseDefinition {
            status,
            headers,
            body,
        }
    }

    pub fn compare(&self, other: &ResponseDefinition) -> TestResult {
        match self.status.match_cmp(&other.status) {
            MatchResult::Matches => {}
            other => return TestResult::fail("Status does not match.", &other),
        }

        match self.headers.match_cmp(&other.headers) {
            MatchResult::Matches => {}
            other => return TestResult::fail("Headers do not match.", &other),
        }

        match self.body.match_cmp(&other.body) {
            MatchResult::Matches => {}
            other => return TestResult::fail("Body does not match.", &other),
        }

        return TestResult::Passed;
    }
}

impl SuiteVariables for ResponseDefinition {
    fn populate_variables(&mut self, variables: &mut VariableMap) -> Result<(), CaptiError> {
        self.headers.populate_variables(variables)?;
        self.body.populate_variables(variables)?;

        Ok(())
    }
}

impl fmt::Display for ResponseDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, " ")?;

        if let Some(status) = &self.status {
            writeln!(f, "  {}", status)?;
        }

        if let Some(headers) = &self.headers {
            writeln!(f, "  {}", headers)?;
        }

        if let Some(body) = &self.body {
            if let Ok(json) = serde_json::to_string_pretty(&body) {
                writeln!(f, "  Body:")?;
                for line in json.lines() {
                    writeln!(f, "    {}", line)?;
                }
            }
        }

        writeln!(f, " ")?;

        Ok(())
    }
}
