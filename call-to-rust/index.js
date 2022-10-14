const { run } = wasm_bindgen;
wasm_bindgen('./pkg/call_to_rust_bg.wasm').then(run).catch(console.error);
