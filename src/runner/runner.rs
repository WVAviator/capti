use std::path::PathBuf;

use colored::Colorize;
use walkdir::WalkDir;

use crate::{formatting::Heading, progress_println, suite::report::TestResultsReport, Suite};

use super::run_config::RunConfig;

pub struct Runner {
    suites: Vec<Suite>,
    config: Option<RunConfig>,
}

impl Runner {
    pub fn from_path(path: PathBuf, config_path: Option<PathBuf>) -> Self {
        let suites = WalkDir::new(&path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().extension().unwrap_or_default() == "yaml"
                    || e.path().extension().unwrap_or_default() == "yml"
            })
            .map(|e| e.path().to_path_buf())
            .filter_map(|path| {
                std::fs::read_to_string(path)
                    .map_err(|e| {
                        eprintln!("Failed to read suite: {}", e);
                        e
                    })
                    .ok()
            })
            .filter_map(|data| {
                serde_yaml::from_str::<Suite>(&data)
                    .map_err(|e| {
                        eprintln!("Failed to parse suite: {}", e);
                        e
                    })
                    .ok()
            })
            .collect::<Vec<Suite>>();

        progress_println!("Found {} test suites", suites.len());

        let config = match config_path {
            Some(path) => match std::fs::read_to_string(path) {
                Ok(config) => serde_yaml::from_str::<RunConfig>(&config).ok(),
                Err(e) => {
                    eprintln!("Failed to read config file: {}", e);
                    None
                }
            },
            None => WalkDir::new(&path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| match e.path().file_name() {
                    Some(name) => name == "capti-config.yaml" || name == "capti-config.yml",
                    None => false,
                })
                .map(|e| e.path().to_path_buf())
                .filter_map(|path| std::fs::read_to_string(path).ok())
                .find_map(|data| serde_yaml::from_str::<RunConfig>(&data).ok()),
        };

        progress_println!("Found config file: {}", config.is_some());

        Runner { suites, config }
    }

    pub async fn run(&mut self) {
        if let Some(config) = &self.config {
            if let Some(setup) = &config.setup {
                progress_println!("Running test setup scripts");
                setup.execute_before_all().await;
            }
        }

        let mut futures = Vec::new();
        for suite in self.suites.iter_mut() {
            let future = process(suite);
            futures.push(future);
        }

        let reports = futures::future::join_all(futures).await;

        if let Some(config) = &self.config {
            if let Some(setup) = &config.setup {
                setup.execute_after_all().await;
            }
        }

        for report in reports.iter() {
            progress_println!("{}", report);
        }

        let total_tests = reports
            .iter()
            .fold(0, |acc, report| acc + report.total_tests)
            .to_string();
        let total_passed = reports.iter().fold(0, |acc, report| acc + report.passed);
        let total_failed = reports.iter().fold(0, |acc, report| acc + report.failed);
        let total_errors = reports.iter().fold(0, |acc, report| acc + report.errors);

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
async fn process(suite: &mut Suite) -> TestResultsReport {
    let report = suite.run().await;
    report
}
