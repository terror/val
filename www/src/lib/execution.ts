import type { Execution } from 'val-wasm';

export function failedExecution(error: unknown): Execution {
  return {
    errors: [
      {
        kind: 'Evaluator',
        message: error instanceof Error ? error.message : String(error),
        range: { start: 0, end: 0 },
      },
    ],
    exitCode: 1,
    output: '',
  };
}
