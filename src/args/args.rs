use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, 
        long, 
        value_hint = clap::ValueHint::DirPath, 
        default_value = ".", 
        help = "Path to your tests directory.", 
        long_help = "The path argument should point to the directory where your tests are located. If no path is provided, the current working directory will be used.")]
    pub path: PathBuf,
}
