import { expect, test, vi } from "vitest";
import { ApiError, type Model } from "../../src/lib/api/client.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
import { createProductionStoreSettings } from "../../src/lib/controller/production-store-index.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";

function fixture() {
  const data = createProductionStoreState();
  data.capabilities = {
    local_workspace_links: true,
    search: false,
  } as Model<"Capabilities">;
  const links = {
    version: 1,
    revision: 4n,
    links: [],
  } as Model<"WorkspaceLinks">;
  const list = vi.fn(async () => links);
  const link = vi.fn(async (): Promise<Model<"WorkspaceLinks">> => ({
    ...links,
    revision: 5n,
  }));
  const source = {
    localWorkspaces: { list, link },
  } as unknown as StoreDataSource;
  let current = true;
  const settings = createProductionStoreSettings({
    data,
    source: () => source,
    operation: () => null,
    version: () => 1,
    current: () => current,
    restore: async () => {},
  });
  return {
    data,
    list,
    link,
    settings,
    disconnect: () => {
      current = false;
      settings.clear();
    },
  };
}

test("local settings load and link physical workspaces without text search", async () => {
  const { data, list, link, settings } = fixture();
  await settings.refresh();
  expect(list).toHaveBeenCalledOnce();
  settings.setWorkspacePath("/my workspace");
  await settings.linkWorkspace();
  expect(link).toHaveBeenCalledWith({
    version: 1,
    expected_revision: 4n,
    path: "/my workspace",
  });
  expect(data.workspaceLinks?.revision).toBe(5n);
  expect(data.workspacePath).toBe("");
  expect(data.searchStatusState).toBe("unavailable");
});

test("failed linking retains the draft and explicit refresh recovers revision", async () => {
  const { data, list, link, settings } = fixture();
  await settings.refresh();
  settings.setWorkspacePath("/workspace");
  link.mockRejectedValueOnce(new ApiError("workspace_links_changed"));
  await settings.linkWorkspace();
  expect(data.workspacePath).toBe("/workspace");
  expect(data.workspaceLinksError).toContain("workspace links changed");
  expect(data.workspaceLinks?.revision).toBe(4n);
  list.mockResolvedValueOnce({ version: 1, revision: 6n, links: [] });
  await settings.refreshWorkspaceLinks();
  await settings.linkWorkspace();
  expect(link).toHaveBeenLastCalledWith({
    version: 1,
    expected_revision: 6n,
    path: "/workspace",
  });
});

test("workspace links ignore late responses after disconnect", async () => {
  const f = fixture();
  let deliver!: (value: Model<"WorkspaceLinks">) => void;
  f.list.mockImplementationOnce(
    () =>
      new Promise((resolve) => {
        deliver = resolve;
      }),
  );
  const pending = f.settings.refreshWorkspaceLinks();
  f.disconnect();
  deliver({ version: 1, revision: 7, links: [] });
  await pending;
  expect(f.data.workspaceLinks).toBeNull();
  expect(f.data.workspaceLinksState).toBe("idle");
});

test("unsupported capability never reads or links device folders", async () => {
  const { data, list, link, settings } = fixture();
  data.capabilities = { search: false } as Model<"Capabilities">;
  await settings.refresh();
  settings.setWorkspacePath("/workspace");
  await settings.linkWorkspace();
  expect(list).not.toHaveBeenCalled();
  expect(link).not.toHaveBeenCalled();
  expect(data.workspaceLinksState).toBe("unavailable");
});

test("link submission is single flight and ignores completion after repository switch", async () => {
  const f = fixture();
  await f.settings.refreshWorkspaceLinks();
  let deliver!: (value: Model<"WorkspaceLinks">) => void;
  f.link.mockImplementationOnce(
    () =>
      new Promise((resolve) => {
        deliver = resolve;
      }),
  );
  f.settings.setWorkspacePath("/workspace");
  const pending = f.settings.linkWorkspace();
  await f.settings.linkWorkspace();
  f.settings.setWorkspacePath("/changed while submitting");
  expect(f.data.workspacePath).toBe("/workspace");
  expect(f.link).toHaveBeenCalledOnce();
  f.disconnect();
  deliver({ version: 1, revision: 9, links: [] });
  await pending;
  expect(f.data.workspaceLinks).toBeNull();
  expect(f.data.workspacePath).toBe("");
  expect(f.data.workspaceLinking).toBe(false);
});

test("late folder refresh cannot replace the result of linking", async () => {
  const f = fixture();
  await f.settings.refreshWorkspaceLinks();
  let deliver!: (value: Model<"WorkspaceLinks">) => void;
  f.list.mockImplementationOnce(
    () =>
      new Promise((resolve) => {
        deliver = resolve;
      }),
  );
  const pending = f.settings.refreshWorkspaceLinks();
  f.settings.setWorkspacePath("/workspace");
  await f.settings.linkWorkspace();
  deliver({ version: 1, revision: 4, links: [] });
  await pending;
  expect(f.data.workspaceLinks?.revision).toBe(5n);
  expect(f.data.workspaceLinksState).toBe("ready");
});
