use std::fmt::Debug;

use serde::Deserialize;

use crate::{
    client::Client,
    errors::CaptiError,
    formatting::{indent::Indent, Heading},
    m_value::match_context::MatchContext,
    progress::Spinner,
    progress_println,
    variables::{variable_map::VariableMap, SuiteVariables},
};

use super::{
    extract::ResponseExtractor, failure_report::FailureReport, report::ReportedResult,
    request::RequestDefinition, response::ResponseDefinition, test_result::TestResult,
};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct TestDefinition {
    pub test: String,
    pub description: Option<String>,
    #[serde(default)]
    pub should_fail: bool,
    pub request: RequestDefinition,
    pub expect: ResponseDefinition,
    pub extract: Option<ResponseExtractor>,
    #[serde(default)]
    print_response: bool,
    #[serde(default)]
    define: VariableMap,
}

impl TestDefinition {
    pub async fn execute(
        &self,
        client: &Client,
        suite: &str,
        variables: Option<&mut VariableMap>,
    ) -> ReportedResult {
        let spinner = Spinner::start(format!("[{}] {}", &suite, &self.test)).await;

        let test_result = self.process(client, variables).await;
        let reported_result = ReportedResult::new(self, test_result);

        spinner.finish_test(&reported_result);

        return reported_result;
    }

    async fn process(
        &self,
        client: &Client,
        variables: Option<&mut VariableMap>,
    ) -> Result<TestResult, CaptiError> {
        let request = self.request.build_client_request(&client)?;
        let response = request.send().await?;

        let response = ResponseDefinition::from_response(response).await;

        if self.print_response {
            let title = format!("Response: ({})", &self.test);
            let heading = &title.header();
            let footer = &title.footer();

            progress_println!(
                "\n{}\n{}\n{}\n ",
                &heading,
                response.to_string().indent(),
                &footer
            );
        }

        let test_result = self.expect.compare(&response)?;

        let test_result = match (test_result, self.should_fail) {
            (TestResult::Passed, true) => TestResult::Failed(FailureReport::new(
                "Expected failure, but test passed.",
                MatchContext::new(),
            )),
            (TestResult::Failed(_), true) => TestResult::Passed,
            (result, _) => result,
        };

        // Skip extraction if the test failed
        if let TestResult::Failed(_) = test_result {
            return Ok(test_result);
        }

        if let Some(extractor) = &self.extract {
            if let Some(variables) = variables {
                extractor.extract(&response, variables).await?;
            } else {
                return Err(CaptiError::parallel_error("Cannot extract variables from tests running in parallel. Try setting the suite to 'parallel: false'"));
            }
        }

        Ok(test_result)
    }
}

impl SuiteVariables for TestDefinition {
    fn populate_variables(&mut self, variables: &mut VariableMap) -> Result<(), CaptiError> {
        self.request
            .populate_variables(&mut self.define)
            .unwrap_or(());
        self.expect
            .populate_variables(&mut self.define)
            .unwrap_or(());

        self.request.populate_variables(variables)?;
        self.expect.populate_variables(variables)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use crate::{
        m_value::m_value::MValue,
        suite::{headers::MHeaders, response::status::Status},
    };

    use super::*;

    #[test]
    fn test_compare_optional() {
        let matcher = ResponseDefinition {
            headers: MHeaders::default(),
            body: MValue::default(),
            status: Status::none(),
        };
        let response = ResponseDefinition {
            headers: MHeaders::default(),
            body: serde_json::from_str::<MValue>(r#"{"test": "test"}"#).unwrap(),
            status: Status::from(200),
        };

        assert_eq!(matcher.compare(&response).unwrap(), TestResult::Passed);
    }

    #[test]
    fn test_compare_status_matches() {
        let matcher = ResponseDefinition {
            headers: MHeaders::default(),
            body: MValue::Null,
            status: Status::from("2xx"),
        };
        let response = ResponseDefinition {
            headers: MHeaders::default(),
            body: MValue::Null,
            status: Status::from(200),
        };

        assert_eq!(matcher.compare(&response).unwrap(), TestResult::Passed);
    }
}
