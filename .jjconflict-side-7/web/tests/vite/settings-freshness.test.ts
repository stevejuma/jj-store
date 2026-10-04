import { expect, test, vi } from "vitest";
import { observeStoreSettings } from "../../src/lib/controller/settings-refresh.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import type { StoreController } from "../../src/lib/controller/contracts.js";
import type { ApplicationFreshness } from "../../src/lib/controller/application-freshness.js";
test("storage events read storage without refetching installation configuration or other sections", async () => {
  const state = createProductionStoreState();
  const repository = "aa".repeat(16),
    observe = vi.fn(async () => {}),
    refreshSettings = vi.fn(async () => {});
  state.status.connection = "connected";
  let watcher!: Parameters<ApplicationFreshness["watch"]>[0];
  const controller = {
    state: {
      route: { repository },
      status: state.status,
      settings: {
        storageSettings: state.storageSettings,
        syncSettings: state.syncSettings,
        searchSettings: state.searchSettings,
        searchStatusState: state.searchStatusState,
        workspaceLinksState: state.workspaceLinksState,
      },
    },
    configuration: { observe, state: { error: "" } },
    actions: { refreshSettings },
    freshness: {
      healthy: true,
      watch(options: typeof watcher) {
        watcher = options;
        return { stop() {} };
      },
    },
  } as unknown as StoreController;
  const stop = observeStoreSettings(controller, "storage");
  await watcher.refresh();
  expect(observe).toHaveBeenCalledTimes(1);
  expect(refreshSettings).toHaveBeenCalledWith("storage");
  expect(
    watcher.matches({ repository, domain: "storage", witness: "two" }),
  ).toBe(true);
  await watcher.refresh();
  expect(observe).toHaveBeenCalledTimes(1);
  expect(refreshSettings).toHaveBeenCalledTimes(2);
  expect(watcher.matches({ repository, domain: "sync", witness: "two" })).toBe(
    false,
  );
  expect(
    watcher.matches({
      repository: "bb".repeat(16),
      domain: "storage",
      witness: "two",
    }),
  ).toBe(false);
  watcher.reset?.();
  await watcher.refresh();
  expect(observe).toHaveBeenCalledTimes(2);
  stop();
});
