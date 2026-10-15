import { Editor } from '@/components/editor';
import { EditorResetDialog } from '@/components/editor-reset-dialog';
import { EditorSettingsDialog } from '@/components/editor-settings-dialog';
import { Button } from '@/components/ui/button';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import type { Extension } from '@codemirror/state';
import type { EditorView, ViewUpdate } from '@codemirror/view';
import { Play, Square } from 'lucide-react';

interface EditorPaneProps {
  currentExample: string;
  examples: Record<string, string>;
  extensions: Extension[];
  onChange: (value: string) => void;
  onCreateEditor: (view: EditorView) => void;
  onExampleChange: (value: string) => void;
  onRun: () => void;
  onStop: () => void;
  onUpdate: (update: ViewUpdate) => void;
  running: boolean;
  value: string;
}

export const EditorPane = ({
  currentExample,
  examples,
  extensions,
  onChange,
  onCreateEditor,
  onExampleChange,
  onRun,
  onStop,
  onUpdate,
  running,
  value,
}: EditorPaneProps) => {
  return (
    <div className='flex h-full min-h-0 flex-col overflow-hidden'>
      <div className='bg-muted/50 flex shrink-0 flex-wrap items-center justify-between gap-1 border-b px-2 py-1'>
        <div className='flex min-w-0 items-center gap-1'>
          <Select value={currentExample} onValueChange={onExampleChange}>
            <SelectTrigger
              aria-label='Example program'
              className='bg-background dark:bg-background dark:hover:bg-background w-36 cursor-pointer text-sm data-[size=default]:h-7'
            >
              <SelectValue placeholder='Select example' />
            </SelectTrigger>
            <SelectContent>
              {Object.keys(examples).map((key) => (
                <SelectItem key={key} value={key} className='cursor-pointer'>
                  {key}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
          <EditorResetDialog
            example={currentExample}
            onReset={() => onChange(examples[currentExample])}
          />
        </div>
        <div className='ml-auto flex items-center gap-1'>
          <Button
            variant='ghost'
            size='icon'
            className='dark:hover:bg-accent h-7 w-7 cursor-pointer'
            onClick={running ? onStop : onRun}
            title={running ? 'Stop program' : 'Run program (Ctrl/⌘+Enter)'}
            aria-label={running ? 'Stop program' : 'Run program'}
            aria-keyshortcuts={running ? undefined : 'Control+Enter Meta+Enter'}
          >
            {running ? (
              <Square className='size-4' aria-hidden='true' />
            ) : (
              <Play className='size-4' aria-hidden='true' />
            )}
          </Button>
          <EditorSettingsDialog />
        </div>
      </div>

      <div className='flex-1 overflow-hidden'>
        <Editor
          value={value}
          onChange={onChange}
          onCreateEditor={onCreateEditor}
          onUpdate={onUpdate}
          extensions={extensions}
        />
      </div>
    </div>
  );
};
