import { expect, test } from "vitest";
import { initialStoreRoute } from "../../src/lib/controller/store-route-history.svelte.js";
import type { StoreRoute } from "../../src/lib/controller/store-route-types.js";
import { sameRepositorySettingsOwner } from "../../src/lib/ui/store-settings/repository-settings-guards.js";

const retained: StoreRoute = {
  ...initialStoreRoute(),
  page: "settings",
  repository: "ab".repeat(16),
  workspace: "laptop",
  bookmark: "feature",
  snapshotOperationId: "cd".repeat(64),
  commitId: "ef".repeat(64),
  path: "src/main.rs",
  repositorySettingsSection: "sync",
  repositorySettingsTab: "overview",
};

test("repository categories retain the complete pending-action owner", () => {
  expect(
    sameRepositorySettingsOwner(retained, {
      ...retained,
      repositorySettingsSection: "general",
      repositorySettingsTab: undefined,
    }),
  ).toBe(true);
  expect(
    sameRepositorySettingsOwner(retained, {
      ...retained,
      repositorySettingsTab: "progress",
    }),
  ).toBe(true);
});

const ownerChanges: { label: string; change: Partial<StoreRoute> }[] = [
  { label: "repository", change: { repository: "12".repeat(16) } },
  { label: "page", change: { page: "files" } },
  { label: "operation", change: { snapshotOperationId: "34".repeat(64) } },
  { label: "workspace", change: { workspace: "desktop" } },
  { label: "bookmark", change: { bookmark: "main" } },
  { label: "commit", change: { commitId: "56".repeat(64) } },
  { label: "file", change: { path: "src/other.rs" } },
  { label: "directory", change: { directory: "src" } },
  { label: "query", change: { historyQuery: "other" } },
  { label: "Projects", change: { projectsHub: true } },
  { label: "Agents", change: { agentsHub: true } },
  { label: "Threads", change: { threadsHub: true } },
  {
    label: "Global Settings",
    change: { globalSettings: { section: "service", tab: "overview" } },
  },
];

test.each(ownerChanges)("$label changes stay guarded", ({ change }) => {
  expect(
    sameRepositorySettingsOwner(retained, { ...retained, ...change }),
  ).toBe(false);
});

test("entering Settings and unbound or global owners do not bypass guards", () => {
  expect(
    sameRepositorySettingsOwner({ ...retained, page: "files" }, retained),
  ).toBe(false);
  const unbound = { ...retained, repository: "" };
  expect(sameRepositorySettingsOwner(unbound, unbound)).toBe(false);
  const global: StoreRoute = {
    ...retained,
    globalSettings: { section: "agents", tab: "capture" },
  };
  expect(sameRepositorySettingsOwner(global, global)).toBe(false);
});
