use std::path::PathBuf;

use walkdir::WalkDir;

use crate::{progress_println, suite::report::TestResultsReport, Suite};

use super::run_config::RunConfig;

pub struct Runner {
    suites: Vec<Suite>,
    config: Option<RunConfig>,
}

impl Runner {
    pub fn from_path(path: PathBuf) -> Self {
        let suites = WalkDir::new(&path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().extension().unwrap_or_default() == "yaml"
                    || e.path().extension().unwrap_or_default() == "yml"
            })
            .map(|e| e.path().to_path_buf())
            .filter_map(|path| std::fs::read_to_string(path).ok())
            .filter_map(|data| serde_yaml::from_str::<Suite>(&data).ok())
            .collect::<Vec<Suite>>();

        progress_println!("Found {} test suites", suites.len());

        let config = WalkDir::new(&path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| match e.path().file_name() {
                Some(name) => name == "capti-config.yaml" || name == "capti-config.yml",
                None => false,
            })
            .map(|e| e.path().to_path_buf())
            .filter_map(|path| std::fs::read_to_string(path).ok())
            .find_map(|data| serde_yaml::from_str::<RunConfig>(&data).ok());

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

        for report in reports {
            progress_println!("{}", report);
        }
    }
}
async fn process(suite: &mut Suite) -> TestResultsReport {
    let report = suite.run().await;
    report
}
