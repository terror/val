import type { Execution } from 'val-wasm';

export type ExecutionRequest = {
  code: string;
  input: string;
};

export type ExecutionState =
  | { status: 'idle' }
  | { status: 'running' | 'stopped'; code: string }
  | { status: 'complete'; code: string; result: Execution };
