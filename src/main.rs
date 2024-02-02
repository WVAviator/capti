use clap::Parser;
use surf::Args;

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .expect("Unable to establish request client.");

    let response = client.get(&args.url).send().await.unwrap();

    if args.status {
        println!("Status: {}", response.status().as_u16());
    }

    let json = response.json::<serde_json::Value>().await.unwrap();

    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}
