use capti::errors::config_error::ConfigurationError;
use capti::runner::runner::Runner;
use capti::Args;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), ConfigurationError> {
    let args = Args::parse();
    let path = args.path;
    let config = args.config;

    let mut runner = Runner::from_path(path, config);

    runner.run().await;

    Ok(())
}
