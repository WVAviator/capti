use std::fmt;

use colored::Colorize;

use crate::errors::config_error::ConfigurationError;

use super::test::{Test, TestResult};

pub struct TestResultsReport {
    pub total_tests: usize,
    pub passed: usize,
    pub failed: usize,
    pub errors: usize,
    pub results: Vec<ReportedResult>,
}

pub struct ReportedResult {
    pub test: Test,
    pub result: Result<TestResult, ConfigurationError>,
}

impl ReportedResult {
    pub fn new(test: &Test, result: Result<TestResult, ConfigurationError>) -> Self {
        ReportedResult {
            test: test.clone(),
            result,
        }
    }
}

impl fmt::Display for ReportedResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.result {
            Ok(TestResult::Passed) => write!(f, "{}... {}", self.test.test, "[OK]".green()),
            Ok(TestResult::Failed(_)) => write!(f, "{}... {}", self.test.test, "[FAILED]".red()),
            Err(e) => {
                write!(f, "{}... {}\n{:#?}", self.test.test, "[ERROR]".red(), e)
            }
        }
    }
}

impl TestResultsReport {
    pub fn new(tests: Vec<ReportedResult>) -> Self {
        let total_tests = tests.len();

        let (passed, failed, errors) =
            tests
                .iter()
                .fold((0, 0, 0), |(passed, failed, errors), r| match r.result {
                    Ok(TestResult::Passed) => (passed + 1, failed, errors),
                    Ok(TestResult::Failed(_)) => (passed, failed + 1, errors),
                    Err(_) => (passed, failed, errors + 1),
                });

        TestResultsReport {
            total_tests,
            passed,
            failed,
            errors,
            results: tests,
        }
    }
}

impl fmt::Display for TestResultsReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for result in &self.results {
            if let Ok(TestResult::Failed(failure)) = &result.result {
                writeln!(f, "{}", failure)?;
            }
        }

        writeln!(f, "")?;

        for result in &self.results {
            writeln!(f, "{}", result)?;
        }

        writeln!(f, "")?;

        write!(
            f,
            "Summary:\nPassed: {} | Failed: {} | Errors: {} | Total: {}",
            self.passed, self.failed, self.errors, self.total_tests
        )?;
        Ok(())
    }
}
