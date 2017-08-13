rust_daemon_template
====================

This is a template for writing a daemon using Rust. Provided is

* Command-line argument parsing
* Config loading from hard-coded location using yaml
* A `run()` function called by `main()` to support `?` operator. Errors returned
  from `run()` are simply printed to stderr using the `Display` impl, and the
  process exits with `1`.

To get started, clone this project and `s/rust_daemon_template/foo/`.

The template can run out of the box using

```
cargo run -- --config ./config.yml.example debug
```
