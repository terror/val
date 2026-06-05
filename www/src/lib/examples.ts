const files = import.meta.glob<string>('@examples/*.val', {
  as: 'raw',
  eager: true,
});

const entries = Object.entries(files)
  .map(([path, source]): [string, string] => [
    path
      .split('/')
      .pop()!
      .replace(/\.val$/, ''),
    source,
  ])
  .sort(([a], [b]) => a.localeCompare(b));

export const examples = Object.fromEntries(entries);
