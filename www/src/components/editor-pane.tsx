import { Editor } from '@/components/editor';
import { EditorResetDialog } from '@/components/editor-reset-dialog';
import { EditorSettingsDialog } from '@/components/editor-settings-dialog';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import type { Extension } from '@codemirror/state';
import type { EditorView, ViewUpdate } from '@codemirror/view';

interface EditorPaneProps {
  currentExample: string;
  examples: Record<string, string>;
  extensions: Extension[];
  onChange: (value: string) => void;
  onCreateEditor: (view: EditorView) => void;
  onExampleChange: (value: string) => void;
  onUpdate: (update: ViewUpdate) => void;
  value: string;
}

export const EditorPane = ({
  currentExample,
  examples,
  extensions,
  onChange,
  onCreateEditor,
  onExampleChange,
  onUpdate,
  value,
}: EditorPaneProps) => {
  return (
    <div className='flex h-full min-h-0 flex-col overflow-hidden'>
      <div className='bg-muted/50 flex shrink-0 items-center justify-between gap-1 border-b px-2 py-1'>
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
                <SelectItem key={key} value={key}>
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
        <EditorSettingsDialog />
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
