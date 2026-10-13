import { expect, test } from 'bun:test';
import { readFile } from 'node:fs/promises';
import init, { evaluate, execute, parse } from 'val-wasm';

const wasm = await init({
  module_or_path: await readFile(
    new URL('../packages/val-wasm/val_bg.wasm', import.meta.url)
  ),
});

test.each([
  ['(2 ^ 128 + 1) - 2 ^ 128', '1'],
  ['(1 / 3) * 3', '1'],
  ['sqrt(4)', '2'],
])('executes arbitrary precision arithmetic: %s', (source, value) => {
  expect(execute(source, '')).toEqual({
    errors: [],
    exitCode: 0,
    output: '',
    value,
  });
});

test('executes input and output and resets the runtime', () => {
  expect(
    execute(
      'foo = input("foo"); print(foo); println(input()); input()',
      'bar\r\nbaz\nqux'
    )
  ).toEqual({
    errors: [],
    exitCode: 0,
    output: 'foobarbaz\n',
    value: 'qux',
  });

  expect(execute('input()', '')).toEqual({
    errors: [],
    exitCode: 0,
    output: '',
    value: '',
  });
});

test.each([
  [
    '"foo',
    [
      {
        kind: 'Parser',
        message: "found end of input expected something else, or '\"'",
        range: { start: 4, end: 4 },
      },
    ],
  ],
  [
    '"😀"; foo',
    [
      {
        kind: 'Evaluator',
        message: 'Undefined variable `foo`',
        range: { start: 6, end: 9 },
      },
    ],
  ],
])('returns errors with UTF-16 ranges: %s', (source, errors) => {
  expect(execute(source, '')).toEqual({
    errors,
    exitCode: 1,
    output: '',
    value: undefined,
  });
});

test('stops execution at exit and preserves output', () => {
  expect(execute('println("foo"); exit(7); println("bar")', '')).toEqual({
    errors: [],
    exitCode: 7,
    output: 'foo\n',
    value: undefined,
  });
});

test('parses and evaluates through the generated bindings', () => {
  expect(parse('"😀"')).toEqual({
    kind: 'statements',
    range: { start: 0, end: 4 },
    children: [
      {
        kind: 'expression',
        range: { start: 0, end: 4 },
        children: [
          {
            kind: 'string',
            range: { start: 0, end: 4 },
            children: [],
          },
        ],
      },
    ],
  });

  expect(evaluate('1 + 2')).toBe('3');
});

test.each([0, 1, 16, 17])('allocates and frees %i aligned bytes', (size) => {
  wasm.free(0);

  const pointer = wasm.malloc(size);

  expect(pointer).not.toBe(0);
  expect(pointer % 16).toBe(0);

  new Uint8Array(wasm.memory.buffer, pointer, size).fill(42);
  wasm.free(pointer);
});

test('reallocates null, grows and shrinks while preserving bytes', () => {
  const bytes = new TextEncoder().encode('foobar');
  const pointer = wasm.realloc(0, bytes.length);

  expect(pointer).not.toBe(0);
  expect(pointer % 16).toBe(0);
  new Uint8Array(wasm.memory.buffer, pointer, bytes.length).set(bytes);

  const grown = wasm.realloc(pointer, 128 * 1024);

  expect(grown).not.toBe(0);
  expect(grown % 16).toBe(0);
  expect(new Uint8Array(wasm.memory.buffer, grown, bytes.length)).toEqual(
    bytes
  );

  const shrunk = wasm.realloc(grown, 3);

  expect(shrunk).not.toBe(0);
  expect(shrunk % 16).toBe(0);
  expect(new Uint8Array(wasm.memory.buffer, shrunk, 3)).toEqual(
    bytes.subarray(0, 3)
  );

  const empty = wasm.realloc(shrunk, 0);

  expect(empty).not.toBe(0);
  expect(empty % 16).toBe(0);
  wasm.free(empty);
});

test.each([0x7fffffe1, 0x7fffffff, 0xfffffff0, 0xffffffff])(
  'rejects allocation size %i without changing existing memory',
  (size) => {
    const bytes = new TextEncoder().encode('foo');
    const pointer = wasm.malloc(bytes.length);

    expect(pointer).not.toBe(0);
    new Uint8Array(wasm.memory.buffer, pointer, bytes.length).set(bytes);

    expect(wasm.malloc(size)).toBe(0);
    expect(wasm.realloc(0, size)).toBe(0);
    expect(wasm.realloc(pointer, size)).toBe(0);
    expect(new Uint8Array(wasm.memory.buffer, pointer, bytes.length)).toEqual(
      bytes
    );
    wasm.free(pointer);
  }
);
