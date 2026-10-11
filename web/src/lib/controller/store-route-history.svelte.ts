import { untrack } from "svelte";
import {
  createGlobalSettingsNavigation,
  withoutGlobalSettings,
} from "./global-settings-navigation.js";
import {
  browserNavigation,
  type StoreRouteBrowser,
} from "./store-route-browser.js";
export type { StoreRouteBrowser } from "./store-route-browser.js";
import { createStoreNavigationGuards } from "./store-navigation-guards.js";
import { createRepositorySettingsNavigation } from "./repository-settings-navigation.js";
import type { StorePage } from "../ui/community-navigation/model.js";
import type { StoreRoute } from "./contracts.js";
import {
  createStoreStacksNavigation,
  storePageRoute,
} from "./store-stacks-navigation.js";
import {
  storeRouteFromHash,
  storeRouteToHash,
  type StoreRouteHashResult,
} from "./store-route-codec.js";

export const initialStoreRoute = (): StoreRoute => ({
  page: "branches",
  repository: "",
  bookmark: "",
  workspace: null,
  snapshotOperationId: null,
  inspectedOperationId: null,
  commitId: null,
  evolutionCommitId: null,
  changeId: null,
  comparisonParentId: null,
  comparisonGroupId: null,
  path: null,
  directory: "",
  agentSessionId: null,
  reviewThreadId: null,
});

function same(left: StoreRoute, right: StoreRoute) {
  const leftHash = storeRouteToHash(left),
    rightHash = storeRouteToHash(right);
  if (left.globalSettings && right.globalSettings && leftHash === rightHash)
    return same(withoutGlobalSettings(left), withoutGlobalSettings(right));
  return leftHash && rightHash
    ? leftHash === rightHash
    : JSON.stringify(left) === JSON.stringify(right);
}

export function createStoreRouteHistory(
  browser: StoreRouteBrowser | undefined = browserNavigation(),
) {
  const parsed = storeRouteFromHash(browser?.hash() ?? "");
  let entries = $state<StoreRoute[]>([
    parsed.kind === "route" ? parsed.route : initialStoreRoute(),
  ]);
  const newEntryId = () => crypto.randomUUID();
  let entryIds = [browser?.entryId?.() ?? newEntryId()];
  let index = $state(0);
  let error = $state(parsed.kind === "invalid" ? parsed.message : "");
  const guards = createStoreNavigationGuards(() => route);
  function blocked(next: StoreRoute) {
    const reason = guards.reason(next);
    if (reason) {
      error = reason;
      return true;
    }
    return false;
  }
  let external: ((result: StoreRouteHashResult) => void) | undefined;
  const route = $derived(entries[index]!);
  function sync(next: StoreRoute, mode: "push" | "replace") {
    const hash = storeRouteToHash(next);
    if (!hash || !browser) return;
    if (
      browser.hash() === hash &&
      (!browser.entryId || browser.entryId() === entryIds[index])
    )
      return;
    browser[mode](hash, entryIds[index]);
  }
  function push(next: StoreRoute) {
    if (same(next, route) || blocked(next)) return;
    entries = [...entries.slice(0, index + 1), next];
    entryIds = [...entryIds.slice(0, index + 1), newEntryId()];
    index += 1;
    error = "";
    sync(next, "push");
  }
  const repositorySettings = createRepositorySettingsNavigation({
    route: () => route,
    push,
    replace(next) {
      entries[index] = next;
      error = "";
      sync(next, "replace");
    },
  });
  const globalSettings = createGlobalSettingsNavigation({
    route: () => route,
    push,
    replace(next) {
      if (blocked(next)) return;
      entries[index] = next;
      error = "";
      sync(next, "replace");
    },
  });
  const stacks = createStoreStacksNavigation({ route: () => route, push });
  untrack(() => {
    if (browser?.entryId && storeRouteToHash(route))
      browser.replace(
        route.globalSettings ? storeRouteToHash(route)! : browser.hash(),
        entryIds[index],
      );
  });
  const stop = browser?.listen(() => {
    const result = storeRouteFromHash(browser.hash());
    if (result.kind === "none") return;
    if (result.kind === "invalid") {
      error = result.message;
      external?.(result);
      return;
    }
    const browserId = browser.entryId?.();
    const candidate = browserId ? entryIds.indexOf(browserId) : -1;
    const identified =
      candidate >= 0 &&
      storeRouteToHash(entries[candidate]!) === storeRouteToHash(result.route)
        ? candidate
        : -1;
    if (identified >= 0) result.route = entries[identified]!;
    else if (result.route.globalSettings)
      result.route = { ...route, globalSettings: result.route.globalSettings };
    if (!same(result.route, route) && blocked(result.route)) {
      sync(route, "replace");
      return;
    }
    error = "";
    if (same(result.route, route) && (identified < 0 || identified === index))
      return;
    const known =
      identified >= 0
        ? identified
        : browser.entryId
          ? -1
          : entries.findIndex((entry) => same(entry, result.route));
    if (known >= 0) index = known;
    else {
      entries = [...entries.slice(0, index + 1), result.route];
      entryIds = [...entryIds.slice(0, index + 1), browserId ?? newEntryId()];
      index += 1;
      if (browser.entryId) sync(result.route, "replace");
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
    canNavigateTo(next: StoreRoute) {
      return !blocked(next);
    },
    setNavigationGuard: guards.setNavigationGuard,
    stacks,
    repositorySettings,
    globalSettings,
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
      if (
        route.agentsHub ||
        (route.page === "projects" && route.projectsHub) ||
        (route.repository && route.repository !== repository)
      )
        push({
          ...next,
          page: "files",
          projectsHub: false,
          projectId: null,
          taskId: null,
        });
      else {
        entries[index] = next;
        sync(next, "replace");
      }
      error = "";
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
        ...(route.context === "stacks"
          ? { stackHistory: "pinned" as const }
          : {}),
        snapshotOperationId: operation,
        fileDisplay: undefined,
        bookmark: "",
        inspectedOperationId: null,
        commitId: null,
        evolutionCommitId: null,
        changeId: null,
        comparisonParentId: null,
        comparisonGroupId: null,
        path: null,
        directory: "",
        agentSessionId: null,
        reviewThreadId: null,
        projectId: null,
        taskId: null,
        threadId: null,
        replyId: null,
      });
    },
    page(page: StorePage) {
      if (page === "stacks") stacks.enter();
      else push(storePageRoute(route, page));
    },
    navigate(delta: -1 | 1) {
      const next = Math.max(0, Math.min(entries.length - 1, index + delta));
      if (next === index || blocked(entries[next]!)) return false;
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
