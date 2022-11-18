const { run } = wasm_bindgen;
wasm_bindgen('./pkg/separate_memory_transfer_bg.wasm').then(run).catch(console.error);
