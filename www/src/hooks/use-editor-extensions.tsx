import { useEditorSettings } from '@/contexts/editor-settings-context';
import { highlightExtension } from '@/lib/highlight';
import type { Range, ValError } from '@/lib/types';
import { rust } from '@codemirror/lang-rust';
import {
  bracketMatching,
  defaultHighlightStyle,
  indentOnInput,
  syntaxHighlighting,
} from '@codemirror/language';
import { type Diagnostic, linter } from '@codemirror/lint';
import { EditorState, type Extension } from '@codemirror/state';
import { EditorView } from '@codemirror/view';
import { vim } from '@replit/codemirror-vim';
import { useCallback, useMemo } from 'react';

interface UseEditorExtensionsOptions {
  errors: ValError[];
  highlight: Range | undefined;
}

export function useEditorExtensions({
  errors,
  highlight,
}: UseEditorExtensionsOptions): Extension[] {
  const { settings } = useEditorSettings();

  const diagnostics = useCallback(
    (view: EditorView): Diagnostic[] =>
      errors.map((error) => {
        const from = clamp(error.range.start, 0, view.state.doc.length);
        const to = clamp(error.range.end, from, view.state.doc.length);

        return {
          from,
          to,
          severity: 'error',
          message: error.message,
          source: error.kind.toString(),
        };
      }),
    [errors]
  );

  return useMemo(() => {
    const extensions: Extension[] = [
      EditorState.tabSize.of(settings.tabSize),
      bracketMatching(),
      createEditorTheme(settings.fontSize),
      highlightExtension(highlight),
      indentOnInput(),
      linter(diagnostics),
      rust(),
      syntaxHighlighting(defaultHighlightStyle),
    ];

    if (settings.keybindings === 'vim') {
      extensions.push(vim());
    }

    if (settings.lineWrapping) {
      extensions.push(EditorView.lineWrapping);
    }

    return extensions;
  }, [
    diagnostics,
    highlight,
    settings.fontSize,
    settings.keybindings,
    settings.lineWrapping,
    settings.tabSize,
  ]);
}

function createEditorTheme(fontSize: number): Extension {
  return EditorView.theme({
    '&': {
      height: '100%',
      fontSize: `${fontSize}px`,
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
  });
}

function clamp(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(value, max));
}
