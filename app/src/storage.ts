import { useEffect, useState } from "react";

// UI preferences live in localStorage. Access can throw in restricted
// webviews, so every call is guarded and the app works without it.
const LAST_VAULT_KEY = "mdnotes.lastVault";
const EXPLICIT_VAULT_KEY = "mdnotes.explicitVaultOpen";

function rawGet(key: string): string | null {
  try {
    return window.localStorage.getItem(key);
  } catch {
    return null;
  }
}

function rawSet(key: string, value: string) {
  try {
    window.localStorage.setItem(key, value);
  } catch {
    // ignore
  }
}

function rawRemove(key: string) {
  try {
    window.localStorage.removeItem(key);
  } catch {
    // ignore
  }
}

export function requestVaultOpen(path: string) {
  rawSet(LAST_VAULT_KEY, path);
  rawSet(EXPLICIT_VAULT_KEY, path);
}

export function getRememberedVault(): string | null {
  return rawGet(LAST_VAULT_KEY);
}

export function hasPendingVaultOpen(): boolean {
  const pending = rawGet(EXPLICIT_VAULT_KEY);
  const last = rawGet(LAST_VAULT_KEY);
  if (!pending || pending !== last) {
    if (pending) rawRemove(EXPLICIT_VAULT_KEY);
    return false;
  }
  return true;
}

export const storage = {
  get(key: string): string | null {
    if (key === LAST_VAULT_KEY) {
      const value = rawGet(LAST_VAULT_KEY);
      const pending = rawGet(EXPLICIT_VAULT_KEY);
      if (!value || pending !== value) return null;
      rawRemove(EXPLICIT_VAULT_KEY);
      return value;
    }
    return rawGet(key);
  },
  set(key: string, value: string) {
    rawSet(key, value);
  },
  remove(key: string) {
    rawRemove(key);
    if (key === LAST_VAULT_KEY) rawRemove(EXPLICIT_VAULT_KEY);
  },
};

export function useStoredState<T extends string>(key: string, fallback: T) {
  const [value, setValue] = useState<T>(() => (storage.get(key) as T | null) ?? fallback);
  useEffect(() => storage.set(key, value), [key, value]);
  return [value, setValue] as const;
}
