import { useEditorSettings } from '@/contexts/editor-settings-context';
import type { Extension } from '@codemirror/state';
import CodeMirror from '@uiw/react-codemirror';

interface EditorProps {
  extensions: Extension[];
  onChange: (value: string) => void;
  value: string;
}

export const Editor = ({ extensions, onChange, value }: EditorProps) => {
  const { settings } = useEditorSettings();

  return (
    <CodeMirror
      value={value}
      extensions={extensions}
      basicSetup={{
        autocompletion: false,
        closeBrackets: false,
        foldGutter: false,
        highlightActiveLineGutter: false,
        lineNumbers: settings.lineNumbers,
      }}
      height='100%'
      onChange={onChange}
      className='h-full'
    />
  );
};
