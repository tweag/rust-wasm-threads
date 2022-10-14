use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn run() {
    spawn(work).expect("failed to start worker");
}

pub fn spawn(ptr: fn()) -> Result<web_sys::Worker, JsValue> {
    let w = web_sys::Worker::new("./worker.js")?;
    w.post_message(&JsValue::from(ptr as u32))?;
    Ok(w)
}

fn work() {
    log("Working!");
}

#[wasm_bindgen]
pub fn child_entry_point(addr: u32) {
    let f: fn() = unsafe { std::mem::transmute(addr as *const ()) };
    f();
}
