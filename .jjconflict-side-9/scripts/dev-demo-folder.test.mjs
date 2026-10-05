import assert from "node:assert/strict";
import test from "node:test";
import { linkDemoFolder } from "./dev-demo-folder.mjs";
test("demo links only its native folder with current revision and no credential", async () => {
  const calls = [];
  let links = [];
  const options = {
    origin: "http://127.0.0.1:7460/",
    repositoryId: "11".repeat(16),
    path: "/owned/demo",
    fetcher: async (url, init) => {
      calls.push(url);
      assert.equal(init.headers.Authorization, undefined);
      assert.equal(init.headers["X-JJ-Store-Protocol"], "1");
      if (init.method === "POST") {
        assert.deepEqual(JSON.parse(init.body), {
          version: 1,
          expected_revision: 7,
          path: "/owned/demo",
        });
        links = [{ path: "/owned/demo" }];
      }
      return Response.json({ version: 1, revision: 7, links });
    },
  };
  await linkDemoFolder(options);
  await linkDemoFolder(options);
  assert.equal(calls.length, 3);
  assert.equal(calls.filter((url) => url.endsWith("/link")).length, 1);
});
