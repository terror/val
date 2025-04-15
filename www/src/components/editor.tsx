import { Diagnostic, lintGutter, linter } from '@codemirror/lint';
import CodeMirror, { EditorView } from '@uiw/react-codemirror';
import { ParseError } from 'packages/val-wasm/val';
import { forwardRef, useCallback, useImperativeHandle, useRef } from 'react';

interface EditorProps {
  errors: ParseError[];
  onChange?: (value: string, viewUpdate: any) => void;
  value: string;
}

export interface EditorRef {
  view: EditorView | null;
}

export const Editor = forwardRef<EditorRef, EditorProps>(
  ({ value, errors, onChange }, ref) => {
    const viewRef = useRef<EditorView | null>(null);

    useImperativeHandle(ref, () => ({
      get view() {
        return viewRef.current;
      },
    }));

    const theme = EditorView.theme({
      '&': {
        height: '100%',
        fontSize: '14px',
        display: 'flex',
        flexDirection: 'column',
      },
      '&.cm-editor': {
        height: '100%',
      },
      '&.cm-focused': {
        outline: 'none',
      },
      '.cm-scroller': {
        overflow: 'auto',
        flex: '1 1 auto',
        fontFamily:
          'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace',
      },
      '.cm-content': {
        padding: '10px 0',
      },
      '.cm-line': {
        padding: '0 10px',
      },
      '.cm-gutters': {
        backgroundColor: 'transparent',
        borderRight: 'none',
        paddingRight: '8px',
      },
      '.cm-activeLineGutter': {
        backgroundColor: 'rgba(59, 130, 246, 0.1)',
      },
      '.cm-activeLine': {
        backgroundColor: 'rgba(59, 130, 246, 0.1)',
      },
      '.cm-fat-cursor': {
        backgroundColor: 'rgba(59, 130, 246, 0.5)',
        borderLeft: 'none',
        width: '0.6em',
      },
      '.cm-cursor-secondary': {
        backgroundColor: 'rgba(59, 130, 246, 0.3)',
      },
    });

    const diagnostics = () =>
      useCallback(
        (_view: EditorView): Diagnostic[] => {
          return errors.map((error) => {
            try {
              return {
                from: error.range.start,
                to: error.range.end,
                severity: 'error',
                message: error.message,
                source: 'PARSER',
              };
            } catch (e) {
              console.warn('Failed to create diagnostic:', e, error);

              return {
                from: 0,
                to: 0,
                severity: 'error',
                message: error.message,
                source: 'PARSER',
              };
            }
          });
        },
        [errors]
      );

    return (
      <CodeMirror
        value={value}
        theme={theme}
        height='100%'
        extensions={[lintGutter(), linter(diagnostics())]}
        onCreateEditor={(view) => {
          viewRef.current = view;
        }}
        onChange={onChange}
        className='h-full'
      />
    );
  }
);
