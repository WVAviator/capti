use clap::Parser;
use surf::errors::config_error::ConfigurationError;
use surf::Args;
use surf::Suite;

#[tokio::main]
async fn main() -> Result<(), ConfigurationError> {
    let args = Args::parse();

    let mut suite = Suite::from_file(&args.path)?;

    let report = suite.run().await;

    print!("{}", report);

    Ok(())
}
