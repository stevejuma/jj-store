import {
  resumeGlobalSettingsHash,
  rememberGlobalSettingsHash,
} from "./global-settings-route-codec.js";

export interface StoreRouteBrowser {
  hash(): string;
  push(hash: string, entryId?: string): void;
  replace(hash: string, entryId?: string): void;
  entryId?(): string | undefined;
  go(delta: -1 | 1): void;
  listen(listener: () => void): () => void;
}

export function browserNavigation(): StoreRouteBrowser | undefined {
  if (typeof window === "undefined") return undefined;
  let storage: Storage | undefined;
  try {
    storage = window.localStorage;
  } catch {
    /* Optional device preference. */
  }
  const resumed = resumeGlobalSettingsHash(window.location.hash, storage);
  if (resumed !== window.location.hash)
    window.history.replaceState(null, "", resumed);
  rememberGlobalSettingsHash(resumed, storage);
  const write = (hash: string, replace: boolean, entryId?: string) => {
    window.history[replace ? "replaceState" : "pushState"](
      { jjStoreEntry: entryId },
      "",
      hash,
    );
    rememberGlobalSettingsHash(hash, storage);
  };
  return {
    hash: () => window.location.hash,
    push: (hash, id) => write(hash, false, id),
    replace: (hash, id) => write(hash, true, id),
    entryId: () =>
      typeof window.history.state?.jjStoreEntry === "string"
        ? window.history.state.jjStoreEntry
        : undefined,
    go: (delta) => window.history.go(delta),
    listen(notify) {
      const listener = () => {
        rememberGlobalSettingsHash(window.location.hash, storage);
        notify();
      };
      window.addEventListener("hashchange", listener);
      window.addEventListener("popstate", listener);
      return () => {
        window.removeEventListener("hashchange", listener);
        window.removeEventListener("popstate", listener);
      };
    },
  };
}
