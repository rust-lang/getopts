use getopts::Options;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    let mut opts = Options::new();
    opts.reqopt("c", "config", "path to the configuration file", "FILE");

    let matches = opts.parse(&args[1..]).unwrap_or_else(|error| {
        eprintln!("{error}");
        eprintln!("{}", opts.short_usage(program));
        process::exit(2);
    });

    let config = matches
        .opt_str("config")
        .expect("required options are present after parsing");
    println!("Using configuration from {config}");
}
