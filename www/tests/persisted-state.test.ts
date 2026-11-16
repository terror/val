import { restorePersistedState } from '@/hooks/use-persisted-state';
import { expect, test } from 'bun:test';

test.each([
  ['null', { foo: 1, bar: 'baz' }],
  ['1', { foo: 1, bar: 'baz' }],
  ['[2]', { foo: 1, bar: 'baz' }],
  ['{"foo": 2}', { foo: 2, bar: 'baz' }],
  ['{"foo": "bar", "bar": "bob"}', { foo: 1, bar: 'bob' }],
  ['{"baz": 2}', { foo: 1, bar: 'baz' }],
])('restores persisted state: %s', (savedValue, expected) => {
  expect(restorePersistedState(savedValue, { foo: 1, bar: 'baz' })).toEqual(
    expected
  );
});
