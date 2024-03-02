use std::fmt;

use colored::Colorize;

use crate::{errors::CaptiError, formatting::Heading};

use super::{test::TestDefinition, test_result::TestResult};

pub struct TestResultsReport {
    pub suite: String,
    pub total_tests: usize,
    pub passed: usize,
    pub failed: usize,
    pub errors: usize,
    pub results: Vec<ReportedResult>,
}

pub struct ReportedResult {
    pub test: TestDefinition,
    pub result: Result<TestResult, CaptiError>,
}

impl ReportedResult {
    pub fn new(test: &TestDefinition, result: Result<TestResult, CaptiError>) -> Self {
        ReportedResult {
            test: test.clone(),
            result,
        }
    }
}

impl fmt::Display for ReportedResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.result {
            Ok(TestResult::Passed) => {
                write!(f, "{} {}", "✓".green(), self.test.test,)
            }
            Ok(TestResult::Failed(_)) => {
                write!(f, "{} {}", "✗".red(), self.test.test,)
            }
            Err(_) => {
                write!(f, "{} {}", "⚠".yellow(), self.test.test,)
            }
        }
    }
}

impl TestResultsReport {
    pub fn new(suite: impl Into<String>, tests: Vec<ReportedResult>) -> Self {
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
            suite: suite.into(),
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
        writeln!(f, " ")?;
        writeln!(f, "{}", self.suite.header())?;
        writeln!(f, " ")?;

        for result in &self.results {
            writeln!(f, "{}", result)?;
        }

        writeln!(f, " ")?;

        let passed = {
            match self.passed {
                0 => String::from("0").normal(),
                _ => self.passed.to_string().green(),
            }
        };

        let failed = {
            match self.failed {
                0 => String::from("0").normal(),
                _ => self.failed.to_string().red(),
            }
        };

        let errors = {
            match self.errors {
                0 => String::from("0").normal(),
                _ => self.errors.to_string().yellow(),
            }
        };

        write!(
            f,
            "Passed: {} | Failed: {} | Errors: {} ▐  Total: {}",
            passed, failed, errors, self.total_tests
        )?;
        Ok(())
    }
}
