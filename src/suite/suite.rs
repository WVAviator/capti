use serde::Deserialize;

use crate::{
    client::Client,
    errors::config_error::ConfigurationError,
    suite::{
        report::{ReportedResult, TestResultsReport},
        setup::SuiteSetup,
    },
    variables::{variable_map::VariableMap, SuiteVariables},
};

use super::test::Test;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Suite {
    suite: String,
    description: Option<String>,
    #[serde(default)]
    parallel: bool,
    setup: Option<SuiteSetup>,
    tests: Vec<Test>,
    #[serde(default)]
    variables: VariableMap,
    #[serde(skip)]
    client: Client,
}

impl Suite {
    pub fn from_file(path: &str) -> Result<Self, ConfigurationError> {
        let suite = std::fs::read_to_string(path)?;
        let suite = serde_yaml::from_str::<Suite>(&suite)?;
        return Ok(suite);
    }

    pub async fn run(&mut self) -> TestResultsReport {
        let mut results = vec![];

        println!("Running {} tests...", self.tests.len());

        if let Some(setup) = &self.setup {
            setup.execute_before_all().await;
        }

        for test in self.tests.iter_mut() {
            test.populate_variables(&mut self.variables)
                .expect("Error populating variables for test.");
        }

        for test in &self.tests {
            let test_execution = async {
                if let Some(setup) = &self.setup {
                    setup.execute_before_each().await;
                }

                let result = test.execute(&self.client).await;
                let reported_result = ReportedResult::new(test, result);

                if let Some(setup) = &self.setup {
                    setup.execute_after_each().await;
                }

                return reported_result;
            };
            results.push(test_execution);
        }

        let results = match &self.parallel {
            true => futures::future::join_all(results).await,
            false => {
                let mut executed_results = vec![];
                for result in results {
                    executed_results.push(result.await);
                }
                executed_results
            }
        };

        let report = TestResultsReport::new(results);

        if let Some(setup) = &self.setup {
            setup.execute_after_all().await;
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
        let mut suite = serde_yaml::from_str::<Suite>(&example_suite).unwrap();
        let results_report = suite.run().await;

        assert_eq!(results_report.passed, 6);
        assert_eq!(results_report.failed, 0);
    }
}
