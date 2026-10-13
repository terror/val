/* tslint:disable */
/* eslint-disable */
export function start(): void;
export function execute(source: string, input: string): Execution;
export function parse(source: string): ParseResult;
export type ParseResult = { status: "error"; errors: ValError[] } | { status: "ok"; ast: AstNode };

export interface ValError {
    kind: ErrorKind;
    message: string;
    range: Range;
}

export type ErrorKind = "Evaluator" | "Parser";

export interface Range {
    start: number;
    end: number;
}

export interface AstNode {
    kind: string;
    range: Range;
    children: AstNode[];
}

export interface Execution {
    errors: ValError[];
    exitCode: number;
    output: string;
    value?: string;
}


export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly execute: (a: number, b: number, c: number, d: number) => [number, number, number];
  readonly parse: (a: number, b: number) => [number, number, number];
  readonly start: () => void;
  readonly abort: () => void;
  readonly free: (a: number) => void;
  readonly malloc: (a: number) => number;
  readonly realloc: (a: number, b: number) => number;
  readonly __wbindgen_malloc_command_export: (a: number, b: number) => number;
  readonly __wbindgen_realloc_command_export: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free_command_export: (a: number, b: number, c: number) => void;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __externref_table_dealloc_command_export: (a: number) => void;
  readonly __wbindgen_start: () => void;
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
