extern crate getopts;

use std::env;

#[test]
fn main() {
    let program = env::args().next().unwrap();

    getopts::Options::new().parse([program]).unwrap();
}
