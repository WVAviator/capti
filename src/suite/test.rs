use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Test {
    pub test: String,
    pub description: Option<String>,
    pub request: RequestDefinition,
    pub expect: ResponseDefinition,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestDefinition {
    pub method: RequestMethod,
    pub url: String,
    pub params: Option<HashMap<String, String>>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum RequestMethod {
    Get,
    Post,
    Patch,
    Put,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseDefinition {
    pub status: Option<u16>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<serde_json::Value>,
}
