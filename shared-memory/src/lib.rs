use std::sync::mpsc;
use wasm_bindgen::prelude::*;

const SIZE: u32 = 1024 * 1024 * 20;

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
    let (main_to_worker_tx, main_to_worker_rx) = mpsc::sync_channel(1);
    let (worker_to_main_tx, worker_to_main_rx) = mpsc::sync_channel(1);

    spawn(move || {
        let mut vec = std::iter::repeat('g' as u8)
            .take(SIZE as usize)
            .collect::<Vec<_>>();
        let start = PERFORMANCE.now();
        let iters = 10000;
        for _ in 0..iters {
            main_to_worker_tx.send(vec).unwrap();
            vec = worker_to_main_rx.recv().unwrap();
        }
        let end = PERFORMANCE.now();
        log(&format!(
            "buffer ping-pong took {} ms per iteration",
            (end - start) / iters as f64
        ));
    })
    .expect("failed to start worker 1");

    spawn(move || {
        while let Ok(vec) = main_to_worker_rx.recv() {
            worker_to_main_tx.send(vec).unwrap();
        }
    })
    .expect("failed to start worker 2");
}

pub fn spawn(f: impl FnOnce() + Send + 'static) -> Result<web_sys::Worker, JsValue> {
    let w = web_sys::Worker::new("./worker.js")?;
    // Double-boxing because `dyn FnOnce` is unsized and so `Box<dyn FnOnce()>` has
    // an undefined layout (although I think in practice its a pointer and a length?).
    let ptr = Box::into_raw(Box::new(Box::new(f) as Box<dyn FnOnce()>));

    // See `worker.js` for the format of this message.
    let msg: js_sys::Array = [
        &wasm_bindgen::module(),
        &wasm_bindgen::memory(),
        &JsValue::from(ptr as u32),
    ]
    .into_iter()
    .collect();
    if let Err(e) = w.post_message(&msg) {
        // We expect the worker to deallocate the box, but if there was an error then
        // we'll do it ourselves.
        let _ = unsafe { Box::from_raw(ptr) };
        Err(e)
    } else {
        Ok(w)
    }
}

#[wasm_bindgen]
pub fn child_entry_point(ptr: u32) {
    let work = unsafe { Box::from_raw(ptr as *mut Box<dyn FnOnce()>) };
    (*work)();
}
