import type { Range } from '@/lib/types';
import { StateEffect, StateField, type Text } from '@codemirror/state';
import { Decoration, type DecorationSet, EditorView } from '@codemirror/view';

const highlightMark = Decoration.mark({ class: 'cm-highlighted-node' });

export const highlightEffect = StateEffect.define<Range | null>();

export const highlightExtension = StateField.define<DecorationSet>({
  create: () => Decoration.none,
  update(decorations, transaction) {
    decorations = decorations.map(transaction.changes);

    for (const effect of transaction.effects) {
      if (!effect.is(highlightEffect)) {
        continue;
      }

      const range = effect.value;

      if (!range || range.start >= range.end) {
        decorations = Decoration.none;
        continue;
      }

      const from = clamp(range.start, 0, transaction.newDoc.length);

      const to = trimTrailingWhitespace(
        from,
        clamp(range.end, from, transaction.newDoc.length),
        transaction.newDoc
      );

      decorations =
        to > from
          ? Decoration.set([highlightMark.range(from, to)])
          : Decoration.none;
    }

    return decorations;
  },
  provide: (field) => EditorView.decorations.from(field),
});

function trimTrailingWhitespace(from: number, to: number, doc: Text): number {
  for (let pos = to - 1; pos >= from; pos--) {
    const char = doc.sliceString(pos, pos + 1);

    if (!/\s/.test(char)) {
      return pos + 1;
    }
  }

  return from;
}

function clamp(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(value, max));
}
