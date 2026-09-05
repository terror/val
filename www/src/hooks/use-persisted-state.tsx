import { storage } from '@/lib/storage';
import { useCallback, useEffect, useState } from 'react';

export function usePersistedState<T extends object>(
  key: string,
  initialValue: T,
  options?: {
    serialize?: (value: T) => string;
    deserialize?: (value: string) => Partial<T>;
  }
): [T, (action: Partial<T> | ((prevState: T) => Partial<T>)) => void] {
  const [state, setFullState] = useState<T>(() => {
    const savedValue = storage.getItem(key);

    if (savedValue !== null) {
      try {
        const saved: unknown = options?.deserialize
          ? options.deserialize(savedValue)
          : JSON.parse(savedValue);

        return saved !== null &&
          typeof saved === 'object' &&
          !Array.isArray(saved)
          ? { ...initialValue, ...saved }
          : initialValue;
      } catch (error) {
        console.warn(`Error reading ${key} from localStorage:`, error);
        return initialValue;
      }
    }

    return initialValue;
  });

  useEffect(() => {
    try {
      storage.setItem(
        key,
        options?.serialize ? options.serialize(state) : JSON.stringify(state)
      );
    } catch (error) {
      console.warn(`Error saving ${key} to localStorage:`, error);
    }
  }, [key, state, options]);

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
