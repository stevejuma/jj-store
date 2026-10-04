import assert from "node:assert/strict";
import { test } from "vitest";
import { HttpLocalStoreDataSource } from "../../src/lib/controller/http-local-store-data-source.js";
import { ApiError, pretty } from "../../src/lib/api/client.js";
const repository = "11".repeat(16),
  operation = "22".repeat(64),
  head = "33".repeat(20),
  base = "44".repeat(20);
const snapshot = {
  version: 1,
  profile: "git_sha1_jj045",
  repository,
  generation: 1,
  operation,
};
const json = (value: unknown) =>
  new Response(pretty(value), {
    headers: { "content-type": "application/json" },
  });
function fixture(change: (page: any) => void = () => {}) {
  const calls: any[] = [];
  const source = new HttpLocalStoreDataSource(repository, async (url, init) => {
    assert.equal(new Headers(init?.headers).has("authorization"), false);
    if (String(url).endsWith("snapshot"))
      return json({
        version: 1,
        profile: "git_sha1_jj045",
        repository,
        generation: 1,
        heads: [operation],
        metadata_commit: null,
        actor: null,
      });
    calls.push({ url: String(url), body: JSON.parse(String(init?.body)) });
    const page = {
      version: 1,
      operation,
      head,
      base,
      commits: [],
      next: null,
      stats: {},
    };
    change(page);
    return json({
      snapshot,
      data: { kind: "stack_history", data: page },
      next_cursor: null,
      previous_cursor: null,
    });
  });
  return { source, calls };
}
test("local_stack_transport_keeps_operation_range_and_opaque_cursor", async () => {
  const { source, calls } = fixture();
  try {
    await source.stackHistory({
      operation,
      head,
      base,
      limit: 2,
      cursor: "original-cursor",
    });
    assert.equal(calls.length, 1);
    assert.equal(calls[0].url, `/v1/local/repos/${repository}/query`);
    assert.deepEqual(calls[0].body, {
      operation,
      request: {
        kind: "stack_history",
        query: { head, base, limit: 2, after: null },
      },
      cursor: "original-cursor",
    });
  } finally {
    source.close();
  }
});
test("local_stack_transport_rejects_foreign_nested_range_and_oversized_pages", async () => {
  for (const change of [
    (page: any) => {
      page.operation = "55".repeat(64);
    },
    (page: any) => {
      page.head = "66".repeat(20);
    },
    (page: any) => {
      page.base = null;
    },
    (page: any) => {
      page.commits = [{}, {}, {}];
    },
  ]) {
    const { source } = fixture(change);
    try {
      await assert.rejects(
        source.stackHistory({ operation, head, base, limit: 2 }),
        (cause: unknown) =>
          cause instanceof ApiError &&
          ["snapshot_mismatch", "invalid_stack_page"].includes(cause.code),
      );
    } finally {
      source.close();
    }
  }
});
test("local_stack_transport_rejects_invalid_bounds_before_network", async () => {
  const { source, calls } = fixture();
  try {
    for (const limit of [0, -1, Infinity, 1.5, 501])
      await assert.rejects(
        source.stackHistory({ operation, head, base, limit }),
        (cause: unknown) =>
          cause instanceof ApiError && cause.code === "invalid_stack_page",
      );
    assert.equal(calls.length, 0);
  } finally {
    source.close();
  }
});
