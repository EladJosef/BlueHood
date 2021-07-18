/* tslint:disable */
/* eslint-disable */
/**
* used for encrypt file with key and file path
* @param {string} path
* @param {string} encryption_key
*/
export function encrypt_file(path: string, encryption_key: string): void;
/**
* used for decrypt file with key and file path
* @param {string} path
* @param {string} encryption_key
*/
export function decrypt_file(path: string, encryption_key: string): void;
/**
* used for encrypt data
* @param {Uint8Array} file_data
* @param {string} encryption_key
* @param {string} file_name
* @returns {Uint8Array}
*/
export function encrypt_data(file_data: Uint8Array, encryption_key: string, file_name: string): Uint8Array;
/**
* used for decrypt data
* @param {Uint8Array} file_data
* @param {string} encryption_key
* @returns {Uint8Array}
*/
export function decrypt_data(file_data: Uint8Array, encryption_key: string): Uint8Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly encrypt_file: (a: number, b: number, c: number, d: number) => void;
  readonly decrypt_file: (a: number, b: number, c: number, d: number) => void;
  readonly encrypt_data: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly decrypt_data: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
