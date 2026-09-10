import { useCallback, useEffect, useState } from 'react';

export function usePersistedState<T extends object>(
  key: string,
  initialValue: T
): [T, (action: Partial<T> | ((prevState: T) => Partial<T>)) => void] {
  const [state, setFullState] = useState<T>(() => {
    const savedValue = localStorage.getItem(key);

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
    try {
      localStorage.setItem(key, JSON.stringify(state));
    } catch (error) {
      console.warn(`Error saving ${key} to localStorage:`, error);
    }
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
