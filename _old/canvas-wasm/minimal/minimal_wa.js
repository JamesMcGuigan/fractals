// DOCS: https://developer.mozilla.org/en-US/docs/WebAssembly/Loading_and_running
// DOCS: https://compile.fi/canvas-filled-three-ways-js-webassembly-and-webgl/

// let importObject = {};
// fetch('minimal_wa.c.wasm').then(response =>
//     response.arrayBuffer()
// ).then(bytes => {
//     let module   = new WebAssembly.Module(bytes);
//     let instance = new WebAssembly.Instance(module, importObject);
//     render(instance);
//     // instance.exports.exported_func();
// })

const importObject = {
    imports: {
        render: function(...args) {
            console.log(...args);
        }
    }
};
WebAssembly.instantiateStreaming(fetch('minimal_wa.c.wasm'), importObject)
    .then(wasm => {
        // TODO: pass in memory address?
        // TODO: render full screen, based on canvas size
        // TODO: make red
        render(wasm.instance);
        // obj.instance.exports.render()
    });


// const base64data = 'AGFzbQEAAAABBQFgAAF/AhIBA2VudgZtZW1vcnkCAYACgAIDAgEABwsBB19yZW5kZXIAAApJAUcBA38DQCAAQaAGbCECQQAhAQNAIAEgAmpBAnRBgAhqQf+BgHg2AgAgAUEBaiIBQaAGRw0ACyAAQQFqIgBBkANHDQALQYAICw==';
// const decode = (b64) => {
//     const str = window.atob(b64);
//     const array = new Uint8Array(str.length);
//     for (let i = 0; i < str.length; i += 1) {
//         array[i] = str.charCodeAt(i);
//     }
//     return array.buffer;
// };

function render(instance) {
    // Get 2d drawing context
    const canvas = document.getElementById('canvas-wasm-minimal');
    const ctx = canvas.getContext('2d');
    window.resizeCanvas(canvas);
    debugger;

    const width   = 400; // canvas.width;
    const height  = 400; // canvas.height;
    const memSize = width * height * 4;
    assert ( memSize < 65536 );
    const memory  = new WebAssembly.Memory({ initial: memSize, maximum: memSize });

    // const pointer = instance.exports._render();
    const pointer = instance.exports.render();
    // const data = new Uint8ClampedArray(memory.buffer, pointer, width * height * 4);
    const data = new Uint8ClampedArray(memory.buffer, pointer, memSize);
    const img = new ImageData(data, width, height);
    ctx.putImageData(img, 0, 0);
}