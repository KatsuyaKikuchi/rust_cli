use clap::{App, Arg};
use clap::Clap;

#[derive(Clap, Debug)]
#[clap(
name = "My RPN program",
version = "1.0.0",
author = "Katsuya Kikuchi",
about = "Supe awesome sample RPN calculator"
)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();
    match opts.formula_file {
        Some(file) => println!("file is {}", file),
        None => println!("file not found"),
    };

    let verbose = opts.verbose;
    println!("verbose is {}", verbose);
}
