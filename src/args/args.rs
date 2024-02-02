use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub url: String,

    #[arg(short, long, help = "Prints the HTTP response status code.")]
    pub status: bool,
}
