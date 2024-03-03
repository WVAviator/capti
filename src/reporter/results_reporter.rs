use colored::Colorize;

use crate::{formatting::Heading, progress_println, suite::report::TestResultsReport};

pub struct ResultsReporter {
    results: Vec<TestResultsReport>,
}

impl ResultsReporter {
    pub fn new(results: Vec<TestResultsReport>) -> Self {
        ResultsReporter { results }
    }

    pub fn print_results(&self) {
        for report in self.results.iter() {
            progress_println!("{}", report);
        }
    }

    pub fn print_summary(&self) {
        let total_tests = self
            .results
            .iter()
            .fold(0, |acc, report| acc + report.total_tests)
            .to_string();
        let total_passed = self
            .results
            .iter()
            .fold(0, |acc, report| acc + report.passed);
        let total_failed = self
            .results
            .iter()
            .fold(0, |acc, report| acc + report.failed);
        let total_errors = self
            .results
            .iter()
            .fold(0, |acc, report| acc + report.errors);

        let total_passed = match total_passed {
            0 => "0".normal(),
            _ => total_passed.to_string().green(),
        };

        let total_failed = match total_failed {
            0 => "0".normal(),
            _ => total_failed.to_string().red(),
        };

        let total_errors = match total_errors {
            0 => "0".normal(),
            _ => total_errors.to_string().yellow(),
        };

        let heading = "Results Summary".header();

        progress_println!(
            " \n{}\n \nTotal Tests: {}\n \nTotal Passed: {}\nTotal Failed: {}\nTotal Errors: {}\n ",
            heading,
            total_tests,
            total_passed,
            total_failed,
            total_errors,
        );
    }
}
