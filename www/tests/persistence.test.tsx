import { afterEach, expect, mock, spyOn, test } from 'bun:test';
import type { ReactElement } from 'react';
import { act, create } from 'react-test-renderer';

import {
  defaultSettings,
  useEditorSettings,
} from '../src/contexts/editor-settings-context';
import { usePersistedDoc } from '../src/hooks/use-persisted-doc';
import { usePersistedState } from '../src/hooks/use-persisted-state';
import { EditorSettingsProvider } from '../src/providers/editor-settings-provider';

mock.module('../src/lib/examples', () => ({
  examples: { factorial: 'foo', bar: 'baz' },
}));

mock.module('../src/hooks/use-val-wasm', () => ({
  useValWasm: () => ({ loaded: false, loading: true, error: undefined }),
}));

mock.module('val-wasm', () => ({ parse: () => undefined }));

const { default: App } = await import('../src/App');

const windowDescriptor = Object.getOwnPropertyDescriptor(globalThis, 'window');
const renderers: ReturnType<typeof create>[] = [];

afterEach(() => {
  act(() => renderers.splice(0).forEach((renderer) => renderer.unmount()));

  if (windowDescriptor) {
    Object.defineProperty(globalThis, 'window', windowDescriptor);
  } else {
    Reflect.deleteProperty(globalThis, 'window');
  }

  mock.restore();
});

class Storage {
  values = new Map<string, string>();

  constructor(failure?: 'access' | 'read' | 'write') {
    Object.defineProperty(globalThis, 'window', {
      configurable: true,
      value: {
        get localStorage() {
          if (failure === 'access') {
            throw new Error('foo');
          }

          return storage;
        },
        matchMedia: () => ({
          matches: false,
          addEventListener: mock(),
          removeEventListener: mock(),
        }),
      },
    });

    const storage = {
      getItem: (key: string) => {
        if (failure === 'read') {
          throw new Error('foo');
        }

        return this.values.get(key) ?? null;
      },
      setItem: (key: string, value: string) => {
        if (failure === 'write') {
          throw new Error('foo');
        }

        this.values.set(key, value);
      },
    };
  }
}

function render(element: ReactElement) {
  const renderer = create(null);
  renderers.push(renderer);
  act(() => renderer.update(element));
  return renderer;
}

function Doc() {
  const [value, setValue] = usePersistedDoc('foo', 'bar');
  return (
    <input value={value} onChange={(event) => setValue(event.target.value)} />
  );
}

function Settings() {
  const { settings, updateSettings } = useEditorSettings();
  return (
    <button onClick={() => updateSettings({ fontSize: 16 })}>
      {JSON.stringify(settings)}
    </button>
  );
}

test('documents restore saved content including empty strings', () => {
  function check(saved: string | null, expected: string) {
    const storage = new Storage();
    if (saved !== null) {
      storage.values.set('foo', saved);
    }

    const renderer = render(<Doc />);
    expect(renderer.root.findByType('input').props.value).toBe(expected);
    expect(storage.values.get('foo')).toBe(expected);
  }

  check(null, 'bar');
  check('', '');
  check('baz', 'baz');
});

test('document edits are saved and survive remounting', () => {
  const storage = new Storage();
  const renderer = render(<Doc />);

  act(() =>
    renderer.root.findByType('input').props.onChange({ target: { value: '' } })
  );

  expect(storage.values.get('foo')).toBe('');
  act(() => renderer.unmount());
  expect(render(<Doc />).root.findByType('input').props.value).toBe('');
});

test.each(['access', 'read', 'write'] as const)(
  'documents and settings remain editable after storage %s errors',
  (failure) => {
    const storage = new Storage(failure);
    const savedSettings = { ...defaultSettings, fontSize: 12 };
    storage.values.set('foo', 'foo');
    storage.values.set('editor-settings', JSON.stringify(savedSettings));
    const renderer = render(
      <EditorSettingsProvider>
        <Doc />
        <Settings />
      </EditorSettingsProvider>
    );

    expect(renderer.root.findByType('input').props.value).toBe(
      failure === 'write' ? 'foo' : 'bar'
    );
    expect(
      JSON.parse(renderer.root.findByType('button').children.join(''))
    ).toEqual(failure === 'write' ? savedSettings : defaultSettings);

    act(() => {
      renderer.root
        .findByType('input')
        .props.onChange({ target: { value: 'baz' } });
      renderer.root.findByType('button').props.onClick();
    });

    expect(renderer.root.findByType('input').props.value).toBe('baz');
    expect(
      JSON.parse(renderer.root.findByType('button').children.join(''))
    ).toEqual({ ...defaultSettings, fontSize: 16 });

    if (failure === 'read') {
      expect(storage.values.get('foo')).toBe('baz');
      expect(
        JSON.parse(storage.values.get('editor-settings') ?? 'null')
      ).toEqual({
        ...defaultSettings,
        fontSize: 16,
      });
    } else {
      expect(storage.values.get('foo')).toBe('foo');
      expect(
        JSON.parse(storage.values.get('editor-settings') ?? 'null')
      ).toEqual(savedSettings);
    }
  }
);

