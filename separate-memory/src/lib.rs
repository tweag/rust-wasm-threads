use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const SIZE: u32 = 16;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_name = "performance")]
    pub static PERFORMANCE: web_sys::Performance;
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn run() {
    let w = spawn(&10000.into()).expect("failed to start worker");
    let w_clone = w.clone();
    let on_msg: Closure<dyn FnMut(web_sys::MessageEvent)> =
        Closure::new(move |msg: web_sys::MessageEvent| {
            let buf = msg.data().dyn_into::<js_sys::ArrayBuffer>().unwrap();
            w_clone.post_message(&buf).unwrap();
        });
    w.set_onmessage(Some(on_msg.as_ref().unchecked_ref()));
    std::mem::forget(on_msg);
}

pub fn spawn(arg: &JsValue) -> Result<web_sys::Worker, JsValue> {
    let w = web_sys::Worker::new("./worker.js")?;

    // See `worker.js` for the format of this message.
    let msg: js_sys::Array = [&wasm_bindgen::module(), arg].into_iter().collect();
    w.post_message(&msg)?;
    Ok(w)
}

#[wasm_bindgen]
pub fn child_entry_point(count: i32) {
    let vec = js_sys::Uint8Array::new_with_length(SIZE);
    vec.fill(48, 0, SIZE);

    log(&format!("got {}", count));
    let global = js_sys::global()
        .dyn_into::<web_sys::DedicatedWorkerGlobalScope>()
        .unwrap();

    let global_clone = global.clone();
    let mut cur_count = count;
    let mut start = PERFORMANCE.now();
    let on_msg: Closure<dyn FnMut(web_sys::MessageEvent)> =
        Closure::new(move |msg: web_sys::MessageEvent| {
            let buf = msg.data().dyn_into::<js_sys::ArrayBuffer>().unwrap();
            if cur_count == count {
                start = PERFORMANCE.now();
            }
            cur_count -= 1;
            if cur_count >= 0 {
                global_clone.post_message(&buf).unwrap();
            } else {
                let end = PERFORMANCE.now();
                log(&format!(
                    "buffer ping-pong took {} ms per iteration",
                    (end - start) / count as f64
                ));
            }
        });

    global.set_onmessage(Some(on_msg.as_ref().unchecked_ref()));
    global.post_message(&vec.buffer()).unwrap();
    std::mem::forget(on_msg);
}
