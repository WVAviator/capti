use serde::Deserialize;

use crate::{
    errors::config_error::ConfigurationError,
    suite::{
        report::{ReportedResult, TestResultsReport},
        setup::SuiteSetup,
    },
};

use super::test::Test;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Suite {
    suite: String,
    description: Option<String>,
    setup: Option<SuiteSetup>,
    tests: Vec<Test>,
}

impl Suite {
    pub fn from_file(path: &str) -> Result<Self, ConfigurationError> {
        let suite = std::fs::read_to_string(path)?;
        let suite = serde_yaml::from_str::<Suite>(&suite)?;
        return Ok(suite);
    }

    pub async fn run(&self) -> TestResultsReport {
        let mut results = vec![];

        println!("Running {} tests...", self.tests.len());

        if let Some(setup) = &self.setup {
            setup.execute_before_all();
        }

        for test in &self.tests {
            let test_execution = async move {
                if let Some(setup) = &self.setup {
                    setup.execute_before_each();
                }

                let result = test.execute().await;
                let reported_result = ReportedResult::new(test, result);

                if let Some(setup) = &self.setup {
                    setup.execute_after_each();
                }

                return reported_result;
            };
            results.push(test_execution);
        }

        let results = futures::future::join_all(results).await;
        let report = TestResultsReport::new(results);

        if let Some(setup) = &self.setup {
            setup.execute_after_all();
        }

        println!("{}", report);
        return report;
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::suite::request::request_method::RequestMethod;

    use super::*;

    #[test]
    fn deserializes_simple_get_example() {
        let example_suite = fs::read_to_string("examples/simple_get.yaml").unwrap();
        let suite = serde_yaml::from_str::<Suite>(&example_suite).unwrap();

        assert_eq!(suite.suite, String::from("Simple Get Request Tests"));
        assert_eq!(suite.tests[0].request.method, RequestMethod::Get);
        assert_eq!(suite.tests[0].expect.body.as_ref().unwrap()["id"], 1);
    }

    #[tokio::test]
    async fn executes_simple_get_example() {
        let example_suite = fs::read_to_string("examples/simple_get.yaml").unwrap();
        let suite = serde_yaml::from_str::<Suite>(&example_suite).unwrap();
        let results_report = suite.run().await;

        assert_eq!(results_report.passed, 5);
        assert_eq!(results_report.failed, 0);
    }
}
