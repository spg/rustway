use gameoflife::{run, Config};
use std::process;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the file to open
    #[arg(short, long)]
    path: Option<String>,

    /// Whether to generate a random input
    #[arg(short, long, default_value_t = false)]
    random: bool,
}

fn main() {
    let args = Args::parse();

    if args.path == None && args.random == false {
        panic!("Must set either random or path")
    }

    let config: Config = Config {
        file_path: args.path,
        random: args.random,
    };

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
