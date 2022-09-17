import * as wasm from "./life_rust_bg.wasm";

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) {
  return heap[idx];
}

let heap_next = heap.length;

function dropObject(idx) {
  if (idx < 36) return;
  heap[idx] = heap_next;
  heap_next = idx;
}

function takeObject(idx) {
  const ret = getObject(idx);
  dropObject(idx);
  return ret;
}

const lTextDecoder =
  typeof TextDecoder === "undefined"
    ? (0, module.require)("util").TextDecoder
    : TextDecoder;

let cachedTextDecoder = new lTextDecoder("utf-8", {
  ignoreBOM: true,
  fatal: true,
});

cachedTextDecoder.decode();

let cachedUint8Memory0 = new Uint8Array();

function getUint8Memory0() {
  if (cachedUint8Memory0.byteLength === 0) {
    cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
  }
  return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
  return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
  if (heap_next === heap.length) heap.push(heap.length + 1);
  const idx = heap_next;
  heap_next = heap[idx];

  heap[idx] = obj;
  return idx;
}

let WASM_VECTOR_LEN = 0;

const lTextEncoder =
  typeof TextEncoder === "undefined"
    ? (0, module.require)("util").TextEncoder
    : TextEncoder;

let cachedTextEncoder = new lTextEncoder("utf-8");

const encodeString =
  typeof cachedTextEncoder.encodeInto === "function"
    ? function (arg, view) {
        return cachedTextEncoder.encodeInto(arg, view);
      }
    : function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
          read: arg.length,
          written: buf.length,
        };
      };

function passStringToWasm0(arg, malloc, realloc) {
  if (realloc === undefined) {
    const buf = cachedTextEncoder.encode(arg);
    const ptr = malloc(buf.length);
    getUint8Memory0()
      .subarray(ptr, ptr + buf.length)
      .set(buf);
    WASM_VECTOR_LEN = buf.length;
    return ptr;
  }

  let len = arg.length;
  let ptr = malloc(len);

  const mem = getUint8Memory0();

  let offset = 0;

  for (; offset < len; offset++) {
    const code = arg.charCodeAt(offset);
    if (code > 0x7f) break;
    mem[ptr + offset] = code;
  }

  if (offset !== len) {
    if (offset !== 0) {
      arg = arg.slice(offset);
    }
    ptr = realloc(ptr, len, (len = offset + arg.length * 3));
    const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
    const ret = encodeString(arg, view);

    offset += ret.written;
  }

  WASM_VECTOR_LEN = offset;
  return ptr;
}

let cachedInt32Memory0 = new Int32Array();

function getInt32Memory0() {
  if (cachedInt32Memory0.byteLength === 0) {
    cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
  }
  return cachedInt32Memory0;
}

function debugString(val) {
  // primitive types
  const type = typeof val;
  if (type == "number" || type == "boolean" || val == null) {
    return `${val}`;
  }
  if (type == "string") {
    return `"${val}"`;
  }
  if (type == "symbol") {
    const description = val.description;
    if (description == null) {
      return "Symbol";
    } else {
      return `Symbol(${description})`;
    }
  }
  if (type == "function") {
    const name = val.name;
    if (typeof name == "string" && name.length > 0) {
      return `Function(${name})`;
    } else {
      return "Function";
    }
  }
  // objects
  if (Array.isArray(val)) {
    const length = val.length;
    let debug = "[";
    if (length > 0) {
      debug += debugString(val[0]);
    }
    for (let i = 1; i < length; i++) {
      debug += ", " + debugString(val[i]);
    }
    debug += "]";
    return debug;
  }
  // Test for built-in
  const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
  let className;
  if (builtInMatches.length > 1) {
    className = builtInMatches[1];
  } else {
    // Failed to match the standard '[object ClassName]'
    return toString.call(val);
  }
  if (className == "Object") {
    // we're a user defined class or Object
    // JSON.stringify avoids problems with cycles, and is generally much
    // easier than looping through ownProperties of `val`.
    try {
      return "Object(" + JSON.stringify(val) + ")";
    } catch (_) {
      return "Object";
    }
  }
  // errors
  if (val instanceof Error) {
    return `${val.name}: ${val.message}\n${val.stack}`;
  }
  // TODO we could test for more things here, like `Set`s and `Map`s.
  return className;
}

let stack_pointer = 32;

