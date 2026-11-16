import { readStorage, writeStorage } from '@/lib/storage';
import { useEffect, useState } from 'react';

export function usePersistedDoc(
  key: string,
  fallback: string
): [string, (value: string) => void] {
  const [value, setValue] = useState<string>(() => {
    const stored = readStorage(key);

    return stored ?? fallback;
  });

  useEffect(() => {
    writeStorage(key, value);
  }, [key, value]);

  return [value, setValue];
}
