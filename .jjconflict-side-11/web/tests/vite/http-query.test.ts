import { expect, test } from "vitest";
import { createHttpQuery } from "../../src/lib/controller/http-query.js";
import { createStoreQueryClient } from "../../src/lib/controller/store-query-client.js";
test("immutable content shares exact epoch while transient prepared pages expire", async () => {
  const client = createStoreQueryClient(),
    scope = { session: "session", repository: "repository", epoch: "first" };
  const query = createHttpQuery(client, scope, () => true);
  let reads = 0;
  const read = async () => ++reads;
  expect(await query("file", "operation", { id: "file" }, read)).toBe(1);
  expect(await query("file", "operation", { id: "file" }, read)).toBe(1);
  scope.epoch = "second";
  expect(await query("file", "operation", { id: "file" }, read)).toBe(2);
  await query("log", "operation", {}, read);
  const page = client
    .getQueryCache()
    .getAll()
    .find((q) => q.queryKey[4] === "log")!;
  expect(page.options.staleTime).toBe(30_000);
  client.clear();
});