function addBorrowedObject(obj) {
  if (stack_pointer == 1) throw new Error("out of js stack");
  heap[--stack_pointer] = obj;
  return stack_pointer;
}
/**
 * @param {string} canvas_id
 * @param {any} colors
 * @param {any} rules
 */
export function initialize(canvas_id, colors, rules) {
  try {
    const ptr0 = passStringToWasm0(
      canvas_id,
      wasm.__wbindgen_malloc,
      wasm.__wbindgen_realloc
    );
    const len0 = WASM_VECTOR_LEN;
    wasm.initialize(
      ptr0,
      len0,
      addBorrowedObject(colors),
      addBorrowedObject(rules)
    );
  } finally {
    heap[stack_pointer++] = undefined;
    heap[stack_pointer++] = undefined;
  }
}

/**
 */
export function start_render() {
  wasm.start_render();
}

/**
 */
export function stop_render() {
  wasm.stop_render();
}

/**
 */
export function next_frame() {
  wasm.next_frame();
}

/**
 * @param {number} speed
 */
export function set_speed(speed) {
  wasm.set_speed(speed);
}

function isLikeNone(x) {
  return x === undefined || x === null;
}

function handleError(f, args) {
  try {
    return f.apply(this, args);
  } catch (e) {
    wasm.__wbindgen_exn_store(addHeapObject(e));
  }
}

function getArrayU8FromWasm0(ptr, len) {
  return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}

export function __wbindgen_object_drop_ref(arg0) {
  takeObject(arg0);
}

export function __wbindgen_string_new(arg0, arg1) {
  const ret = getStringFromWasm0(arg0, arg1);
  return addHeapObject(ret);
}

export function __wbindgen_json_serialize(arg0, arg1) {
  const obj = getObject(arg1);
  const ret = JSON.stringify(obj === undefined ? null : obj);
  const ptr0 = passStringToWasm0(
    ret,
    wasm.__wbindgen_malloc,
    wasm.__wbindgen_realloc
  );
  const len0 = WASM_VECTOR_LEN;
  getInt32Memory0()[arg0 / 4 + 1] = len0;
  getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}

export function __wbg_instanceof_Window_42f092928baaee84(arg0) {
  const ret = getObject(arg0) instanceof Window;
  return ret;
}

export function __wbg_document_15b2e504fb1556d6(arg0) {
  const ret = getObject(arg0).document;
  return isLikeNone(ret) ? 0 : addHeapObject(ret);
}

export function __wbg_getElementById_927eae2597d26692(arg0, arg1, arg2) {
  const ret = getObject(arg0).getElementById(getStringFromWasm0(arg1, arg2));
  return isLikeNone(ret) ? 0 : addHeapObject(ret);
}

export function __wbg_instanceof_HtmlCanvasElement_9f56aef8c479066b(arg0) {
  const ret = getObject(arg0) instanceof HTMLCanvasElement;
  return ret;
}

export function __wbg_width_54a66e74169bb513(arg0) {
  const ret = getObject(arg0).width;
  return ret;
}

export function __wbg_height_d4607377aede83c6(arg0) {
  const ret = getObject(arg0).height;
  return ret;
}

export function __wbg_getContext_efe7e95b72348104() {
  return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
  }, arguments);
}

export function __wbg_instanceof_CanvasRenderingContext2d_10bb8c4425aab773(
  arg0
) {
  const ret = getObject(arg0) instanceof CanvasRenderingContext2D;
  return ret;
}

export function __wbg_canvas_a29baa374ab1714f(arg0) {
  const ret = getObject(arg0).canvas;
  return isLikeNone(ret) ? 0 : addHeapObject(ret);
}

export function __wbg_setstrokeStyle_6031fd3bde25a376(arg0, arg1) {
  getObject(arg0).strokeStyle = getObject(arg1);
}

export function __wbg_setfillStyle_73949a5c3b61798a(arg0, arg1) {
  getObject(arg0).fillStyle = getObject(arg1);
}

export function __wbg_setshadowOffsetX_45fcefcc79178fd0(arg0, arg1) {
  getObject(arg0).shadowOffsetX = arg1;
}

export function __wbg_setshadowOffsetY_f842eb21b56e97e4(arg0, arg1) {
  getObject(arg0).shadowOffsetY = arg1;
}

export function __wbg_setshadowBlur_2594433fb6f6d4d5(arg0, arg1) {
  getObject(arg0).shadowBlur = arg1;
}

