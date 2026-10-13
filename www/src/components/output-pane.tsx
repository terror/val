import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from '@/components/ui/resizable';
import type { ExecutionState } from '@/lib/types';

interface OutputPaneProps {
  execution: ExecutionState;
  input: string;
  onInputChange: (value: string) => void;
}

export const OutputPane = ({
  execution,
  input,
  onInputChange,
}: OutputPaneProps) => {
  const result = execution.status === 'complete' ? execution.result : undefined;

  const status =
    execution.status === 'running'
      ? 'Running…'
      : execution.status === 'stopped'
        ? 'Stopped'
        : result
          ? `Finished with exit code ${result.exitCode}`
          : 'Run the program to see its output.';

  return (
    <section
      aria-label='Program output'
      className='flex h-full min-h-0 flex-col overflow-hidden'
    >
      <ResizablePanelGroup
        autoSaveId='val-output-panel-layout'
        direction='vertical'
        className='min-h-0 flex-1'
      >
        <ResizablePanel id='output-panel' defaultSize={70} minSize={20}>
          <div className='h-full overflow-auto p-3'>
            <p role='status' className='text-muted-foreground mb-3 text-xs'>
              {status}
            </p>
            {result && (
              <>
                {result.output && (
                  <pre className='font-mono text-sm break-words whitespace-pre-wrap'>
                    {result.output}
                  </pre>
                )}
                {result.value !== undefined && (
                  <pre className='font-mono text-sm break-words whitespace-pre-wrap'>
                    {result.value}
                  </pre>
                )}
                {result.errors.length > 0 && (
                  <div
                    role='alert'
                    className='mt-3 text-red-700 dark:text-red-400'
                  >
                    {result.errors.map((error, index) => (
                      <pre
                        key={index}
                        className='font-mono text-sm break-words whitespace-pre-wrap'
                      >
                        {error.message}
                      </pre>
                    ))}
                  </div>
                )}
              </>
            )}
          </div>
        </ResizablePanel>
        <ResizableHandle aria-label='Resize input pane' />
        <ResizablePanel
          id='input-panel'
          defaultSize={30}
          minSize={20}
          collapsible
          collapsedSize={0}
        >
          <div className='flex h-full min-h-0 flex-col gap-2 overflow-hidden p-3'>
            <label
              htmlFor='program-input'
              className='text-muted-foreground shrink-0 text-xs'
            >
              Input
            </label>
            <textarea
              id='program-input'
              value={input}
              onChange={(event) => onInputChange(event.target.value)}
              disabled={execution.status === 'running'}
              spellCheck={false}
              placeholder='One line per input() call'
              className='bg-background placeholder:text-muted-foreground min-h-0 w-full flex-1 resize-none rounded border p-2 font-mono text-sm outline-none disabled:opacity-50'
            />
          </div>
        </ResizablePanel>
      </ResizablePanelGroup>
    </section>
  );
};
