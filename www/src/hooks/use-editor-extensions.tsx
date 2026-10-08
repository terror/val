import { useEditorSettings } from '@/contexts/editor-settings-context';
import { createEditorTheme } from '@/lib/editor-theme';
import { createDiagnosticsExtension } from '@/lib/extensions/diagnostics';
import { highlightExtension } from '@/lib/extensions/highlight';
import { syntaxHighlightExtension } from '@/lib/extensions/syntax-highlight';
import type { ValError } from '@/lib/types';
import { bracketMatching, indentOnInput } from '@codemirror/language';
import { EditorState, type Extension } from '@codemirror/state';
import { EditorView } from '@codemirror/view';
import { vim } from '@replit/codemirror-vim';
import { useMemo } from 'react';

const baseExtensions: Extension[] = [
  bracketMatching(),
  highlightExtension,
  indentOnInput(),
  syntaxHighlightExtension,
];

interface UseEditorExtensionsOptions {
  darkMode: boolean;
  errors: ValError[];
}

export function useEditorExtensions({
  darkMode,
  errors,
}: UseEditorExtensionsOptions): Extension[] {
  const { settings } = useEditorSettings();

  const diagnostics = useMemo(
    () => createDiagnosticsExtension(errors),
    [errors]
  );

  const theme = useMemo(
    () => createEditorTheme(settings.fontSize, darkMode),
    [settings.fontSize, darkMode]
  );

  const keybindings = useMemo(
    () => (settings.keybindings === 'vim' ? vim() : []),
    [settings.keybindings]
  );

  return useMemo(
    () => [
      baseExtensions,
      EditorState.tabSize.of(settings.tabSize),
      diagnostics,
      theme,
      keybindings,
      settings.lineWrapping ? EditorView.lineWrapping : [],
    ],
    [diagnostics, theme, keybindings, settings.lineWrapping, settings.tabSize]
  );
}
