import { indentService } from '@codemirror/language';
import { EditorState, type Extension } from '@codemirror/state';
import { highlight } from 'val-wasm';

const closing = new Set([')', ']', '}']);
const opening = new Set(['(', '[', '{']);

export const indentationExtension: Extension = [
  indentService.of((context, pos) => {
    const text = context.state.doc.sliceString(0, pos);

    const depth = highlight(text)
      .filter(({ kind }) => kind === 'Punctuation')
      .map(({ range }) => text.slice(range.start, range.end))
      .reduce((depth, bracket) => {
        if (opening.has(bracket)) {
          return depth + 1;
        }

        if (closing.has(bracket)) {
          return Math.max(depth - 1, 0);
        }

        return depth;
      }, 0);

    const dedent = closing.has(context.textAfterPos(pos).trimStart()[0])
      ? 1
      : 0;

    return Math.max(depth - dedent, 0) * context.unit;
  }),
  EditorState.languageData.of(() => [{ indentOnInput: /^\s*[)\]}]$/ }]),
];
