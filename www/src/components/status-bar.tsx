import type { ValError } from '@/lib/types';
import type { EditorState } from '@codemirror/state';
import { CircleCheck, CircleX } from 'lucide-react';

interface StatusBarProps {
  errors: ValError[];
  state: EditorState | undefined;
}

export const StatusBar = ({ errors, state }: StatusBarProps) => {
  const head = state?.selection.main.head ?? 0;
  const line = state?.doc.lineAt(head);
  const row = line?.number ?? 1;
  const column = head - (line?.from ?? 0) + 1;

  return (
    <div className='bg-muted/40 text-muted-foreground flex min-h-7 shrink-0 items-center gap-x-3 overflow-x-auto border-t px-2 font-mono text-xs whitespace-nowrap'>
      <div role='status' className='flex items-center gap-x-3 font-medium'>
        {errors.length === 0 ? (
          <span
            title='No errors'
            className='text-emerald-700 dark:text-emerald-400'
          >
            <CircleCheck className='h-3.5 w-3.5' aria-hidden='true' />
            <span className='sr-only'>No errors</span>
          </span>
        ) : (
          <span className='flex items-center gap-x-1.5 text-red-700 dark:text-red-400'>
            <CircleX className='h-3.5 w-3.5' aria-hidden='true' />
            <span>
              {errors.length.toLocaleString()}{' '}
              {errors.length === 1 ? 'error' : 'errors'}
            </span>
          </span>
        )}
      </div>
      <span aria-label={`Line ${row}, column ${column}`}>
        {row}:{column}
      </span>
    </div>
  );
};
