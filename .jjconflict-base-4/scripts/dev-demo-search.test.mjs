import test from "node:test";
import assert from "node:assert/strict";
import { waitForSearch } from "./dev-demo-search.mjs";
test("development restart accepts explicitly disabled search without waiting", async () => {
  const paths = [];
  const state = await waitForSearch(
    "http://127.0.0.1:7460/",
    "repo",
    0,
    async (path) => {
      paths.push(path);
      return Response.json({ search: false });
    },
  );
  assert.deepEqual(state, { configured: false });
  assert.equal(paths.length, 1);
});
test("development restart still requires a fresh ready configured index", async () => {
  const paths = [];
  const state = await waitForSearch(
    "http://127.0.0.1:7460/",
    "repo",
    5,
    async (path, options) => {
      paths.push(path);
      assert.equal(options.headers["X-JJ-Store-Protocol"], "1");
      return Response.json(
        paths.length === 1
          ? { search: true }
          : {
              index_generation: "generation",
              indexed_position: 5,
              is_stale: false,
              degraded: false,
            },
      );
    },
  );
  assert.equal(state.indexed_position, 5);
  assert.equal(paths.length, 2);
});
