/* tslint:disable */
import * as wasm from './wasm_example_bg';

const TextDecoder = typeof self === 'object' && self.TextDecoder
    ? self.TextDecoder
    : require('util').TextDecoder;

let cachedDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null ||
        cachegetUint8Memory.buffer !== wasm.memory.buffer)
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

export function __wbg_f_alert_alert_n(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    alert(varg0);
}

let slab = [];

let slab_next = 0;

function addHeapObject(obj) {
    if (slab_next === slab.length)
        slab.push(slab.length + 1);
    const idx = slab_next;
    const next = slab[idx];

    slab_next = next;

    slab[idx] = { obj, cnt: 1 };
    return idx << 1;
}

export function __wbg_static_accessor_document_document() {
    return addHeapObject(document);
}

const __wbg_f_createElement_createElement_HTMLDocument_target = HTMLDocument.prototype.createElement;

let stack = [];

function getObject(idx) {
    if ((idx & 1) === 1) {
        return stack[idx >> 1];
    } else {
        const val = slab[idx >> 1];

    return val.obj;

    }
}

export function __wbg_f_createElement_createElement_HTMLDocument(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    return addHeapObject(__wbg_f_createElement_createElement_HTMLDocument_target.call(getObject(arg0), varg1));
}

function GetOwnOrInheritedPropertyDescriptor(obj, id) {
  while (obj) {
    let desc = Object.getOwnPropertyDescriptor(obj, id);
    if (desc) return desc;
    obj = Object.getPrototypeOf(obj);
  }
  throw "descriptor not found";
}

const __wbg_f_body_body_HTMLDocument_target = GetOwnOrInheritedPropertyDescriptor(HTMLDocument.prototype, 'body').get;;

export function __wbg_f_body_body_HTMLDocument(arg0) {
    return addHeapObject(__wbg_f_body_body_HTMLDocument_target.call(getObject(arg0)));
}

const __wbg_f_appendChild_append_Element_target = Element.prototype.appendChild;

function dropRef(idx) {

    let obj = slab[idx >> 1];

    obj.cnt -= 1;
    if (obj.cnt > 0)
        return;

    // If we hit 0 then free up our space in the slab
    slab[idx >> 1] = slab_next;
    slab_next = idx >> 1;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropRef(idx);
    return ret;
}

export function __wbg_f_appendChild_append_Element(arg0, arg1) {
    __wbg_f_appendChild_append_Element_target.call(getObject(arg0), takeObject(arg1));
}

const __wbg_f_set_inner_set_inner_Element_target = GetOwnOrInheritedPropertyDescriptor(Element.prototype, 'innerHTML').set;;

export function __wbg_f_set_inner_set_inner_Element(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    __wbg_f_set_inner_set_inner_Element_target.call(getObject(arg0), varg1);
}

const TextEncoder = typeof self === 'object' && self.TextEncoder
    ? self.TextEncoder
    : require('util').TextEncoder;

let cachedEncoder = new TextEncoder('utf-8');

function passStringToWasm(arg) {

    const buf = cachedEncoder.encode(arg);
    const ptr = wasm.__wbindgen_malloc(buf.length);
    getUint8Memory().set(buf, ptr);
    return [ptr, buf.length];
}

export function run_alert(arg0) {
    const [ptr0, len0] = passStringToWasm(arg0);
    try {
        return wasm.run_alert(ptr0, len0);
    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);
    }
}

export function create_stuff() {
    return wasm.create_stuff();
}

export function __wbindgen_object_clone_ref(idx) {
    // If this object is on the stack promote it to the heap.
    if ((idx & 1) === 1)
        return addHeapObject(getObject(idx));

    // Otherwise if the object is on the heap just bump the
    // refcount and move on
    const val = slab[idx >> 1];
    val.cnt += 1;
    return idx;
}

export function __wbindgen_object_drop_ref(i) { dropRef(i); }

export function __wbindgen_string_new(p, l) {
    return addHeapObject(getStringFromWasm(p, l));
}

export function __wbindgen_number_new(i) { return addHeapObject(i); }

export function __wbindgen_number_get(n, invalid) {
    let obj = getObject(n);
    if (typeof(obj) === 'number')
        return obj;
    getUint8Memory()[invalid] = 1;
    return 0;
}

export function __wbindgen_undefined_new() { return addHeapObject(undefined); }

export function __wbindgen_null_new() {
    return addHeapObject(null);
}

export function __wbindgen_is_null(idx) {
    return getObject(idx) === null ? 1 : 0;
}

export function __wbindgen_is_undefined(idx) {
    return getObject(idx) === undefined ? 1 : 0;
}

export function __wbindgen_boolean_new(v) {
    return addHeapObject(v === 1);
}

export function __wbindgen_boolean_get(i) {
    let v = getObject(i);
    if (typeof(v) === 'boolean') {
        return v ? 1 : 0;
    } else {
        return 2;
    }
}

export function __wbindgen_symbol_new(ptr, len) {
    let a;
    if (ptr === 0) {
        a = Symbol();
    } else {
        a = Symbol(getStringFromWasm(ptr, len));
    }
    return addHeapObject(a);
}

export function __wbindgen_is_symbol(i) {
    return typeof(getObject(i)) === 'symbol' ? 1 : 0;
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null ||
        cachegetUint32Memory.buffer !== wasm.memory.buffer)
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    return cachegetUint32Memory;
}

export function __wbindgen_string_get(i, len_ptr) {
    let obj = getObject(i);
    if (typeof(obj) !== 'string')
        return 0;
    const [ptr, len] = passStringToWasm(obj);
    getUint32Memory()[len_ptr / 4] = len;
    return ptr;
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

