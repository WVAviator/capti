use crate::{progress_println, suite::report::TestResultsReport, Suite};

use super::run_config::RunConfig;

pub struct Runner {
    suites: Vec<Suite>,
}

impl Runner {
    pub fn new(suites: Vec<Suite>) -> Self {
        Runner { suites }
    }

    pub async fn run(&mut self) -> Vec<TestResultsReport> {
        if let Some(setup) = &RunConfig::global().setup {
            progress_println!("Running test setup scripts");
            setup.execute_before_all().await;
        }

        let mut futures = Vec::new();
        for suite in self.suites.iter_mut() {
            let future = suite.run();
            futures.push(future);
        }

        let results = futures::future::join_all(futures).await;

        if let Some(setup) = &RunConfig::global().setup {
            setup.execute_after_all().await;
        }

        results
    }
}
