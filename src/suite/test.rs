use std::{
    collections::HashMap,
    fmt::{self, Debug},
};

use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use crate::{
    client::client::get_client,
    matcher::{match_result::MatchResult, status_matcher::StatusMatcher, MatchCmp},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Test {
    pub test: String,
    pub description: Option<String>,
    pub request: RequestDefinition,
    pub expect: ResponseDefinition,
}

impl Test {
    pub async fn execute(&self) -> TestResult {
        let client = get_client();

        let request = self.request.build_client_request(&client);
        let response = match request.send().await {
            Ok(response) => response,
            Err(e) => return TestResult::Error(e.to_string()),
        };

        let response = ResponseDefinition::from_response(response).await;

        self.expect.compare(&response)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestResult {
    Passed,
    Failed(FailureReport),
    Error(String),
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestDefinition {
    pub method: RequestMethod,
    pub url: String,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<serde_json::Value>,
}

impl RequestDefinition {
    pub fn build_client_request(&self, client: &reqwest::Client) -> RequestBuilder {
        let mut request_builder = match self.method {
            RequestMethod::Get => client.get(&self.url),
        };

        if let Some(headers) = &self.headers {
            for (key, value) in headers {
                request_builder = request_builder.header(key, value);
            }
        }

        if let Some(body) = &self.body {
            let body_json = serde_json::to_string(&body).unwrap();
            request_builder = request_builder.body(body_json);
        }

        return request_builder;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum RequestMethod {
    Get,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseDefinition {
    pub status: Option<StatusMatcher>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<serde_json::Value>,
}

impl ResponseDefinition {
    pub async fn from_response(response: reqwest::Response) -> Self {
        let status = Some(StatusMatcher::Exact(response.status().as_u16()));
        let header_map = response.headers().clone();
        let headers = header_map
            .into_iter()
            .filter_map(|(header, value)| match header {
                Some(header) => Some((header, value)),
                None => None,
            })
            .map(|(header, value)| {
                (
                    header.as_str().to_string(),
                    value.to_str().unwrap().to_string(),
                )
            })
            .collect::<HashMap<String, String>>();

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