test('saved objects receive new defaults and support partial updates', () => {
  const storage = new Storage();
  storage.values.set('foo', '{"foo":3}');

  function Foo() {
    const [state, setState] = usePersistedState('foo', { foo: 1, bar: 2 });
    return (
      <button onClick={() => setState({ bar: 4 })}>
        {JSON.stringify(state)}
      </button>
    );
  }

  const renderer = render(<Foo />);
  expect(JSON.parse(storage.values.get('foo') ?? 'null')).toEqual({
    foo: 3,
    bar: 2,
  });
  act(() => renderer.root.findByType('button').props.onClick());
  expect(JSON.parse(storage.values.get('foo') ?? 'null')).toEqual({
    foo: 3,
    bar: 4,
  });
});

test('invalid saved object shapes fall back to defaults', () => {
  spyOn(console, 'warn').mockImplementation(() => undefined);

  function Foo() {
    const [state] = usePersistedState('foo', { bar: 1 });
    return <span>{JSON.stringify(state)}</span>;
  }

  function check(saved: string) {
    const storage = new Storage();
    storage.values.set('foo', saved);
    render(<Foo />);
    expect(JSON.parse(storage.values.get('foo') ?? 'null')).toEqual({ bar: 1 });
  }

  ['foo', 'null', '[]', '1', 'true', '"bar"'].forEach(check);
});

test('saved editor settings validate fields and restore defaults', () => {
  spyOn(console, 'warn').mockImplementation(() => undefined);

  function check(saved: string, expected = defaultSettings) {
    const storage = new Storage();
    storage.values.set('editor-settings', saved);
    const renderer = render(
      <EditorSettingsProvider>
        <Settings />
      </EditorSettingsProvider>
    );
    expect(
      JSON.parse(renderer.root.findByType('button').children.join(''))
    ).toEqual(expected);
    expect(JSON.parse(storage.values.get('editor-settings') ?? 'null')).toEqual(
      expected
    );
  }

  ['foo', 'null', '[]', '1', 'true', '"bar"'].forEach((saved) => check(saved));
  check('{"fontSize":16}', { ...defaultSettings, fontSize: 16 });
  check(
    '{"fontSize":18,"keybindings":"vim","lineNumbers":false,"lineWrapping":false,"tabSize":8}',
    {
      fontSize: 18,
      keybindings: 'vim',
      lineNumbers: false,
      lineWrapping: false,
      tabSize: 8,
    }
  );
  check(
    '{"fontSize":"18","keybindings":null,"lineNumbers":"false","lineWrapping":0,"tabSize":"8"}'
  );
  check('{"fontSize":0,"keybindings":"foo","tabSize":-1}');
  check('{"fontSize":1e400,"tabSize":2.5}');
  check('{"fontSize":12,"tabSize":4,"foo":true}', {
    ...defaultSettings,
    fontSize: 12,
    tabSize: 4,
  });
});

test.each(['access', 'read', 'write'] as const)(
  'the app renders after storage %s errors',
  (failure) => {
    new Storage(failure);
    expect(
      render(
        <EditorSettingsProvider>
          <App />
        </EditorSettingsProvider>
      ).toJSON()
    ).not.toBeNull();
  }
);

test('only saved examples present in the example list are restored', () => {
  function check(saved: string, expected: string) {
    const storage = new Storage();
    storage.values.set('val-editor-example', saved);
    render(
      <EditorSettingsProvider>
        <App />
      </EditorSettingsProvider>
    );
    expect(storage.values.get('val-editor-example')).toBe(expected);
  }

  check('bar', 'bar');
  check('baz', 'factorial');
  check('__proto__', 'factorial');
});
