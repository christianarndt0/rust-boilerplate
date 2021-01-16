# rust-boilerplate

## Cargo does (almost) everything

### build, install and run

#### --release

By default, cargo runs everything in debug mode.

Scopes preceded with `#[cfg(debug_assertions)` will only be used when **not** using this flag.

Scopes preceded with `#[cfg(not(debug_assertions))]` will only be used when *using* this flag.

```console
cargo build --release
```

#### --example

By default, cargo builds and runs `./src/main.rs` but you can use different implementations by passing an executable from `./examples/`.

```console
cargo build --example example1
```

#### or both

```console
cargo build --example example1 --release
```

### test

`#[cfg(test)]`: The following scope will only be built for tests

`#[ŧest]`: Define a test function, e.g.

```rust
#[test]
fn do_something() {
    assert_eq!(4, 4);
}
```

Run all tests with:

```console
cargo test
```

### doc

```console
cargo doc
```

## mdBook

TODO: Building user guides with mdBook...
