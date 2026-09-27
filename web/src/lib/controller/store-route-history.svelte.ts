import type { StorePage } from "../ui/community-navigation/model.js";
import type { StoreRoute } from "./contracts.js";
import {
  storeRouteFromHash,
  storeRouteToHash,
  type StoreRouteHashResult,
} from "./store-route-codec.js";

export interface StoreRouteBrowser {
  hash(): string;
  push(hash: string): void;
  replace(hash: string): void;
  go(delta: -1 | 1): void;
  listen(listener: () => void): () => void;
}

export const initialStoreRoute = (): StoreRoute => ({
  page: "branches",
  repository: "",
  bookmark: "",
  snapshotOperationId: null,
  inspectedOperationId: null,
  commitId: null,
  changeId: null,
  comparisonParentId: null,
  comparisonGroupId: null,
  path: null,
  directory: "",
});

function browserNavigation(): StoreRouteBrowser | undefined {
  if (typeof window === "undefined") return undefined;
  return {
    hash: () => window.location.hash,
    push: (hash) => window.history.pushState(null, "", hash),
    replace: (hash) => window.history.replaceState(null, "", hash),
    go: (delta) => window.history.go(delta),
    listen(listener) {
      window.addEventListener("hashchange", listener);
      window.addEventListener("popstate", listener);
      return () => {
        window.removeEventListener("hashchange", listener);
        window.removeEventListener("popstate", listener);
      };
    },
  };
}

const same = (left: StoreRoute, right: StoreRoute) =>
  JSON.stringify(left) === JSON.stringify(right);

export function createStoreRouteHistory(
  browser: StoreRouteBrowser | undefined = browserNavigation(),
) {
  const parsed = storeRouteFromHash(browser?.hash() ?? "");
  let entries = $state<StoreRoute[]>([
    parsed.kind === "route" ? parsed.route : initialStoreRoute(),
  ]);
  let index = $state(0);
  let error = $state(parsed.kind === "invalid" ? parsed.message : "");
  let external: ((result: StoreRouteHashResult) => void) | undefined;
  const route = $derived(entries[index]!);
  function sync(next: StoreRoute, mode: "push" | "replace") {
    const hash = storeRouteToHash(next);
    if (!hash || !browser || browser.hash() === hash) return;
    browser[mode](hash);
  }
  function push(next: StoreRoute) {
    if (same(next, route)) return;
    entries = [...entries.slice(0, index + 1), next];
    index += 1;
    error = "";
    sync(next, "push");
  }
  const stop = browser?.listen(() => {
    const result = storeRouteFromHash(browser.hash());
    if (result.kind === "none") return;
    if (result.kind === "invalid") {
      error = result.message;
      external?.(result);
      return;
    }
    error = "";
    if (same(result.route, route)) return;
    const known = entries.findIndex((entry) => same(entry, result.route));
    if (known >= 0) index = known;
    else {
      entries = [result.route];
      index = 0;
    }
    external?.(result);
  });
  return {
    get route() {
      return route;
    },
    get canBack() {
      return index > 0;
    },
    get canForward() {
      return index < entries.length - 1;
    },
    get error() {
      return error;
    },
    push,
    replace(next: StoreRoute) {
      entries[index] = next;
      error = "";
      sync(next, "replace");
    },
    connect(repository: string, operation: string | null) {
      const next =
        route.repository === repository
          ? {
              ...route,
              snapshotOperationId: route.snapshotOperationId ?? operation,
            }
          : {
              ...initialStoreRoute(),
              repository,
              snapshotOperationId: operation,
            };
      entries = [next];
      index = 0;
      error = "";
      sync(next, "replace");
      return next;
    },
    refresh(operation: string | null) {
      const next = { ...route, snapshotOperationId: operation };
      entries[index] = next;
      sync(next, "replace");
      return next;
    },
    selectSnapshot(operation: string) {
      push({
        ...route,
        snapshotOperationId: operation,
        bookmark: "",
        inspectedOperationId: null,
        commitId: null,
        changeId: null,
        comparisonParentId: null,
        comparisonGroupId: null,
        path: null,
        directory: "",
      });
    },
    page(page: StorePage) {
      push({
        ...route,
        page,
        bookmark: "",
        inspectedOperationId: null,
        commitId: null,
        changeId: null,
        comparisonParentId: null,
        comparisonGroupId: null,
        path: null,
        directory: "",
      });
    },
    navigate(delta: -1 | 1) {
      const next = Math.max(0, Math.min(entries.length - 1, index + delta));
      if (next === index) return false;
      if (browser) {
        browser.go(delta);
        return false;
      }
      index = next;
      return true;
    },
    onExternalRoute(handler: (result: StoreRouteHashResult) => void) {
      external = handler;
    },
    destroy() {
      stop?.();
      external = undefined;
    },
  };
}