export function __wbg_setshadowColor_2df8d06dcaf6c80b(arg0, arg1, arg2) {
  getObject(arg0).shadowColor = getStringFromWasm0(arg1, arg2);
}

export function __wbg_beginPath_4a4302577da62125(arg0) {
  getObject(arg0).beginPath();
}

export function __wbg_fill_e5193c88834dc182(arg0) {
  getObject(arg0).fill();
}

export function __wbg_stroke_fe693002a3fc8e6a(arg0) {
  getObject(arg0).stroke();
}

export function __wbg_arc_b54bf161e510b1a8() {
  return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).arc(arg1, arg2, arg3, arg4, arg5);
  }, arguments);
}

export function __wbg_closePath_82947eae4ee7ec93(arg0) {
  getObject(arg0).closePath();
}

export function __wbg_clearRect_53506e6d8f66e793(arg0, arg1, arg2, arg3, arg4) {
  getObject(arg0).clearRect(arg1, arg2, arg3, arg4);
}

export function __wbg_fillRect_3b87fb719605af54(arg0, arg1, arg2, arg3, arg4) {
  getObject(arg0).fillRect(arg1, arg2, arg3, arg4);
}

export function __wbg_newnoargs_971e9a5abe185139(arg0, arg1) {
  const ret = new Function(getStringFromWasm0(arg0, arg1));
  return addHeapObject(ret);
}

export function __wbg_call_33d7bcddbbfa394a() {
  return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
  }, arguments);
}

export function __wbg_self_fd00a1ef86d1b2ed() {
  return handleError(function () {
    const ret = self.self;
    return addHeapObject(ret);
  }, arguments);
}

export function __wbg_window_6f6e346d8bbd61d7() {
  return handleError(function () {
    const ret = window.window;
    return addHeapObject(ret);
  }, arguments);
}

export function __wbg_globalThis_3348936ac49df00a() {
  return handleError(function () {
    const ret = globalThis.globalThis;
    return addHeapObject(ret);
  }, arguments);
}

export function __wbg_global_67175caf56f55ca9() {
  return handleError(function () {
    const ret = global.global;
    return addHeapObject(ret);
  }, arguments);
}

export function __wbindgen_is_undefined(arg0) {
  const ret = getObject(arg0) === undefined;
  return ret;
}

export function __wbindgen_object_clone_ref(arg0) {
  const ret = getObject(arg0);
  return addHeapObject(ret);
}

export function __wbg_new_3a746f2619705add(arg0, arg1) {
  const ret = new Function(getStringFromWasm0(arg0, arg1));
  return addHeapObject(ret);
}

export function __wbg_call_f54d3a6dadb199ca(arg0, arg1) {
  const ret = getObject(arg0).call(getObject(arg1));
  return addHeapObject(ret);
}

export function __wbindgen_jsval_eq(arg0, arg1) {
  const ret = getObject(arg0) === getObject(arg1);
  return ret;
}

export function __wbg_self_ac379e780a0d8b94(arg0) {
  const ret = getObject(arg0).self;
  return addHeapObject(ret);
}

export function __wbg_crypto_1e4302b85d4f64a2(arg0) {
  const ret = getObject(arg0).crypto;
  return addHeapObject(ret);
}

export function __wbg_getRandomValues_1b4ba144162a5c9e(arg0) {
  const ret = getObject(arg0).getRandomValues;
  return addHeapObject(ret);
}

export function __wbg_require_6461b1e9a0d7c34a(arg0, arg1) {
  const ret = require(getStringFromWasm0(arg0, arg1));
  return addHeapObject(ret);
}

export function __wbg_randomFillSync_1b52c8482374c55b(arg0, arg1, arg2) {
  getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));
}

export function __wbg_getRandomValues_1ef11e888e5228e9(arg0, arg1, arg2) {
  getObject(arg0).getRandomValues(getArrayU8FromWasm0(arg1, arg2));
}

export function __wbindgen_debug_string(arg0, arg1) {
  const ret = debugString(getObject(arg1));
  const ptr0 = passStringToWasm0(
    ret,
    wasm.__wbindgen_malloc,
    wasm.__wbindgen_realloc
  );
  const len0 = WASM_VECTOR_LEN;
  getInt32Memory0()[arg0 / 4 + 1] = len0;
  getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}

export function __wbindgen_throw(arg0, arg1) {
  throw new Error(getStringFromWasm0(arg0, arg1));
}
