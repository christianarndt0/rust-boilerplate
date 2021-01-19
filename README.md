# rust-boilerplate

## Cargo does (almost) everything

Cargo makes building, testing, documenting, running and installing rust applications very easy and standardized.

### Building, Running and Installing

Compile a local package and all of its dependencies with `cargo build`.

```console
cargo build [--release]
```

#### --release

By default, cargo runs everything in debug mode.

Scopes preceded with `#[cfg(debug_assertions)` will only be used when **not** using this flag.

Scopes preceded with `#[cfg(not(debug_assertions))]` will only be used when *using* this flag.

#### --example

By default, cargo builds and runs `./src/lib.rs` and/or `./src/main.rs` but you can use different implementations by passing an executable from `./examples/`.

```console
cargo build --example EXAMPLE.rs [--release]
```

will build `./src/lib.rs` and/or `./src/main.rs`.

#### or both

```console
cargo build --example example1 --release
```

***

Replace `build` with `run` to also launch the executable.

Replace `build` with `install` to install the binary system-wide. On Linux, the default location is `$HOME/.cargo/bin/`.

### Testing and Documenting

Execute all unit and integration tests and build examples of a local package

```console
cargo test
```

Cargo uses rustdoc (TODO insert link) to build documentation for your crate. Create documentation with

```console
cargo doc
```

## User Guides with mdBook

TODO: Building user guides with mdBook...
