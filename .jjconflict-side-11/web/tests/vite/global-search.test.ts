import { expect, test, vi } from "vitest";
import { createGlobalSearch } from "../../src/lib/controller/global-search.svelte.js";
import { emptyRoute } from "../../src/lib/controller/global-search-presentation.js";
import type { StoreRepositorySummary } from "../../src/lib/controller/contracts.js";
const repo = (n: number) => n.toString(16).padStart(32, "0");
const sources: StoreRepositorySummary[] = [1, 2, 3].map((n) => ({
  id: repo(n),
  name: `Repository ${n}`,
  permission: "read",
  principal: "44".repeat(16),
}));
function page(
  repository: string,
  id = "11".repeat(16),
  next: string | null = null,
) {
  return {
    version: 1,
    snapshot: { repo_id: repository },
    results: [
      {
        kind: "message",
        id,
        title: "Planning",
        excerpt: "Current message",
        actor: "44".repeat(16),
        at_millis: 2000,
        revision: "55".repeat(32),
        thread: "22".repeat(16),
        message: id,
        project: null,
        task: null,
        comment: null,
      },
    ],
    next_cursor: next,
  };
}
const json = (data: unknown, status = 200) =>
  new Response(JSON.stringify(data), {
    status,
    headers: { "content-type": "application/json" },
  });
