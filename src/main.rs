use clap_v3::{App, Arg};

fn main() {
    let matches = App::new("My RPN program")
        .arg(
            Arg::with_name("formula_file")
                .value_name("FILE")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::with_name("verbose")
                .short('v')
                .long("verbose")
                .required(false)
        ).get_matches();

    match matches.value_of("formula_file") {
        Some(file) => println!("file name is {}", file),
        None => println!("no file")
    };

    let verbose = matches.is_present("verbose");
    println!("verbose is {}", verbose);
}
