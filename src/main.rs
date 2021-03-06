use clap::Clap;
use std::fs::File;
use std::io::{BufReader, BufRead, stdin};
use anyhow::{Result};
use std::path::PathBuf;

mod rpn_calculator;

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
    formula_file: Option<PathBuf>,
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose);
    } else {
        // ファイル指定なし
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose);
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> {
    let calc = rpn_calculator::RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line.unwrap();

        match calc.eval(&line) {
            Ok(ans) => println!("{}", ans),
            Err(e) => eprintln!("{:#?}", e),
        }
    }

    Ok(())
}
