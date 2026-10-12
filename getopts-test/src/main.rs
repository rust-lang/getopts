use getopts::{Matches, Options};
use std::{env, process};

const BRIEF: &str = "Usage: getopts-test [options] INPUT...";

fn options() -> Options {
    let mut options = Options::new();
    options.optflagmulti("v", "verbose", "increase verbosity");
    options.optopt("o", "output", "write output to FILE", "FILE");
    options.optmulti("I", "include", "add DIR to the include path", "DIR");
    options.optflagopt("", "color", "control color output", "WHEN");
    options.optflag("h", "help", "print this help menu");
    options
}

fn fail(options: &Options, message: impl std::fmt::Display) -> ! {
    eprintln!("error: {message}");
    eprintln!();
    eprint!("{}", options.usage(BRIEF));
    process::exit(2);
}

fn print_matches(matches: &Matches) {
    let output = matches
        .opt_str("output")
        .unwrap_or_else(|| "<stdout>".into());
    let color = matches
        .opt_default("color", "auto")
        .unwrap_or_else(|| "never".into());

    println!("verbosity: {}", matches.opt_count("verbose"));
    println!("output: {output}");
    println!("color: {color}");

    for include in matches.opt_strs("include") {
        println!("include: {include}");
    }
    for input in &matches.free {
        println!("input: {input}");
    }
}

fn main() {
    let options = options();
    let matches = options
        .parse(env::args().skip(1))
        .unwrap_or_else(|error| fail(&options, error));

    if matches.opt_present("help") {
        print!("{}", options.usage(BRIEF));
        return;
    }
    if matches.free.is_empty() {
        fail(&options, "at least one INPUT is required");
    }

    print_matches(&matches);
}
