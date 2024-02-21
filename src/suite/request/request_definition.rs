use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use crate::{
    errors::CaptiError,
    m_value::m_value::MValue,
    variables::{variable_map::VariableMap, SuiteVariables},
};

use super::{request_headers::RequestHeaders, request_method::RequestMethod};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestDefinition {
    pub method: RequestMethod,
    pub url: String,
    pub headers: Option<RequestHeaders>,
    pub body: Option<MValue>,
}

impl RequestDefinition {
    pub fn build_client_request(
        &self,
        client: &reqwest::Client,
    ) -> Result<RequestBuilder, CaptiError> {
        let mut request_builder = match self.method {
            RequestMethod::Get => client.get(&self.url),
            RequestMethod::Post => client.post(&self.url),
            RequestMethod::Patch => client.patch(&self.url),
            RequestMethod::Put => client.put(&self.url),
            RequestMethod::Delete => client.delete(&self.url),
        };

        if let Some(headers) = &self.headers {
            request_builder = request_builder.headers(headers.clone().into());
        }

        if let Some(body) = &self.body {
            let body_json = serde_json::to_string(&body)?;
            request_builder = request_builder.body(body_json);
        }

        return Ok(request_builder);
    }
}

impl SuiteVariables for RequestDefinition {
    fn populate_variables(&mut self, variables: &mut VariableMap) -> Result<(), CaptiError> {
        self.url = variables.replace_variables(&self.url)?.into();
        self.headers.populate_variables(variables)?;
        self.body.populate_variables(variables)?;

        Ok(())
    }
}
