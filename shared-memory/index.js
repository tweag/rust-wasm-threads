const { run } = wasm_bindgen;
wasm_bindgen('./pkg/shared_memory_bg.wasm').then(run).catch(console.error);
