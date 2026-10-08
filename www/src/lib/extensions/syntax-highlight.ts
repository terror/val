import { rust } from '@codemirror/lang-rust';
import { HighlightStyle, syntaxHighlighting } from '@codemirror/language';
import type { Extension } from '@codemirror/state';
import { tags } from '@lezer/highlight';

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

export const syntaxHighlightExtension: Extension = [
  rust(),
  syntaxHighlighting(highlightStyle),
];
