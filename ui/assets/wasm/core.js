let wasm;

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

let WASM_VECTOR_LEN = 0;

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}
/**
*/
export const XFontsData = Object.freeze({ RobotoRegular:0,"0":"RobotoRegular", });
/**
*/
export const XImageFormat = Object.freeze({ Png:0,"0":"Png", });
/**
*/
export class XCore {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(XCore.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_xcore_free(ptr);
    }
    /**
    * @param {string} nickname
    * @param {string} description
    */
    constructor(nickname, description) {
        const ptr0 = passStringToWasm0(nickname, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(description, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.xcore_new(ptr0, len0, ptr1, len1);
        return XCore.__wrap(ret);
    }
}
/**
*/
export class XDebug {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_xdebug_free(ptr);
    }
    /**
    * @param {XCore} x
    */
    static show_character(x) {
        _assertClass(x, XCore);
        wasm.xdebug_show_character(x.__wbg_ptr);
    }
}
/**
*/
export class XFont {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(XFont.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_xfont_free(ptr);
    }
    /**
    * @returns {number}
    */
    get data() {
        const ret = wasm.__wbg_get_xfont_data(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set data(arg0) {
        wasm.__wbg_set_xfont_data(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get font_size() {
        const ret = wasm.__wbg_get_xfont_font_size(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set font_size(arg0) {
        wasm.__wbg_set_xfont_font_size(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get offset_x() {
        const ret = wasm.__wbg_get_xfont_offset_x(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set offset_x(arg0) {
        wasm.__wbg_set_xfont_offset_x(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get offset_y() {
        const ret = wasm.__wbg_get_xfont_offset_y(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set offset_y(arg0) {
        wasm.__wbg_set_xfont_offset_y(this.__wbg_ptr, arg0);
    }
    /**
    * @param {number} data
    * @param {number} font_size
    * @param {number} offset_x
    * @param {number} offset_y
    */
    constructor(data, font_size, offset_x, offset_y) {
        const ret = wasm.xfont_new(data, font_size, offset_x, offset_y);
        return XFont.__wrap(ret);
    }
}
/**
*/
export class XImage {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(XImage.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_ximage_free(ptr);
    }
    /**
    * @returns {number}
    */
    get format() {
        const ret = wasm.__wbg_get_xfont_data(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set format(arg0) {
        wasm.__wbg_set_xfont_data(this.__wbg_ptr, arg0);
    }
    /**
    * @param {number} format
    * @param {Uint8Array} bytes
    */
    constructor(format, bytes) {
        const ptr0 = passArray8ToWasm0(bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.ximage_from_bytes(format, ptr0, len0);
        return ret === 0 ? undefined : XImage.__wrap(ret);
    }
    /**
    * @returns {Uint8Array}
    */
    data() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.ximage_data(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
/**
*/
export class XImageGen {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_ximagegen_free(ptr);
    }
    /**
    * @param {XCore} x
    * @param {number} format
    * @param {XImage} i1
    * @param {XImage} i2
    * @returns {XImage | undefined}
    */
    static combine2(x, format, i1, i2) {
        _assertClass(x, XCore);
        _assertClass(i1, XImage);
        var ptr0 = i1.__destroy_into_raw();
        _assertClass(i2, XImage);
        var ptr1 = i2.__destroy_into_raw();
        const ret = wasm.ximagegen_combine2(x.__wbg_ptr, format, ptr0, ptr1);
        return ret === 0 ? undefined : XImage.__wrap(ret);
    }
    /**
    * @param {XCore} x
    * @param {number} format
    * @param {Uint8Array} color
    * @param {number} width
    * @param {number} height
    * @returns {XImage | undefined}
    */
    static color(x, format, color, width, height) {
        _assertClass(x, XCore);
        const ptr0 = passArray8ToWasm0(color, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.ximagegen_color(x.__wbg_ptr, format, ptr0, len0, width, height);
        return ret === 0 ? undefined : XImage.__wrap(ret);
    }
    /**
    * @param {XCore} x
    * @param {number} format
    * @param {XFont} font
    * @param {Uint8Array} bg_color
    * @param {Uint8Array} text_color
    * @param {string} text
    * @param {number} width
    * @param {number} height
    * @returns {XImage | undefined}
    */
    static text(x, format, font, bg_color, text_color, text, width, height) {
        _assertClass(x, XCore);
        _assertClass(font, XFont);
        var ptr0 = font.__destroy_into_raw();
        const ptr1 = passArray8ToWasm0(bg_color, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passArray8ToWasm0(text_color, wasm.__wbindgen_malloc);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len3 = WASM_VECTOR_LEN;
        const ret = wasm.ximagegen_text(x.__wbg_ptr, format, ptr0, ptr1, len1, ptr2, len2, ptr3, len3, width, height);
        return ret === 0 ? undefined : XImage.__wrap(ret);
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_log_9656b3e7b23d6ed7 = function(arg0, arg1) {
        console.log(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, maybe_memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedInt32Memory0 = null;
    cachedUint8Memory0 = null;


    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(input) {
    if (wasm !== undefined) return wasm;

    if (typeof input === 'undefined') {
        input = new URL('core_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await input, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync }
export default __wbg_init;
