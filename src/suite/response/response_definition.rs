use serde::{Serialize, Deserialize};

use crate::{
    client::client::get_client,
    errors::config_error::ConfigurationError,
    matcher::{match_result::MatchResult, status_matcher::StatusMatcher, MatchCmp},
    crate::suite::response::ResponseHeaders
};

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

// impl MatchCmp for ResponseDefinition {
//   fn match_cmp(&self, other: &Self) -> MatchResult {
//         match self.status.match_cmp(&other.status) {
//             MatchResult::Matches => {}
//             other => return other.with_context(format!("at compare ( {:#?}: {:#?} )", &self, &other)),
//         }

//         match self.headers.match_cmp(&other.headers) {
//             MatchResult::Matches => {}
//             other => return other.with_context(format!("at compare ( {:#?}: {:#?} )", &self, &other)),
//         }

//         match self.body.match_cmp(&other.body) {
//             MatchResult::Matches => {}
//             other => return other.with_context(format!("at compare ( {:#?}: {:#?} )", &self, &other)),
//         }

//         return MatchResult::Matches;
//   }
// }
