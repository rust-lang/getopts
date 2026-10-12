use assert_cmd::Command;
use predicates::prelude::*;

const BRIEF: &str = "Usage: getopts-test [options] INPUT...";

fn command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_getopts-test"))
}

#[test]
fn parses_options_and_positional_arguments() {
    command()
        .args([
            "-vv",
            "--output",
            "report.txt",
            "-I",
            "src",
            "--include",
            "tests",
            "--color=always",
            "input.txt",
            "extra.txt",
        ])
        .assert()
        .success()
        .stdout(
            "verbosity: 2\n\
             output: report.txt\n\
             color: always\n\
             include: src\n\
             include: tests\n\
             input: input.txt\n\
             input: extra.txt\n",
        )
        .stderr("");
}

#[test]
fn uses_defaults_and_honors_the_option_terminator() {
    command()
        .args(["--color", "--", "-input.txt"])
        .assert()
        .success()
        .stdout(
            "verbosity: 0\n\
             output: <stdout>\n\
             color: auto\n\
             input: -input.txt\n",
        )
        .stderr("");
}

#[test]
fn prints_help() {
    command()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(BRIEF))
        .stdout(predicate::str::contains("--color [WHEN]"))
        .stderr("");
}

#[test]
fn reports_parse_and_validation_errors() {
    command()
        .arg("--unknown")
        .assert()
        .code(2)
        .stderr(predicate::str::contains("Unrecognized option"));

    command()
        .assert()
        .code(2)
        .stderr(predicate::str::contains("at least one INPUT is required"));
}
