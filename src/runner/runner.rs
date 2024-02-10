use walkdir::WalkDir;

use crate::{suite::report::TestResultsReport, Suite};

use super::run_config::RunConfig;

pub struct Runner {
    suites: Vec<Suite>,
    total_tests: usize,
    config: Option<RunConfig>,
}

impl Runner {
    pub fn from_path(path: &String) -> Self {
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

        let config = WalkDir::new(&path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().to_str() == Some("config.yaml"))
            .map(|e| e.path().to_path_buf())
            .filter_map(|path| std::fs::read_to_string(path).ok())
            .find_map(|data| serde_yaml::from_str::<RunConfig>(&data).ok());

        let total_tests = suites.iter().map(|suite| suite.get_test_count()).sum();

        Runner {
            suites,
            total_tests,
            config,
        }
    }

    pub async fn run(&mut self) {
        let mut progress_bars = self
            .suites
            .iter()
            .map(|suite| {
                let pb = indicatif::ProgressBar::new(suite.get_test_count() as u64);
                let prefix = format!("Running {}", &suite.suite);
                pb.set_prefix(prefix);
                pb
            })
            .collect::<Vec<indicatif::ProgressBar>>();

        let primary_pb = indicatif::ProgressBar::new(self.total_tests as u64);

        primary_pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .expect("Error setting progress bar style."),
        );

        let mut futures = Vec::new();
        for (suite, pb) in self.suites.iter_mut().zip(progress_bars.iter_mut()) {
            let future = process(suite, pb, &primary_pb);
            futures.push(future);
        }

        let reports = futures::future::join_all(futures).await;

        primary_pb.finish_with_message("All suites finished.");

        for report in reports {
            println!("{}", report);
        }
    }
}
async fn process(
    suite: &mut Suite,
    pb: &indicatif::ProgressBar,
    primary_pb: &indicatif::ProgressBar,
) -> TestResultsReport {
    let report = suite
        .run(|_result| {
            pb.inc(1);
            primary_pb.inc(1);
        })
        .await;
    pb.finish_with_message("Done.");
    report
}
