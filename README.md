getopts
===

A Rust library for option parsing for CLI utilities.

[Documentation](https://docs.rs/getopts)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
getopts = "0.2"
```

## Examples

- [`required_option.rs`](examples/required_option.rs) shows how to define and
  read an option that must be provided.
- [`repeated_options.rs`](examples/repeated_options.rs) shows how to collect
  repeated option values and count repeated flags.

Run them from this repository with:

```console
cargo run --example required_option -- --config config.toml
cargo run --example repeated_options -- -vv -I src --include vendor
```

## Contributing

The `getopts` library is used by `rustc`, so we have to be careful about not changing its behavior.
