import { useEffect, useState } from "react";

// UI preferences live in localStorage. Access can throw in restricted
// webviews, so every call is guarded and the app works without it.
//
// `mdnotes.lastVault` is intentionally write-only for now. Older builds
// reopened it automatically during startup, which could immediately recurse
// through a very large folder and saturate CPU/disk before the user could
// recover the UI. We keep recording the path for a future explicit
// "reopen recent vault" action, but startup must stay idle until the user
// chooses a vault.
const UNSAFE_STARTUP_KEYS = new Set(["mdnotes.lastVault"]);

export const storage = {
  get(key: string): string | null {
    if (UNSAFE_STARTUP_KEYS.has(key)) return null;
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
