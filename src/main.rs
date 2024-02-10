use clap::Parser;
use surf::errors::config_error::ConfigurationError;
use surf::runner::runner::Runner;
use surf::Args;

#[tokio::main]
async fn main() -> Result<(), ConfigurationError> {
    let args = Args::parse();

    let mut runner = Runner::from_path(&args.path);

    runner.run().await;

    Ok(())
}
