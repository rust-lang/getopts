use getopts::Options;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optmulti("I", "include", "add a directory to the search path", "DIR");
    opts.optflagmulti("v", "verbose", "increase output verbosity");

    let matches = opts.parse(&args[1..]).unwrap_or_else(|error| {
        eprintln!("{error}");
        process::exit(2);
    });

    println!("Verbosity: {}", matches.opt_count("verbose"));
    for directory in matches.opt_strs("include") {
        println!("Include: {directory}");
    }
}
