/* tslint:disable */
/* eslint-disable */
/**
* @param {any} input_table
* @returns {any}
*/
export function test_json_input(input_table: any): any;
/**
* @param {string} query
* @param {(string)[]} columns
* @returns {boolean}
*/
export function check_syntax(query: string, columns: (string)[]): boolean;
/**
* @param {string} query
* @param {any} input_table
* @returns {any}
*/
export function fitl_filter(query: string, input_table: any): any;
/**
* @param {string} query
* @param {any} input_table
* @param {string} table_format
* @returns {any}
*/
export function fitl_filter_custom_table_format(query: string, input_table: any, table_format: string): any;
/**
*/
export enum TableFormat {
  Custom = 0,
  JsonArray = 1,
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly test_json_input: (a: number, b: number) => void;
  readonly check_syntax: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly fitl_filter: (a: number, b: number, c: number, d: number) => void;
  readonly fitl_filter_custom_table_format: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
