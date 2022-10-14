importScripts('./pkg/shared_memory.js')

const { child_entry_point } = wasm_bindgen;

self.onmessage = async event => {
  // We expect a message with three elements [module, memory, ptr], where:
  //  - module is a WebAssembly.Module
  //  - memory is the WebAssembly.Memory object that the main thread is using
  //    (and we want to use it too).
  //  - ptr is the pointer (within `memory`) of the function that we want to execute.
  //
  // The auto-generated `wasm_bindgen` function takes two *optional* arguments.
  // The first is the module (you can pass in a url, like we did in "index.js",
  // or a module object); the second is the memory block to use, and if you
  // don't provide one (like we didn't in "index.js") then a new one will be
  // allocated for you.
  let init = await wasm_bindgen('./pkg/shared_memory_bg.wasm', event.data[1]).catch(err => {
    // Propagate to main `onerror`:
    setTimeout(() => {
      throw err;
    });
    // Rethrow to keep promise rejected and prevent execution of further commands:
    throw err;
  });

  child_entry_point(event.data[2]);
  
  // Clean up thread resources. Depending on what you're doing with the thread, this might
  // not be what you want. (For example, if the thread spawned some javascript tasks
  // and exited, this is going to cancel those tasks.) But if you're using threads in the
  // usual native way (where you spin one up to do some work until it finisheds) then
  // you'll want to clean up the thread's resources.

  // Free memory (stack, thread-locals) held (in the wasm linear memory) by the thread.
  init.__wbindgen_thread_destroy();
  // Tell the browser to stop the thread.
  close();
};
