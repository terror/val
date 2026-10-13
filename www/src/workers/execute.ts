import { failedExecution } from '@/lib/execution';
import type { ExecutionRequest } from '@/lib/types';
import init, { execute } from 'val-wasm';
import wasmUrl from 'val-wasm/val_bg.wasm?url';

self.onmessage = async ({ data }: MessageEvent<ExecutionRequest>) => {
  try {
    await init({ module_or_path: wasmUrl });
    self.postMessage(execute(data.code, data.input));
  } catch (error) {
    self.postMessage(failedExecution(error));
  }
};
