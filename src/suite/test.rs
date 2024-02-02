use std::collections::HashMap;

use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use crate::{client::client::get_client, matcher::MatchCmp};

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

        if self.expect.compare(&response) {
            TestResult::Passed
        } else {
            TestResult::Failed
        }
    }
}

pub enum TestResult {
    Passed,
    Failed,
    Error(String),
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
    pub status: Option<u16>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<serde_json::Value>,
}

impl ResponseDefinition {
    pub async fn from_response(response: reqwest::Response) -> Self {
        let status = Some(response.status().as_u16());
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

    pub fn compare(&self, other: &ResponseDefinition) -> bool {
        if !self.status.match_cmp(&other.status) {
            return false;
        }

        if !self.headers.match_cmp(&other.headers) {
            return false;
        }

        if !self.body.match_cmp(&other.body) {
            return false;
        }

        return true;
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
            status: Some(200),
        };

        assert!(matcher.compare(&response));
    }

    #[test]
    fn test_compare_status_matches() {
        let matcher = ResponseDefinition {
            headers: None,
            body: None,
            status: Some(200),
        };
        let response = ResponseDefinition {
            headers: None,
            body: None,
            status: Some(200),
        };
        assert!(matcher.compare(&response));
    }
}
