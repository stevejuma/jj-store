import type { StoreRouteBrowser } from "../controller/store-route-browser.js";
import { connectionHash, readConnectionHash } from "./transport.js";
/** An inactive owner cannot interpret or overwrite another server's history. */
export function connectionBrowser(
  base: StoreRouteBrowser,
  id: string,
  active: () => boolean,
  initial: string,
  remember: (hash: string) => void,
): StoreRouteBrowser & { activate(): void } {
  let retained = initial;
  const listeners = new Set<() => void>();
  return {
    activate() {
      if (active()) for (const notify of listeners) notify();
    },
    hash: () => {
      if (!active()) return retained;
      const route = readConnectionHash(base.hash());
      return route.id === id ? route.hash : retained;
    },
    entryId: () => (active() ? base.entryId?.() : undefined),
    push(hash, entry) {
      retained = hash;
      remember(hash);
      if (active()) base.push(connectionHash(hash, id), entry);
    },
    replace(hash, entry) {
      retained = hash;
      remember(hash);
      if (active()) base.replace(connectionHash(hash, id), entry);
    },
    go(delta) {
      if (active()) base.go(delta);
    },
    listen(notify) {
      listeners.add(notify);
      const stop = base.listen(() => {
        if (!active()) return;
        const parsed = readConnectionHash(base.hash());
        if (parsed.id !== id) return;
        retained = parsed.hash;
        remember(retained);
        notify();
      });
      return () => {
        stop();
        listeners.delete(notify);
      };
    },
  };
}
