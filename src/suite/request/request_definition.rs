use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use crate::{
    errors::CaptiError,
    m_value::m_value::MValue,
    variables::{variable_map::VariableMap, SuiteVariables},
};

use super::{
    query_params::QueryParams, request_headers::RequestHeaders, request_method::RequestMethod,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestDefinition {
    pub method: RequestMethod,
    pub url: String,
    #[serde(default)]
    pub params: QueryParams,
    pub headers: Option<RequestHeaders>,
    pub body: Option<MValue>,
}

impl RequestDefinition {
    pub fn build_client_request(
        &self,
        client: &reqwest::Client,
    ) -> Result<RequestBuilder, CaptiError> {
        let url = format!("{}{}", &self.url, &self.params.as_query_string());

        let mut request_builder = match self.method {
            RequestMethod::Get => client.get(url),
            RequestMethod::Post => client.post(url),
            RequestMethod::Patch => client.patch(url),
            RequestMethod::Put => client.put(url),
            RequestMethod::Delete => client.delete(url),
        };

        if let Some(headers) = &self.headers {
            request_builder = request_builder.headers(headers.clone().into());
        }

        if let Some(body) = &self.body {
            let body_json = serde_json::to_string(&body)?;
            request_builder = request_builder.body(body_json);
        }

        Ok(request_builder)
    }
}

impl SuiteVariables for RequestDefinition {
    fn populate_variables(&mut self, variables: &mut VariableMap) -> Result<(), CaptiError> {
        self.url = variables.replace_variables(&self.url)?.into();
        self.params.populate_variables(variables)?;
        self.headers.populate_variables(variables)?;
        self.body.populate_variables(variables)?;

        Ok(())
    }
}
