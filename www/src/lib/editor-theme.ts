import type { Extension } from '@codemirror/state';
import { EditorView } from '@codemirror/view';

export function createEditorTheme(
  fontSize: number,
  darkMode: boolean
): Extension {
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
