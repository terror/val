import { Editor } from '@/components/editor';
import { EditorSettingsDialog } from '@/components/editor-settings-dialog';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import type { Extension } from '@codemirror/state';

interface EditorPaneProps {
  currentExample: string;
  examples: Record<string, string>;
  extensions: Extension[];
  onChange: (value: string) => void;
  onExampleChange: (value: string) => void;
  value: string;
}

export const EditorPane = ({
  currentExample,
  examples,
  extensions,
  onChange,
  onExampleChange,
  value,
}: EditorPaneProps) => {
  return (
    <div className='flex h-full min-h-0 flex-col overflow-hidden'>
      <div className='flex items-center justify-between border-b bg-gray-50 px-2 py-1'>
        <Select value={currentExample} onValueChange={onExampleChange}>
          <SelectTrigger className='h-7 w-36 bg-white text-sm'>
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

        <EditorSettingsDialog />
      </div>

      <div className='flex-1 overflow-hidden'>
        <Editor value={value} onChange={onChange} extensions={extensions} />
      </div>
    </div>
  );
};
