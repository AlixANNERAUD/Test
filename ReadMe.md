<h1 align="center">Rust WASM application template</h1>

This is a template for a Rust WASM application. It uses `wasm-wasi32` as the target.

## Prerequisites

Before you begin, ensure you have met the following requirements:

* Latest version of Rust and Cargo.
* `wasm32-wasi` target:
```sh
rustup target add wasm32-wasi
```
* `wasi` extension for `cargo`:
```sh
cargo install cargo-wasi
```
* `wasmtime` runtime:
```sh
curl https://wasmtime.dev/install.sh -sSf | bash
```

## Usage

To build the application, run the following command followed by the desired build options:

```sh
cargo wasi ...
```
