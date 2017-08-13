Rust Daemon Template
====================

This is a template for writing a daemon using Rust. The template includes

* Command-line argument parsing
* Config loading from hard-coded location using yaml
* A `run()` function called by `main()` to support `?` operator. Errors returned
  from `run()` are simply printed to stderr using the `Display` impl, and the
  process exits with `1`.
* Logging to stderr using the log crate and controlled by the command line -v
  flags.

To get started, clone this project and `s/rust_daemon_template/foo/`.

The template can run out of the box using

```
cargo run -- --config ./config.yml.example debug
```

## FAQ

* _Where is the daemonization logic?_ There isn't any; your system's process
  manager should be able to handle this for you.
