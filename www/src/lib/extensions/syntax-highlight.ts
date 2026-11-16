import { type Extension, RangeSetBuilder } from '@codemirror/state';
import {
  Decoration,
  type DecorationSet,
  EditorView,
  ViewPlugin,
  type ViewUpdate,
} from '@codemirror/view';
import { type HighlightKind, highlight } from 'val-wasm';

const marks: Record<HighlightKind, Decoration | undefined> = {
  Boolean: Decoration.mark({ class: 'cm-val-number' }),
  Comment: Decoration.mark({ class: 'cm-val-comment' }),
  Function: Decoration.mark({ class: 'cm-val-function' }),
  Identifier: undefined,
  Invalid: Decoration.mark({ class: 'cm-val-invalid' }),
  Keyword: Decoration.mark({ class: 'cm-val-keyword' }),
  Number: Decoration.mark({ class: 'cm-val-number' }),
  Operator: Decoration.mark({ class: 'cm-val-operator' }),
  Punctuation: Decoration.mark({ class: 'cm-val-punctuation' }),
  String: Decoration.mark({ class: 'cm-val-string' }),
};

function decorations(view: EditorView): DecorationSet {
  const builder = new RangeSetBuilder<Decoration>();

  for (const { kind, range } of highlight(view.state.doc.toString())) {
    const mark = marks[kind];

    if (mark) {
      builder.add(range.start, range.end, mark);
    }
  }

  return builder.finish();
}

const plugin = ViewPlugin.fromClass(
  class {
    decorations: DecorationSet;

    constructor(view: EditorView) {
      this.decorations = decorations(view);
    }

    update(update: ViewUpdate) {
      if (update.docChanged) {
        this.decorations = decorations(update.view);
      }
    }
  },
  { decorations: (value) => value.decorations }
);

const theme = EditorView.baseTheme({
  '.cm-val-comment': { color: 'var(--syntax-comment)', fontStyle: 'italic' },
  '.cm-val-function': { color: 'var(--syntax-function)' },
  '.cm-val-invalid': { color: 'var(--syntax-error)' },
  '.cm-val-keyword': { color: 'var(--syntax-keyword)', fontWeight: '600' },
  '.cm-val-number': { color: 'var(--syntax-number)' },
  '.cm-val-operator': { color: 'var(--syntax-type)' },
  '.cm-val-punctuation': { color: 'var(--syntax-punctuation)' },
  '.cm-val-string': { color: 'var(--syntax-string)' },
});

export const syntaxHighlightExtension: Extension = [plugin, theme];
