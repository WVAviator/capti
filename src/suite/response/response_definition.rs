use std::fmt;

use serde::Deserialize;

use crate::{
    errors::CaptiError,
    m_value::{m_value::MValue, status_matcher::StatusMatcher},
    suite::test::TestResult,
    variables::{variable_map::VariableMap, SuiteVariables},
};

use super::{response_headers::ResponseHeaders, status::Status};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ResponseDefinition {
    pub status: Status,
    #[serde(default)]
    pub headers: ResponseHeaders,
    #[serde(default)]
    pub body: MValue,
}

impl ResponseDefinition {
    pub async fn from_response(response: reqwest::Response) -> Self {
        let status = Status::from(StatusMatcher::Exact(response.status().as_u16()));

        let headers = ResponseHeaders::from(response.headers());

        let body = match response.json::<MValue>().await {
            Ok(body) => body,
            Err(_) => MValue::Null,
        };

        ResponseDefinition {
            status,
            headers,
            body,
        }
    }

    pub fn compare(&self, other: &ResponseDefinition) -> TestResult {
        if !self.status.eq(&other.status) {
            return TestResult::fail("Status does not match.");
        }

        if !self.headers.eq(&other.headers) {
            return TestResult::fail("Headers do not match.");
        }

        if !self.body.eq(&other.body) {
            return TestResult::fail("Body does not match.");
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

        writeln!(f, "  {}", self.status)?;

        writeln!(f, "  {}", self.headers)?;

        if let Ok(json) = serde_json::to_string_pretty(&self.body) {
            writeln!(f, "  Body:")?;
            for line in json.lines() {
                writeln!(f, "    {}", line)?;
            }
        }

        writeln!(f, " ")?;

        Ok(())
    }
}
