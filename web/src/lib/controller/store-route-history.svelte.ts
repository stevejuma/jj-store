import type { StorePage } from "../ui/community-navigation/model.js";
import type { StoreRoute } from "./contracts.js";

const initialRoute = (): StoreRoute => ({
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

export function createStoreRouteHistory() {
  let entries = $state<StoreRoute[]>([initialRoute()]);
  let index = $state(0);
  const route = $derived(entries[index]!);
  function push(next: StoreRoute) {
    if (JSON.stringify(next) === JSON.stringify(route)) return;
    entries = [...entries.slice(0, index + 1), next];
    index += 1;
  }
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
    push,
    replace(next: StoreRoute) {
      entries[index] = next;
    },
    connect(repository: string, operation: string | null) {
      entries = [
        { ...initialRoute(), repository, snapshotOperationId: operation },
      ];
      index = 0;
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
      index = Math.max(0, Math.min(entries.length - 1, index + delta));
      return route;
    },
  };
}
