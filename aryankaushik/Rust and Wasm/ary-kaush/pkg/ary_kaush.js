let imports = {};
let wasm;

/**
* @param {number} n1
* @param {number} n2
* @returns {number}
*/
module.exports.add2numbers = function(n1, n2) {
    const ret = wasm.add2numbers(n1, n2);
    return ret;
};

const path = require('path').join(__dirname, 'ary_kaush_bg.wasm');
const bytes = require('fs').readFileSync(path);

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
wasm = wasmInstance.exports;
module.exports.__wasm = wasm;

