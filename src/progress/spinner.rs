use std::time::Duration;

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    progress_println,
    suite::{report::ReportedResult, test::TestResult},
};

use super::multiprogress::multiprogress;

pub struct Spinner {
    spinner: ProgressBar,
    text: String,
}

impl Spinner {
    pub async fn start(text: impl Into<String>) -> Self {
        let text = text.into();

        let load_template = format!("{{spinner}} {}...", text);

        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template(load_template.as_str())
                .expect("Invalid template for spinner."),
        );
        spinner.enable_steady_tick(Duration::from_millis(100));

        let multiprogress = multiprogress()
            .lock()
            .expect("Failed to lock multiprogress.");
        let spinner = multiprogress.add(spinner);

        Spinner { spinner, text }
    }

    pub fn finish_test(self, reported_result: &ReportedResult) {
        let finish_template = match &reported_result.result {
            Ok(TestResult::Passed) => {
                format!("{} {}... {}", "✓".green(), self.text, "[OK]".green())
            }
            Ok(TestResult::Failed(_)) => {
                format!("{} {}... {}", "✗".red(), self.text, "[FAILED]".red())
            }
            Err(_) => format!("{} {}... {}", "⚠".yellow(), self.text, "[ERROR]".yellow()),
        };

        self.spinner.set_style(
            ProgressStyle::default_spinner()
                .template(finish_template.as_str())
                .expect("Invalid template for spinner."),
        );

        self.spinner.finish();

        {
            let multiprogress = multiprogress()
                .lock()
                .expect("Failed to lock multiprogress.");
            multiprogress.remove(&self.spinner);
            multiprogress
                .println(format!("{}", finish_template))
                .expect("Unable to print line after multiprogress.");
        }

        match &reported_result.result {
            Ok(TestResult::Passed) => {}
            Ok(TestResult::Failed(failure_report)) => {
                progress_println!("{}", failure_report);
            }
            Err(e) => {
                progress_println!("{} {}", "→".yellow(), e);
            }
        }
    }

    pub fn finish(self, message: impl Into<String>) {
        let finish_template = format!("✔︎ {}... {}", self.text, message.into());
        self.spinner.set_style(
            ProgressStyle::default_spinner()
                .template(finish_template.as_str())
                .expect("Invalid template for spinner."),
        );

        self.spinner.finish();

        let multiprogress = multiprogress()
            .lock()
            .expect("Failed to lock multiprogress.");
        multiprogress.remove(&self.spinner);
        multiprogress
            .println(format!("{}", finish_template))
            .expect("Unable to print line after multiprogress.");
    }
}
