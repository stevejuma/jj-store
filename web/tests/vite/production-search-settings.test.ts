import { searchAdvancedDefaults } from "../../src/lib/controller/search-advanced.js";
import { test, expect, vi } from "vitest";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import { createProductionSearchSettings } from "../../src/lib/controller/production-search-settings.js";
import {
  searchConfiguration,
  searchDraft,
  HttpSearchSettings,
  type SearchConfiguration,
  type SearchSettings,
} from "../../src/lib/controller/search-settings.js";
import { ApiError, Client } from "../../src/lib/api/client.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
const configuration: SearchConfiguration = {
  ...searchAdvancedDefaults,
  enabled: true,
  index_source: false,
  scopes: { main: { kind: "bookmark", name: "main" } },
  include_prefixes: [],
  exclude_prefixes: [],
  index_bytes: 536870912,
};
function fixture() {
  const data = createProductionStoreState();
  data.capabilities = { local_search_management: true } as NonNullable<
    typeof data.capabilities
  >;
  const value = {
    version: 1,
    revision: "a",
    desired: configuration,
    effective: configuration,
    operation: "idle" as const,
    error: null,
  };
  const status = vi.fn(async () => structuredClone(value));
  const save = vi.fn(async () => structuredClone(value));
  const control = vi.fn(async () => structuredClone(value));
  const source = {
    searchSettings: { status, save, control },
  } as unknown as StoreDataSource;
  let current = true;
  const settings = createProductionSearchSettings({
    data,
    source: () => source,
    version: () => 1,
    current: () => current,
  });
  return {
    data,
    value,
    status,
    save,
    control,
    settings,
    disconnect() {
      current = false;
      settings.clear();
    },
  };
}
test("search configuration keeps explicit scope identities and validates duplicates and budgets", () => {
  const draft = searchDraft(configuration);
  expect(searchConfiguration(draft)).toEqual(configuration);
  expect(
    Object.hasOwn(
      searchConfiguration({ ...draft, bookmarks: "__proto__=main" }).scopes,
      "__proto__",
    ),
  ).toBe(true);
  expect(() =>
    searchConfiguration({ ...draft, workspaces: "main=default" }),
  ).toThrow(/unique/);
  expect(() => searchConfiguration({ ...draft, budget: "2049" })).toThrow(
    /2048/,
  );
});
test("save preserves draft and its revision through stale failure and refresh", async () => {
  const f = fixture();
  await f.settings.refreshSearchSettings();
  f.settings.setSearchDraft({ exclude: "private" });
  f.save.mockRejectedValueOnce(new ApiError("search_settings_changed"));
  await f.settings.saveSearchSettings();
  expect(f.data.searchSettings.draft?.exclude).toBe("private");
  await f.settings.saveSearchSettings();
  expect(f.save).toHaveBeenCalledOnce();
  f.status.mockResolvedValueOnce({ ...f.value, revision: "b" });
  await f.settings.refreshSearchSettings();
  expect(f.data.searchSettings.draft?.exclude).toBe("private");
  await f.settings.saveSearchSettings();
  expect(f.save).toHaveBeenLastCalledWith("a", {
    ...configuration,
    exclude_prefixes: ["private"],
  });
});
test("pending edits block apply and late replies cannot affect another repository", async () => {
  const f = fixture();
  await f.settings.refreshSearchSettings();
  f.settings.setSearchDraft({ enabled: false });
  await f.settings.controlSearch("apply");
  expect(f.control).not.toHaveBeenCalled();
  f.settings.cancelSearchDraft();
  let deliver!: (v: NonNullable<SearchSettings["value"]>) => void;
  f.control.mockImplementationOnce(
    () =>
      new Promise((resolve) => {
        deliver = resolve as typeof deliver;
      }),
  );
  const pending = f.settings.controlSearch("apply");
  await f.settings.controlSearch("apply");
  expect(f.control).toHaveBeenCalledOnce();
  f.disconnect();
  deliver(f.value);
  await pending;
  expect(f.data.searchSettings.value).toBeNull();
});

test("a successful status read cannot resolve a failed save with a retained draft", async () => {
  const f = fixture();
  await f.settings.refreshSearchSettings();
  f.settings.setSearchDraft({ exclude: "private" });
  f.save.mockRejectedValueOnce(new ApiError("search_settings_changed"));
  await f.settings.saveSearchSettings();
  const failure = f.data.searchSettings.error;
  expect(failure).toMatch(/search settings changed/);
  f.status.mockResolvedValueOnce({ ...f.value, revision: "b" });
  await f.settings.refreshSearchSettings();
  expect(f.data.searchSettings.error).toBe(failure);
  expect(f.data.searchSettings.draft?.exclude).toBe("private");
  expect(f.data.searchSettings.state).toBe("ready");
  expect(f.save).toHaveBeenCalledOnce();
  f.settings.cancelSearchDraft();
  expect(f.data.searchSettings.error).toBe("");
  expect(f.data.searchSettings.draft?.exclude).toBe("");
});

