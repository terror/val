import GithubSlugger from 'github-slugger';
import { fromMarkdown } from 'mdast-util-from-markdown';
import { toString } from 'mdast-util-to-string';

import builtins from '../../../docs/builtins.md?raw';
import installation from '../../../docs/installation.md?raw';
import language from '../../../docs/language.md?raw';
import numbers from '../../../docs/numbers.md?raw';
import playground from '../../../docs/playground.md?raw';
import usage from '../../../docs/usage.md?raw';

export const documentation = [
  { id: 'installation', title: 'Installation', content: installation },
  { id: 'usage', title: 'Command line', content: usage },
  { id: 'language', title: 'Language', content: language },
  { id: 'numbers', title: 'Numbers and precision', content: numbers },
  { id: 'builtins', title: 'Built-ins', content: builtins },
  { id: 'playground', title: 'Playground', content: playground },
].map((section) => {
  const slugger = new GithubSlugger();

  const headings = fromMarkdown(section.content).children.flatMap((node) => {
    if (node.type !== 'heading' || node.depth !== 2) {
      return [];
    }

    const title = toString(node);

    return [
      {
        id: `${section.id}-${slugger.slug(title)}`,
        line: node.position?.start.line,
        title,
      },
    ];
  });

  return { ...section, headings };
});

export type DocumentationSection = (typeof documentation)[number];
