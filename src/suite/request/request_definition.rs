use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use super::{request_headers::RequestHeaders, request_method::RequestMethod};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestDefinition {
    pub method: RequestMethod,
    pub url: String,
    pub headers: Option<RequestHeaders>,
    pub body: Option<serde_json::Value>,
}

impl RequestDefinition {
    pub fn build_client_request(&self, client: &reqwest::Client) -> RequestBuilder {
        let mut request_builder = match self.method {
            RequestMethod::Get => client.get(&self.url),
        };

        if let Some(headers) = &self.headers {
            request_builder = request_builder.headers(headers.clone().into());
        }

        if let Some(body) = &self.body {
            let body_json = serde_json::to_string(&body).unwrap(); // TODO: Propogate error
            request_builder = request_builder.body(body_json);
        }

        return request_builder;
    }
}