test("a successful explicit save resolves its retained failure", async () => {
  const f = fixture();
  await f.settings.refreshSearchSettings();
  f.settings.setSearchDraft({ exclude: "private" });
  f.save.mockRejectedValueOnce(new ApiError("search_settings_changed"));
  await f.settings.saveSearchSettings();
  await f.settings.refreshSearchSettings();
  expect(f.data.searchSettings.error).toMatch(/search settings changed/);
  await f.settings.saveSearchSettings();
  expect(f.data.searchSettings.error).toBe("");
  expect(f.data.searchSettings.dirty).toBe(false);
  expect(f.save).toHaveBeenCalledTimes(2);
});

test("automatic status reads retain the revision that started a dirty search draft", async () => {
  const f = fixture();
  await f.settings.refreshSearchSettings();
  f.settings.setSearchDraft({ exclude: "private" });
  f.status.mockResolvedValueOnce({ ...f.value, revision: "b" });
  await f.settings.refreshSearchSettings();
  await f.settings.saveSearchSettings();
  expect(f.save).toHaveBeenLastCalledWith("a", {
    ...configuration,
    exclude_prefixes: ["private"],
  });
});

test("cancelling a search draft adopts the refreshed configuration revision", async () => {
  const f = fixture();
  await f.settings.refreshSearchSettings();
  f.settings.setSearchDraft({ exclude: "private" });
  f.status.mockResolvedValueOnce({ ...f.value, revision: "b" });
  await f.settings.refreshSearchSettings();
  f.settings.cancelSearchDraft();
  f.settings.setSearchDraft({ exclude: "updated" });
  await f.settings.saveSearchSettings();
  expect(f.save).toHaveBeenLastCalledWith("b", {
    ...configuration,
    exclude_prefixes: ["updated"],
  });
});
test("save and apply use independent revision checked protected requests", async () => {
  const fetcher = vi.fn<typeof fetch>(
    async () =>
      new Response("{}", { headers: { "content-type": "application/json" } }),
  );
  const source = new HttpSearchSettings(
    new Client("ab".repeat(16), null, () => {}, fetcher),
  );
  await source.save("revision", configuration);
  await source.control("revision", "apply");
  expect(fetcher.mock.calls[0]?.[0]).toContain("/local/search/save");
  expect(fetcher.mock.calls[1]?.[0]).toContain("/local/search/control");
  expect(JSON.parse(fetcher.mock.calls[1]![1]!.body as string)).toEqual({
    version: 1,
    expected_revision: "revision",
    action: "apply",
  });
});

test("invalid search drafts stay inline across refresh and clear on cancellation", async () => {
  const f = fixture();
  await f.settings.refreshSearchSettings();
  f.settings.setSearchDraft({ budget: "2049" });
  await f.settings.saveSearchSettings();
  expect(f.save).not.toHaveBeenCalled();
  expect(f.data.searchSettings.error).toBe("");
  expect(f.data.searchSettings.validation).toMatch(/2048/);
  await f.settings.refreshSearchSettings();
  expect(f.data.searchSettings.validation).toMatch(/2048/);
  expect(f.data.searchSettings.draft?.budget).toBe("2049");
  f.settings.cancelSearchDraft();
  expect(f.data.searchSettings.validation).toBe("");
});

test("advanced Search limits roundtrip and reject invalid worker bounds", () => {
  const draft = searchDraft({
    ...configuration,
    poll_millis: 25,
    timeout_seconds: 45,
    snapshot_every: 0,
    max_text_bytes: 1000,
    max_files: 50,
    max_work: 10000,
    max_output_bytes: 1048576,
  });
  expect(searchConfiguration(draft)).toMatchObject({
    poll_millis: 25,
    timeout_seconds: 45,
    snapshot_every: 0,
    max_text_bytes: 1000,
    max_files: 50,
    max_work: 10000,
    max_output_bytes: 1048576,
  });
  for (const patch of [
    { poll_millis: "0" },
    { timeout_seconds: "301" },
    { snapshot_every: "-1" },
    { max_text_bytes: "1048577" },
    { max_files: "10001" },
    { max_work: "1000001" },
    { max_output_bytes: "0" },
  ])
    expect(() => searchConfiguration({ ...draft, ...patch })).toThrow(
      /whole number/,
    );
});
