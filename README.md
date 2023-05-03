<div align="center">

  <h1>Weedle 4 - Focal Coala</h1>

  <strong>A Web IDL parser</strong>

  <p>
    <a href="https://crates.io/crates/weedle4"><img src="https://img.shields.io/crates/v/weedle4.svg?style=flat-square" alt="Crates.io version" /></a>
    <a href="https://docs.rs/weedle4"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="Documentation" /></a>
    <a href="LICENSE"><img src="https://img.shields.io/crates/l/weedle4/2.0.0?style=flat-square" alt="MIT License" /></a>
  </p>

  <sub>
  Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a>.
  <br>
  Forked with [weedle2 patches](https://github.com/mozilla/uniffi-rs/tree/dc524271245c5f1e6b6113e6c201acb5e21da111/weedle2) and automated webref test to make sure that it can always read all latest web(idl) files.
  </sub>
</div>

## About

Parses valid WebIDL definitions & produces a data structure starting from
[`Definitions`](https://docs.rs/weedle4/latest/weedle/type.Definitions.html).

## Usage

### `Cargo.toml`

```toml
[dependencies]
weedle4 = "0.4.0"
```

### `src/main.rs`

```rust
fn main() {
    let parsed = weedle::parse("
        interface Window {
            readonly attribute Storage sessionStorage;
        };
    ").unwrap();

    println!("{:?}", parsed);
}
```
