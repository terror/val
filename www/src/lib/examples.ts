const files = import.meta.glob<string>('@examples/*.val', {
  eager: true,
  import: 'default',
  query: '?raw',
});

const entries = Object.entries(files)
  .map(([path, source]): [string, string] => [
    path.slice(path.lastIndexOf('/') + 1).replace(/\.val$/, ''),
    source,
  ])
  .sort(([a], [b]) => a.localeCompare(b));

export const examples = Object.fromEntries(entries);
