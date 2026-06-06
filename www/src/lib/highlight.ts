import type { Range } from '@/lib/types';
import type { Extension } from '@codemirror/state';
import { Decoration, EditorView, ViewPlugin } from '@codemirror/view';

const highlightMark = Decoration.mark({ class: 'cm-highlighted-node' });

export const highlightExtension = (range: Range | undefined): Extension => {
  if (!range || range.start >= range.end) {
    return [];
  }

  const currentRange = range;

  return [
    EditorView.decorations.of((view) => {
      const from = clamp(currentRange.start, 0, view.state.doc.length);
      const to = trimTrailingWhitespace(
        from,
        clamp(currentRange.end, from, view.state.doc.length),
        view
      );

      if (to <= from) {
        return Decoration.none;
      }

      return Decoration.set([highlightMark.range(from, to)]);
    }),
    ViewPlugin.fromClass(
      class {
        constructor(view: EditorView) {
          const from = clamp(currentRange.start, 0, view.state.doc.length);

          queueMicrotask(() => {
            view.dispatch({
              effects: EditorView.scrollIntoView(from, { y: 'center' }),
            });
          });
        }
      }
    ),
  ];
};

function trimTrailingWhitespace(
  from: number,
  to: number,
  view: EditorView
): number {
  for (let pos = to - 1; pos >= from; pos--) {
    const char = view.state.doc.sliceString(pos, pos + 1);

    if (!/\s/.test(char)) {
      return pos + 1;
    }
  }

  return from;
}

function clamp(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(value, max));
}
