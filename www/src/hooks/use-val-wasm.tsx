import { useEffect, useState } from 'react';
import init from 'val-wasm';

type State =
  | { status: 'loading' }
  | { status: 'ready' }
  | { status: 'error'; error: string };

interface UseValWasm {
  error: string | undefined;
  loaded: boolean;
  loading: boolean;
}

export function useValWasm(): UseValWasm {
  const [state, setState] = useState<State>({ status: 'loading' });

  useEffect(() => {
    let cancelled = false;

    const initialize = async () => {
      try {
        await init();

        if (!cancelled) {
          setState({ status: 'ready' });
        }
      } catch (err) {
        if (!cancelled) {
          setState({
            status: 'error',
            error: `Failed to initialize val: ${
              err instanceof Error ? err.message : String(err)
            }`,
          });
        }
      }
    };

    initialize();

    return () => {
      cancelled = true;
    };
  }, []);

  return {
    error: state.status === 'error' ? state.error : undefined,
    loaded: state.status === 'ready',
    loading: state.status === 'loading',
  };
}
