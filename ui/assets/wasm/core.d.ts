/* tslint:disable */
/* eslint-disable */
/**
*/
export enum XFontsData {
  RobotoRegular = 0,
}
/**
*/
export enum XImageFormat {
  Png = 0,
}
/**
*/
export class XCore {
  free(): void;
/**
* @param {string} nickname
* @param {string} description
*/
  constructor(nickname: string, description: string);
}
/**
*/
export class XDebug {
  free(): void;
/**
* @param {XCore} x
*/
  static show_character(x: XCore): void;
}
/**
*/
export class XFont {
  free(): void;
/**
* @param {number} data
* @param {number} font_size
* @param {number} offset_x
* @param {number} offset_y
*/
  constructor(data: number, font_size: number, offset_x: number, offset_y: number);
/**
*/
  data: number;
/**
*/
  font_size: number;
/**
*/
  offset_x: number;
/**
*/
  offset_y: number;
}
/**
*/
export class XImage {
  free(): void;
/**
* @param {number} format
* @param {Uint8Array} bytes
*/
  constructor(format: number, bytes: Uint8Array);
/**
* @returns {Uint8Array}
*/
  data(): Uint8Array;
/**
*/
  format: number;
}
/**
*/
export class XImageGen {
  free(): void;
/**
* @param {XCore} x
* @param {number} format
* @param {XImage} i1
* @param {XImage} i2
* @returns {XImage | undefined}
*/
  static combine2(x: XCore, format: number, i1: XImage, i2: XImage): XImage | undefined;
/**
* @param {XCore} x
* @param {number} format
* @param {Uint8Array} color
* @param {number} width
* @param {number} height
* @returns {XImage | undefined}
*/
  static color(x: XCore, format: number, color: Uint8Array, width: number, height: number): XImage | undefined;
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
  static text(x: XCore, format: number, font: XFont, bg_color: Uint8Array, text_color: Uint8Array, text: string, width: number, height: number): XImage | undefined;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_ximagegen_free: (a: number) => void;
  readonly ximagegen_combine2: (a: number, b: number, c: number, d: number) => number;
  readonly ximagegen_color: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly ximagegen_text: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => number;
  readonly __wbg_xdebug_free: (a: number) => void;
  readonly xdebug_show_character: (a: number) => void;
  readonly __wbg_xcore_free: (a: number) => void;
  readonly xcore_new: (a: number, b: number, c: number, d: number) => number;
  readonly __wbg_xfont_free: (a: number) => void;
  readonly __wbg_get_xfont_data: (a: number) => number;
  readonly __wbg_set_xfont_data: (a: number, b: number) => void;
  readonly __wbg_get_xfont_font_size: (a: number) => number;
  readonly __wbg_set_xfont_font_size: (a: number, b: number) => void;
  readonly __wbg_get_xfont_offset_x: (a: number) => number;
  readonly __wbg_set_xfont_offset_x: (a: number, b: number) => void;
  readonly __wbg_get_xfont_offset_y: (a: number) => number;
  readonly __wbg_set_xfont_offset_y: (a: number, b: number) => void;
  readonly xfont_new: (a: number, b: number, c: number, d: number) => number;
  readonly __wbg_ximage_free: (a: number) => void;
  readonly ximage_from_bytes: (a: number, b: number, c: number) => number;
  readonly ximage_data: (a: number, b: number) => void;
  readonly __wbg_get_ximage_format: (a: number) => number;
  readonly __wbg_set_ximage_format: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
