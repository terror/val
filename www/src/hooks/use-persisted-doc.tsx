import { storage } from '@/lib/storage';
import { useCallback, useEffect, useState } from 'react';

export function usePersistedDoc(
  key: string,
  fallback: string
): [string, (value: string) => void] {
  const [value, setValue] = useState(() => storage.getItem(key) ?? fallback);

  useEffect(() => {
    storage.setItem(key, value);
  }, [key, value]);

  return [value, useCallback((next: string) => setValue(next), [])];
}
