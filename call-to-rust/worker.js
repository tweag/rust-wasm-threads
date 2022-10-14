importScripts('./pkg/call_to_rust.js')

const { child_entry_point } = wasm_bindgen;

self.onmessage = async event => {
  await wasm_bindgen('./pkg/call_to_rust_bg.wasm');
  console.log(child_entry_point);
  console.log(event.data);
  child_entry_point(Number(event.data));
};
