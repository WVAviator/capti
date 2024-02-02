use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Suite {
    suite: String,
    description: Option<String>,
    setup: Option<SuiteSetup>,
    tests: Vec<Test>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SuiteSetup {
    before_all: Option<Vec<SetupInstruction>>,
    before_each: Option<Vec<SetupInstruction>>,
    after_all: Option<Vec<SetupInstruction>>,
    after_each: Option<Vec<SetupInstruction>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetupInstruction {
    description: String,
    script: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Test {
    test: String,
    description: Option<String>,
    request: RequestDefinition,
    expect: ResponseDefinition,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestDefinition {
    method: RequestMethod,
    url: String,
    params: Option<HashMap<String, String>>,
    headers: Option<HashMap<String, String>>,
    body: Option<serde_json::Value>,
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
    status: Option<u16>,
    headers: Option<HashMap<String, String>>,
    body: Option<serde_json::Value>,
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn deserializes_simple_get_example() {
        let example_suite = fs::read_to_string("examples/simple_get.yaml").unwrap();
        let suite = serde_yaml::from_str::<Suite>(&example_suite).unwrap();
        assert_eq!(suite.suite, String::from("Simple Get Request Tests"));
        assert_eq!(suite.tests[0].request.method, RequestMethod::Get);
        assert_eq!(suite.tests[0].expect.body.as_ref().unwrap()["userId"], 1);
    }
}
