# {{project-name}}

Rust template that implements a simple logging framework and loading configs from file to build on top of.

## Creating a new project from the template

This uses [cargo-generate](https://github.com/cargo-generate/cargo-generate) to automatically replace crate placeholders with your project's info.
After installing, use

```console
cargo generate --git https://github.com/christianarndt0/rust-boilerplate
```

to create a new project from this template.

## Examples

Load the appropriate default config from `./config/`, create a terminal and file logger, and print the loaded config.

```console
cargo run --example log_config [--release] 
```
