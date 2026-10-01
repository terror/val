import { useEditorSettings } from '@/contexts/editor-settings-context';
import { highlightExtension } from '@/lib/highlight';
import type { Range, ValError } from '@/lib/types';
import { rust } from '@codemirror/lang-rust';
import {
  HighlightStyle,
  bracketMatching,
  indentOnInput,
  syntaxHighlighting,
} from '@codemirror/language';
import { type Diagnostic, linter } from '@codemirror/lint';
import { EditorState, type Extension } from '@codemirror/state';
import { EditorView } from '@codemirror/view';
import { tags } from '@lezer/highlight';
import { vim } from '@replit/codemirror-vim';
import { useCallback, useMemo } from 'react';

interface UseEditorExtensionsOptions {
  darkMode: boolean;
  errors: ValError[];
  highlight: Range | undefined;
}

export function useEditorExtensions({
  darkMode,
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
      createEditorTheme(settings.fontSize, darkMode),
      highlightExtension(highlight),
      indentOnInput(),
      linter(diagnostics),
      rust(),
      syntaxHighlighting(highlightStyle),
    ];

    if (settings.keybindings === 'vim') {
      extensions.push(vim());
    }

    if (settings.lineWrapping) {
      extensions.push(EditorView.lineWrapping);
    }

    return extensions;
  }, [
    darkMode,
    diagnostics,
    highlight,
    settings.fontSize,
    settings.keybindings,
    settings.lineWrapping,
    settings.tabSize,
  ]);
}

const highlightStyle = HighlightStyle.define([
  { tag: tags.keyword, color: 'var(--syntax-keyword)', fontWeight: '600' },
  { tag: tags.name, color: 'var(--editor-foreground)' },
  { tag: tags.function(tags.variableName), color: 'var(--syntax-function)' },
  { tag: tags.typeName, color: 'var(--syntax-type)' },
  { tag: [tags.number, tags.bool, tags.atom], color: 'var(--syntax-number)' },
  { tag: [tags.string, tags.character], color: 'var(--syntax-string)' },
  { tag: tags.comment, color: 'var(--syntax-comment)', fontStyle: 'italic' },
  { tag: tags.operator, color: 'var(--syntax-type)' },
  { tag: tags.punctuation, color: 'var(--syntax-punctuation)' },
  { tag: tags.invalid, color: 'var(--syntax-error)' },
]);

function createEditorTheme(fontSize: number, darkMode: boolean): Extension {
  return EditorView.theme(
    {
      '&': {
        height: '100%',
        fontSize: `${fontSize}px`,
        display: 'flex',
        flexDirection: 'column',
        backgroundColor: 'var(--editor-background)',
        color: 'var(--editor-foreground)',
      },
      '&.cm-focused': {
        outline: 'none',
      },
      '.cm-cursor, .cm-dropCursor': {
        borderLeftColor: 'var(--editor-cursor)',
      },
      '&.cm-focused > .cm-scroller > .cm-selectionLayer .cm-selectionBackground, .cm-selectionBackground, .cm-content::selection, .cm-content ::selection':
        {
          backgroundColor: 'var(--selection-background)',
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
        caretColor: 'var(--editor-cursor)',
      },
      '.cm-gutters': {
        backgroundColor: 'var(--editor-background)',
        borderRight: '1px solid var(--editor-gutter-border)',
        color: 'var(--editor-gutter-foreground)',
        paddingRight: '8px',
      },
      '.cm-activeLineGutter, .cm-activeLine': {
        backgroundColor: 'var(--editor-active-line)',
      },
      '.cm-matchingBracket': {
        backgroundColor: 'var(--editor-highlight-background)',
        color: 'var(--editor-foreground)',
      },
      '.cm-nonmatchingBracket': {
        backgroundColor: 'var(--editor-nonmatching-bracket-background)',
        color: 'var(--syntax-error)',
      },
      '.cm-tooltip, .cm-panels': {
        backgroundColor: 'var(--editor-background)',
        borderColor: 'var(--editor-gutter-border)',
        color: 'var(--editor-foreground)',
      },
      '.cm-fat-cursor': {
        backgroundColor: 'var(--editor-fat-cursor)',
        borderLeft: 'none',
        width: '0.6em',
      },
      '.cm-cursor-secondary': {
        backgroundColor: 'var(--editor-cursor-secondary)',
      },
    },
    { dark: darkMode }
  );
}

function clamp(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(value, max));
}
