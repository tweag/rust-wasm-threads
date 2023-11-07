# separate-memory-transfer

An example in which we spawn a worker and transfer a buffer back and forth between the main thread
and the worker.

To run this example, `cd` into this directory and then:
```sh
cargo install --version 0.2.81 wasm-bindgen-cli --force # if you aren't getting it from the nix flake
cargo build --release
wasm-bindgen ../target/wasm32-unknown-unknown/release/separate_memory_transfer.wasm --target=no-modules --out-dir=pkg
python ../server.py
```
and then open your browser and go to `http://localhost:8000`.

Note that the version of the `wasm-bindgen` binary must exactly match the version of the `wasm-bindgen` crate
that `cargo` pulls in. If you're using nix, the dev shell in `flake.nix` should take care of that for you.
