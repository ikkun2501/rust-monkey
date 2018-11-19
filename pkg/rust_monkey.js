/* tslint:disable */
import * as wasm from './rust_monkey_bg';

let cachedTextEncoder = new TextEncoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function passStringToWasm(arg) {

    const buf = cachedTextEncoder.encode(arg);
    const ptr = wasm.__wbindgen_malloc(buf.length);
    getUint8Memory().set(buf, ptr);
    return [ptr, buf.length];
}

let cachedTextDecoder = new TextDecoder('utf-8');

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

let cachedGlobalArgumentPtr = null;
function globalArgumentPtr() {
    if (cachedGlobalArgumentPtr === null) {
        cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
    }
    return cachedGlobalArgumentPtr;
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

function freeRepl(ptr) {

    wasm.__wbg_repl_free(ptr);
}
/**
*/
export class Repl {

    static __wrap(ptr) {
        const obj = Object.create(Repl.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeRepl(ptr);
    }

    /**
    * @returns {Repl}
    */
    static new() {
        return Repl.__wrap(wasm.repl_new());
    }
    /**
    * @param {string} arg0
    * @returns {string}
    */
    input(arg0) {
        const [ptr0, len0] = passStringToWasm(arg0);
        const retptr = globalArgumentPtr();
        try {
            wasm.repl_input(retptr, this.ptr, ptr0, len0);
            const mem = getUint32Memory();
            const rustptr = mem[retptr / 4];
            const rustlen = mem[retptr / 4 + 1];

            const realRet = getStringFromWasm(rustptr, rustlen).slice();
            wasm.__wbindgen_free(rustptr, rustlen * 1);
            return realRet;


        } finally {
            wasm.__wbindgen_free(ptr0, len0 * 1);

        }

    }
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

