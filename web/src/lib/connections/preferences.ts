/** Public device hints only. Credentials and protected drafts are never restored. */
export interface ConnectionPreferences {
  version: 1;
  selected: string;
  routes: Record<string, string>;
}
const validId = (id: string) => id === "local" || /^[0-9a-f]{32}$/.test(id);
export const preferenceKey = "jj-store/connections/v1";
export function readPreferences(value: string | null): ConnectionPreferences {
  const empty: ConnectionPreferences = {
    version: 1,
    selected: "local",
    routes: {},
  };
  if (!value || value.length > 72 * 1024) return empty;
  try {
    const saved = JSON.parse(value);
    if (
      saved?.version !== 1 ||
      typeof saved.selected !== "string" ||
      !validId(saved.selected) ||
      !saved.routes ||
      Array.isArray(saved.routes) ||
      typeof saved.routes !== "object" ||
      Object.keys(saved.routes).length > 17
    )
      return empty;
    const routes: Record<string, string> = {};
    for (const [id, hash] of Object.entries(saved.routes)) {
      if (
        !validId(id) ||
        typeof hash !== "string" ||
        hash.length > 4096 ||
        !hash.startsWith("#/") ||
        /[\u0000-\u001f]/.test(hash)
      )
        return empty;
      routes[id] = hash;
    }
    return { version: 1, selected: saved.selected, routes };
  } catch {
    return empty;
  }
}
/** Scope layout preferences even when two installations use the same repository ID. */
export function connectionStorage(
  storage: Storage | undefined,
  scope: string,
): Storage | undefined {
  if (!storage) return undefined;
  const prefix = `jj-store/connection/${scope}/`;
  const keys = () =>
    Array.from({ length: storage.length }, (_, i) => storage.key(i)).filter(
      (key): key is string => !!key?.startsWith(prefix),
    );
  return {
    get length() {
      return keys().length;
    },
    key: (index) => keys()[index]?.slice(prefix.length) ?? null,
    getItem: (key) => storage.getItem(prefix + key),
    setItem: (key, value) => storage.setItem(prefix + key, value),
    removeItem: (key) => storage.removeItem(prefix + key),
    clear() {
      for (const key of keys()) storage.removeItem(key);
    },
  };
}