function setup(fetcher: typeof fetch, repositories = sources) {
  const open = vi.fn(async () => {});
  const search = createGlobalSearch({
    repositories: () => repositories,
    token: () => null,
    route: () => emptyRoute(repo(1)),
    open,
    fetcher,
  });
  return { search, open };
}
test("fans out over authorized sources with two readers and preserves exact native result routes", async () => {
  let active = 0,
    max = 0;
  const requests: URL[] = [];
  const f = setup(async (input) => {
    const url = new URL(String(input), "http://fixture");
    requests.push(url);
    active++;
    max = Math.max(max, active);
    await new Promise((resolve) => setTimeout(resolve, 5));
    active--;
    return json(
      url.pathname.endsWith("capabilities")
        ? { search: false }
        : page(url.pathname.split("/")[3]!),
    );
  });
  await f.search.run("needle");
  expect(max).toBe(2);
  expect(f.search.results).toHaveLength(3);
  expect(
    requests
      .filter((r) => r.pathname.endsWith("content/search"))
      .map((r) => r.pathname.split("/")[3]),
  ).toEqual(sources.map((r) => r.id));
  await f.search.select(f.search.results[1]!);
  expect(f.open).toHaveBeenCalledWith(
    expect.objectContaining({
      repository: repo(2),
      page: "threads",
      threadId: "22".repeat(16),
      replyId: "11".repeat(16),
    }),
  );
});
test("scopes and independent continuations retain every filter and avoid code calls", async () => {
  const requests: URL[] = [];
  const f = setup(async (input) => {
    const url = new URL(String(input), "http://fixture");
    requests.push(url);
    return json(
      page(
        repo(2),
        url.searchParams.has("cursor") ? "12".repeat(16) : undefined,
        url.searchParams.has("cursor") ? null : "opaque",
      ),
    );
  });
  const scope = {
    label: "Planning",
    repository: repo(2),
    section: "threads" as const,
    thread: "22".repeat(16),
  };
  await f.search.run("needle", scope);
  expect(f.search.canLoadMore).toBe(true);
  await f.search.more();
  expect(requests).toHaveLength(2);
  expect([...requests[1]!.searchParams.entries()]).toEqual(
    expect.arrayContaining([
      ["q", "needle"],
      ["section", "threads"],
      ["thread", scope.thread],
      ["limit", "50"],
      ["cursor", "opaque"],
    ]),
  );
  expect(f.search.canLoadMore).toBe(false);
  expect(f.search.results).toHaveLength(2);
});
test("clear and replacement discard late results and preserve explicit source errors", async () => {
  let release!: () => void;
  const f = setup(
    async (input) => {
      const url = new URL(String(input), "http://fixture");
      if (url.searchParams.get("q") === "old")
        await new Promise<void>((resolve) => (release = resolve));
      return json(page(repo(1), "11".repeat(16)));
    },
    [sources[0]!],
  );
  const old = f.search.run("old", { label: "Threads", section: "threads" });
  await f.search.run("new", { label: "Threads", section: "threads" });
  release();
  await old;
  expect(f.search.results).toHaveLength(1);
  f.search.clear();
  expect(f.search.results).toEqual([]);
  expect(f.search.canLoadMore).toBe(false);
  const denied = setup(
    async () => json({ code: "access_denied" }, 403),
    [sources[0]!],
  );
  await denied.search.run("");
  expect(denied.search.results).toEqual([]);
  expect(denied.search.errors[0]).toContain("Repository 1");
  expect(denied.search.status).toBe("failed");
});
test("ambiguous code heads produce a source error and do not select an arbitrary operation", async () => {
  const requests: URL[] = [];
  const f = setup(
    async (input) => {
      const url = new URL(String(input), "http://fixture");
      requests.push(url);
      return json(
        url.pathname.endsWith("capabilities")
          ? { search: true }
          : { heads: ["11".repeat(64), "22".repeat(64)] },
      );
    },
    [sources[0]!],
  );
  await f.search.run("code", { label: "Code", section: "code" });
  expect(requests.some((r) => r.pathname.endsWith("/search"))).toBe(false);
  expect(f.search.errors[0]).toContain("Choose a code operation");
});
test("revocation removes cached source results and continuations and stops code discovery", async () => {
  let revoked = false;
  const requests: URL[] = [];
  const f = setup(
    async (input) => {
      const url = new URL(String(input), "http://fixture");
      requests.push(url);
      if (revoked) return json({ code: "access_denied" }, 403);
      return json(page(repo(1), undefined, "next"));
    },
    [sources[0]!],
  );
  await f.search.run("", { label: "Threads", section: "threads" });
  expect(f.search.results).toHaveLength(1);
  expect(f.search.canLoadMore).toBe(true);
  revoked = true;
  requests.length = 0;
  await f.search.more();
  expect(f.search.results).toEqual([]);
  expect(f.search.canLoadMore).toBe(false);
  await f.search.run("needle");
  expect(requests.some((url) => url.pathname.endsWith("/capabilities"))).toBe(
    false,
  );
});
test("sources removed while a query is in flight cannot leave results or continuations", async () => {
  let authorized = [sources[0]!],
    release!: () => void;
  const search = createGlobalSearch({
    repositories: () => authorized,
    token: () => null,
    route: () => emptyRoute(repo(1)),
    open: async () => {},
    fetcher: async () => {
      await new Promise<void>((resolve) => (release = resolve));
      return json(page(repo(1), undefined, "next"));
    },
  });
  const pending = search.run("", { label: "Threads", section: "threads" });
  authorized = [];
  release();
  await pending;
  expect(search.results).toEqual([]);
  expect(search.canLoadMore).toBe(false);
});

test("global_search_does_not_issue_native_requests_for_local_profiles", async () => {
  const requests: URL[] = [];
  const f = setup(
    async (input) => {
      const url = new URL(String(input), "http://fixture");
      requests.push(url);
      return json(
        url.pathname.endsWith("capabilities")
          ? { search: false }
          : page(sources[1]!.id),
      );
    },
    [
      { ...sources[0]!, profile: "git_sha1_jj045" },
      { ...sources[1]!, profile: "native" },
    ],
  );
  await f.search.run("needle");
  expect(
    requests.every((url) => url.pathname.split("/")[3] === sources[1]!.id),
  ).toBe(true);
  expect(f.search.results).toHaveLength(1);
  expect(f.search.errors).toEqual([
    "Repository 1: Search is unavailable for local Git-backed repositories.",
  ]);
  expect(f.search.canLoadMore).toBe(false);
  await f.search.run("needle", {
    label: "Local repository",
    repository: sources[0]!.id,
    section: "all",
  });
  expect(f.search.results).toEqual([]);
  expect(
    requests.every((url) => url.pathname.split("/")[3] === sources[1]!.id),
  ).toBe(true);
});
