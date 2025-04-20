import { highlightExtension } from '@/lib/highlight';
import { ValError } from '@/lib/types';
import { useEditorSettings } from '@/providers/editor-settings-provider';
import { rust } from '@codemirror/lang-rust';
import {
  bracketMatching,
  defaultHighlightStyle,
  indentOnInput,
  syntaxHighlighting,
} from '@codemirror/language';
import { Diagnostic, linter } from '@codemirror/lint';
import { vim } from '@replit/codemirror-vim';
import CodeMirror, { EditorState, EditorView } from '@uiw/react-codemirror';
import {
  forwardRef,
  useCallback,
  useEffect,
  useImperativeHandle,
  useRef,
} from 'react';

interface EditorProps {
  errors: ValError[];
  onChange?: (value: string, viewUpdate: any) => void;
  onEditorReady?: (view: EditorView) => void;
  value: string;
}

export interface EditorRef {
  view: EditorView | null;
}

export const Editor = forwardRef<EditorRef, EditorProps>(
  ({ value, errors, onChange, onEditorReady }, ref) => {
    const { settings } = useEditorSettings();

    const viewRef = useRef<EditorView | null>(null);

    useImperativeHandle(ref, () => ({
      get view() {
        return viewRef.current;
      },
    }));

    useEffect(() => {
      if (viewRef.current && onEditorReady) {
        onEditorReady(viewRef.current);
      }
    }, [viewRef.current, onEditorReady]);

    const createExtensions = useCallback(() => {
      const extensions = [
        EditorState.tabSize.of(settings.tabSize),
        bracketMatching(),
        highlightExtension,
        indentOnInput(),
        linter(diagnostics()),
        rust(),
        syntaxHighlighting(defaultHighlightStyle),
      ];

      if (settings.lineWrapping) {
        extensions.push(EditorView.lineWrapping);
      }

      if (settings.keybindings === 'vim') {
        extensions.push(vim());
      }

      return extensions;
    }, [settings]);

    const createTheme = useCallback(
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
          '.cm-line': {
            padding: '0 10px',
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

    const handleEditorCreate = (view: EditorView) => {
      viewRef.current = view;

      if (onEditorReady) {
        onEditorReady(view);
      }
    };

    return (
      <CodeMirror
        value={value}
        theme={createTheme()}
        basicSetup={{
          foldGutter: false,
          highlightActiveLineGutter: false,
          lineNumbers: settings.lineNumbers,
        }}
        height='100%'
        extensions={createExtensions()}
        onCreateEditor={handleEditorCreate}
        onChange={onChange}
        className='h-full'
      />
    );
  }
);

Editor.displayName = 'Editor';
