/*importScripts("./../../pkg-webgl2/core.js")

self.onmessage = async (event) => {
    // event.data[0] should be the Memory object, and event.data[1] is the value to pass into child_entry_point
    const { child_entry_point } = await wasm_bindgen(
        "./../../pkg-webgl2/core_bg.wasm", event.data[0]
    );

    child_entry_point(Number(event.data[1]))
}*/
function instantiateWorker() {
    self.onmessage = async (event) => {
        // give the memory to the webassembly instance
        WebAssembly.instantiateStreaming(fetch('./../../pkg-webgl2/core_bg.wasm'), { env: event.data[0] } )
            .then(results => {
                console.log(results)
            });
    }
}
