use clap::Parser;
use surf::{Args, ResponseOutput};

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let output = ResponseOutput::new(args)
        .await
        .expect("Failed to fetch response.");

    print!("{}", output);
}
