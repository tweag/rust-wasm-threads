# call-to-rust

A simple example in which we spawn a worker and tell it to call a certain rust function.

To run this example, `cd` into this directory and then:
```rust
> cargo build --release
> wasm-bindgen ../target/wasm32-unknown-unknown/release/call_to_rust.wasm --target=no-modules --out-dir=pkg
> python ../server.py
```
and then open your browser and go to `http://localhost:8000`.

Note that the version of the `wasm-bindgen` binary must exactly match the version of the `wasm-bindgen` crate
that `cargo` pulls in. If you're using nix, the dev shell in `flake.nix` should take care of that for you.
