use capti::errors::config_error::ConfigurationError;
use capti::runner::runner::Runner;
use capti::Args;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), ConfigurationError> {
    let args = Args::parse();

    let mut runner = Runner::from_path(&args.path);

    runner.run().await;

    Ok(())
}
