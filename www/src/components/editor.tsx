import { useEditorSettings } from '@/contexts/editor-settings-context';
import type { Extension } from '@codemirror/state';
import type { EditorView, ViewUpdate } from '@codemirror/view';
import CodeMirror from '@uiw/react-codemirror';
import { useMemo } from 'react';

interface EditorProps {
  extensions: Extension[];
  onChange: (value: string) => void;
  onCreateEditor: (view: EditorView) => void;
  onUpdate: (update: ViewUpdate) => void;
  value: string;
}

export const Editor = ({
  extensions,
  onChange,
  onCreateEditor,
  onUpdate,
  value,
}: EditorProps) => {
  const { settings } = useEditorSettings();

  const basicSetup = useMemo(
    () => ({
      autocompletion: false,
      closeBrackets: false,
      foldGutter: false,
      highlightSelectionMatches: false,
      lineNumbers: settings.lineNumbers,
    }),
    [settings.lineNumbers]
  );

  return (
    <CodeMirror
      theme='none'
      value={value}
      extensions={extensions}
      basicSetup={basicSetup}
      height='100%'
      onChange={onChange}
      onCreateEditor={onCreateEditor}
      onUpdate={onUpdate}
      className='h-full'
    />
  );
};
