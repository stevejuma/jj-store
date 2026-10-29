import { expect, test, vi } from "vitest";
import { createProductionStoreActions } from "../../src/lib/controller/production-store-actions.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import { createStoreRouteHistory } from "../../src/lib/controller/store-route-history.svelte.js";

test("selecting a visible file pins its commit before reference restoration settles", async () => {
  const data = createProductionStoreState();
  const history = createStoreRouteHistory();
  const commit = "66".repeat(64);
  history.connect("11".repeat(16), "22".repeat(64));
  history.replace({ ...history.route, page: "files", bookmark: "main" });
  data.snapshot = { id: commit } as typeof data.snapshot;
  const loadBlob = vi.fn(async () => {});
  const actions = createProductionStoreActions({
    data,
    history,
    reader: { loadBlob },
    search: {},
    mutations: { actions: {} },
    conflicts: {},
    branches: {},
    settings: {},
    invalidate: vi.fn(),
  } as unknown as Parameters<typeof createProductionStoreActions>[0]);
  await actions.selectFile("bin");
  expect(history.route).toMatchObject({ commitId: commit, path: "bin" });
  expect(loadBlob).toHaveBeenCalledExactlyOnceWith("bin");
});
