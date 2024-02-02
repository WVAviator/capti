use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    url: String,
}

fn main() {
    let args = Args::parse();

    println!("url: {}", args.url);
}
