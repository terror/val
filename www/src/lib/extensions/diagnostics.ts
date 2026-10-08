import type { ValError } from '@/lib/types';
import { type Diagnostic, linter } from '@codemirror/lint';
import type { Extension } from '@codemirror/state';

export const createDiagnosticsExtension = (errors: ValError[]): Extension =>
  linter((view): Diagnostic[] =>
    errors.map((error) => {
      const from = clamp(error.range.start, 0, view.state.doc.length);
      const to = clamp(error.range.end, from, view.state.doc.length);

      return {
        from,
        to,
        severity: 'error',
        message: error.message,
        source: error.kind.toString(),
      };
    })
  );

function clamp(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(value, max));
}
