use capti::errors::CaptiError;
use capti::runner::runner::Runner;
use capti::Args;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), CaptiError> {
    let args = Args::parse();
    let path = args.path;
    let config = args.config;

    let mut runner = Runner::from_path(path, config);

    runner.run().await;

    Ok(())
}
