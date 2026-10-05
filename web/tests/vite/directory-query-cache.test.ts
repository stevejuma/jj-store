import { expect, test, vi } from "vitest";
import { HttpLocalStoreDataSource } from "../../src/lib/controller/http-local-store-data-source.js";
import { createStoreQueryClient } from "../../src/lib/controller/store-query-client.js";
import { createHttpQuery } from "../../src/lib/controller/http-query.js";

test("local directory reads reuse successful responses within their generation and expire cursors", async () => {
  const repository = "11".repeat(16),
    operation = "22".repeat(64),
    commit = "33".repeat(20);
  const queries = createStoreQueryClient();
  let requests = 0,
    generation = 1;
  const source = new HttpLocalStoreDataSource(
    repository,
    async (url, init) => {
      if (String(url).endsWith("snapshot"))
        return Response.json({
          version: 1,
          profile: "git_sha1_jj045",
          repository,
          generation,
          heads: [operation],
          metadata_commit: null,
          actor: "44".repeat(16),
        });
      requests++;
      const body = JSON.parse(String(init?.body));
      return Response.json({
        snapshot: {
          version: 1,
          profile: "git_sha1_jj045",
          repository,
          generation,
          operation: body.operation,
        },
        data: {
          kind: "paths",
          data: {
            version: 1,
            commit,
            path: body.request.query.path,
            recursive: false,
            entries: [],
            previous: null,
            next: "b",
          },
        },
        next_cursor: "cursor",
        previous_cursor: null,
      });
    },
    queries,
    "session",
  );
  try {
    const read = (path = "", options = {}) =>
      source.paths(commit, operation, {
        path,
        includeLastChange: false,
        ...options,
      });
    const first = await read();
    expect(await read()).toEqual(first);
    expect(requests).toBe(1);
    await read("arch");
    await read("", { cursor: "cursor" });
    expect(requests).toBe(3);
    generation++;
    await source.snapshot(true);
    await read();
    expect(requests).toBe(4);
    const now = Date.now();
    const time = vi.spyOn(Date, "now").mockReturnValue(now + 30_001);
    try {
      await read();
    } finally {
      time.mockRestore();
    }
    expect(requests).toBe(5);
    await read("", { includeLastChange: true, lastChangeMode: "history" });
    await read("", { includeLastChange: true, lastChangeMode: "history" });
    expect(requests).toBe(7);
    source.close();
    await expect(read()).rejects.toMatchObject({ name: "AbortError" });
    expect(requests).toBe(7);
  } finally {
    source.close();
    queries.clear();
  }
});

test("native pending history batches are not replayed from the 30-second page cache", async () => {
  const client = createStoreQueryClient();
  const query = createHttpQuery(
    client,
    { session: "session", repository: "repo", epoch: "epoch" },
    () => true,
  );
  let reads = 0;
  const read = async () => ++reads;
  try {
    await query("paths", "operation", { include_last_change: false }, read);
    await query("paths", "operation", { include_last_change: false }, read);
    expect(reads).toBe(1);
    await query("paths", "operation", { last_change_mode: "history" }, read);
    await query("paths", "operation", { last_change_mode: "history" }, read);
    expect(reads).toBe(3);
  } finally {
    client.clear();
  }
});
