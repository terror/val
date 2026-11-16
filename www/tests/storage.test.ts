import { readStorage, writeStorage } from '@/lib/storage';
import { expect, test } from 'bun:test';

test('storage errors are not thrown', () => {
  const fail = () => {
    throw new Error('foo');
  };

  globalThis.localStorage = {
    getItem: fail,
    setItem: fail,
  } as unknown as Storage;

  expect(readStorage('foo')).toBeNull();
  expect(() => writeStorage('foo', 'bar')).not.toThrow();
});
