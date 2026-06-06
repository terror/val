import { useEffect, useState } from 'react';
import init from 'val-wasm';

interface UseValWasm {
  error: string | undefined;
  loaded: boolean;
  loading: boolean;
}

export function useValWasm(): UseValWasm {
  const [error, setError] = useState<string | undefined>(undefined);
  const [loaded, setLoaded] = useState<boolean>(false);
  const [loading, setLoading] = useState<boolean>(true);

  useEffect(() => {
    let cancelled = false;

    const initialize = async () => {
      try {
        setLoading(true);
        await init();

        if (!cancelled) {
          setLoaded(true);
        }
      } catch (err) {
        if (!cancelled) {
          setError(
            `Failed to initialize val: ${
              err instanceof Error ? err.message : String(err)
            }`
          );
        }
      } finally {
        if (!cancelled) {
          setLoading(false);
        }
      }
    };

    initialize();

    return () => {
      cancelled = true;
    };
  }, []);

  return { error, loaded, loading };
}
