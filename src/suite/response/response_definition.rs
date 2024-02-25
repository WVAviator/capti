use std::fmt;

use serde::Deserialize;

use crate::{
    errors::CaptiError,
    m_value::{m_match::MMatch, m_value::MValue, status_matcher::StatusMatcher},
    suite::{headers::MHeaders, test::TestResult},
    variables::{variable_map::VariableMap, SuiteVariables},
};

use super::status::Status;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ResponseDefinition {
    pub status: Status,
    #[serde(default)]
    pub headers: MHeaders,
    #[serde(default)]
    pub body: MValue,
}

impl ResponseDefinition {
    pub async fn from_response(response: reqwest::Response) -> Self {
        let status = Status::from(StatusMatcher::Exact(response.status().as_u16()));

        let headers = MHeaders::from(response.headers());

        let body_text = response.text().await.unwrap_or("".to_string());
        let body = match serde_json::from_str::<MValue>(&body_text) {
            Ok(body) => body,
            Err(_) => MValue::String(body_text),
        };

        ResponseDefinition {
            status,
            headers,
            body,
        }
    }

    pub fn compare(&self, other: &ResponseDefinition) -> TestResult {
        if !self.status.matches(&other.status) {
            return TestResult::fail(
                "Status does not match.",
                self.status.get_context(&other.status),
            );
        }

        if !self.headers.matches(&other.headers) {
            return TestResult::fail(
                "Headers do not match.",
                self.headers.get_context(&other.headers),
            );
        }

        if !self.body.matches(&other.body) {
            return TestResult::fail("Body does not match.", self.body.get_context(&other.body));
        }

        TestResult::Passed
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

        writeln!(f, "  Status: {}\n ", self.status)?;

        writeln!(f, "  Headers:\n{}", self.headers)?;

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
