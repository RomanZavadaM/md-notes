import { useEffect, useState } from "react";

// UI preferences live in localStorage. Access can throw in restricted
// webviews, so every call is guarded and the app works without it.
export const storage = {
  get(key: string): string | null {
    try {
      return window.localStorage.getItem(key);
    } catch {
      return null;
    }
  },
  set(key: string, value: string) {
    try {
      window.localStorage.setItem(key, value);
    } catch {
      // ignore
    }
  },
  remove(key: string) {
    try {
      window.localStorage.removeItem(key);
    } catch {
      // ignore
    }
  },
};

export function useStoredState<T extends string>(key: string, fallback: T) {
  const [value, setValue] = useState<T>(() => (storage.get(key) as T | null) ?? fallback);
  useEffect(() => storage.set(key, value), [key, value]);
  return [value, setValue] as const;
}
