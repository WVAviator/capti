use capti::errors::CaptiError;
use capti::loader::suite_loader::SuiteLoader;
use capti::reporter::results_reporter::ResultsReporter;
use capti::runner::runner::Runner;
use capti::Args;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), CaptiError> {
    let args = Args::parse();

    let path = args.path;
    let config = args.config;

    let loader = SuiteLoader::new(&path);
    loader.load_config(&config);

    let suites = loader.load_suites();
    let mut runner = Runner::new(suites);

    let results = runner.run().await;

    let reporter = ResultsReporter::new(results);
    reporter.print_results();
    reporter.print_summary();

    Ok(())
}
