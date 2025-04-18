import { highlightExtension } from '@/lib/cm-highlight-extension';
import { useEditorSettings } from '@/providers/editor-settings-provider';
import { rust } from '@codemirror/lang-rust';
import {
  bracketMatching,
  defaultHighlightStyle,
  indentOnInput,
  syntaxHighlighting,
} from '@codemirror/language';
import { Diagnostic, lintGutter, linter } from '@codemirror/lint';
import CodeMirror, { EditorState, EditorView } from '@uiw/react-codemirror';
import { forwardRef, useCallback, useImperativeHandle, useRef } from 'react';
import { ValError } from 'val-wasm';

interface EditorProps {
  errors: ValError[];
  onChange?: (value: string, viewUpdate: any) => void;
  value: string;
}

export interface EditorRef {
  view: EditorView | null;
}

export const Editor = forwardRef<EditorRef, EditorProps>(
  ({ value, errors, onChange }, ref) => {
    const { settings } = useEditorSettings();

    const viewRef = useRef<EditorView | null>(null);

    useImperativeHandle(ref, () => ({
      get view() {
        return viewRef.current;
      },
    }));

    const createEditorTheme = useCallback(
      () =>
        EditorView.theme({
          '&': {
            height: '100%',
            fontSize: `${settings.fontSize}px`,
            display: 'flex',
            flexDirection: 'column',
          },
          '&.cm-editor': {
            height: '100%',
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
        }),
      [settings]
    );

    const createExtensions = useCallback(() => {
      const extensions = [
        EditorState.tabSize.of(settings.tabSize),
        bracketMatching(),
        highlightExtension,
        indentOnInput(),
        lintGutter(),
        linter(diagnostics()),
        rust(),
        syntaxHighlighting(defaultHighlightStyle),
      ];

      if (settings.lineWrapping) {
        extensions.push(EditorView.lineWrapping);
      }

      return extensions;
    }, [settings]);

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
                source: error.kind.toString(),
              };
            } catch (e) {
              console.warn('Failed to create diagnostic:', e, error);

              return {
                from: 0,
                to: 0,
                severity: 'error',
                message: error.message,
                source: error.kind.toString(),
              };
            }
          });
        },
        [errors]
      );

    return (
      <CodeMirror
        value={value}
        theme={createEditorTheme()}
        basicSetup={{
          foldGutter: false,
          highlightActiveLineGutter: false,
          lineNumbers: settings.lineNumbers,
        }}
        height='100%'
        extensions={createExtensions()}
        onCreateEditor={(view) => {
          viewRef.current = view;
        }}
        onChange={onChange}
        className='h-full'
      />
    );
  }
);
