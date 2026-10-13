import { failedExecution } from '@/lib/execution';
import type { ExecutionRequest, ExecutionState } from '@/lib/types';
import { useCallback, useEffect, useRef, useState } from 'react';
import type { Execution } from 'val-wasm';

export function useValExecution() {
  const worker = useRef<Worker>();

  const [state, setState] = useState<ExecutionState>({ status: 'idle' });

  const terminate = useCallback(() => {
    worker.current?.terminate();
    worker.current = undefined;
  }, []);

  useEffect(() => terminate, [terminate]);

  const run = useCallback(
    (code: string, input: string) => {
      terminate();
      setState({ status: 'running', code });

      try {
        const current = new Worker(
          new URL('../workers/execute.ts', import.meta.url),
          { type: 'module' }
        );

        worker.current = current;

        const complete = (result: Execution) => {
          if (worker.current !== current) {
            return;
          }

          terminate();
          setState({ status: 'complete', code, result });
        };

        current.onmessage = ({ data }: MessageEvent<Execution>) => {
          complete(data);
        };

        current.onerror = (event) => {
          event.preventDefault();
          complete(
            failedExecution(event.message || 'Program execution failed')
          );
        };

        current.onmessageerror = () => {
          complete(failedExecution('Failed to receive program output'));
        };

        current.postMessage({ code, input } satisfies ExecutionRequest);
      } catch (error) {
        terminate();
        setState({ status: 'complete', code, result: failedExecution(error) });
      }
    },
    [terminate]
  );

  const stop = useCallback(() => {
    terminate();
    setState((previous) =>
      previous.status === 'running'
        ? { status: 'stopped', code: previous.code }
        : previous
    );
  }, [terminate]);

  return { run, state, stop };
}
