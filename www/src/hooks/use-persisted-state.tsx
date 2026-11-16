import { readStorage, writeStorage } from '@/lib/storage';
import { useCallback, useEffect, useState } from 'react';

export function usePersistedState<T extends object>(
  key: string,
  initialValue: T
): [T, (action: Partial<T> | ((prevState: T) => Partial<T>)) => void] {
  const [state, setFullState] = useState<T>(() => {
    const savedValue = readStorage(key);

    if (savedValue !== null) {
      try {
        return JSON.parse(savedValue);
      } catch (error) {
        console.warn(`Error reading ${key} from localStorage:`, error);
        return initialValue;
      }
    }

    return initialValue;
  });

  useEffect(() => {
    writeStorage(key, JSON.stringify(state));
  }, [key, state]);

  const setState = useCallback(
    (action: Partial<T> | ((prevState: T) => Partial<T>)) => {
      setFullState((prevState) => ({
        ...prevState,
        ...(typeof action === 'function' ? action(prevState) : action),
      }));
    },
    []
  );

  return [state, setState];
}
