use serde::{Deserialize, Serialize};

use super::{setup::SuiteSetup, test::Test};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Suite {
    suite: String,
    description: Option<String>,
    setup: Option<SuiteSetup>,
    tests: Vec<Test>,
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::suite::test::RequestMethod;

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
