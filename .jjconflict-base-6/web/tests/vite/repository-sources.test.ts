import { expect, test } from "vitest";
import { RepositorySources } from "../../src/lib/controller/repository-sources.js";
import type { StoreRepositorySummary } from "../../src/lib/controller/contracts.js";
const repository = {
  id: "11".repeat(16),
  name: "shared",
  profile: "native",
  permission: "read",
} as StoreRepositorySummary;
test("view leases reuse one repository transport and retain another view after closure", async () => {
  let reads = 0;
  const sources = new RepositorySources(async () => {
    reads++;
    return new Response('{"version":1}', {
      headers: { "content-type": "application/json" },
    });
  });
  const a = sources.acquire(repository, "token"),
    b = sources.acquire(repository, "token");
  await Promise.all([a.capabilities(), b.capabilities()]);
  a.close();
  await b.capabilities();
  expect(reads).toBe(1);
  b.close();
  sources.clear();
});
test("credential replacement clears protected cache scopes", async () => {
  let reads = 0;
  const sources = new RepositorySources(async () => {
    reads++;
    return new Response('{"version":1}', {
      headers: { "content-type": "application/json" },
    });
  });
  const a = sources.acquire(repository, "first");
  await a.capabilities();
  const b = sources.acquire(repository, "second");
  await b.capabilities();
  expect(reads).toBe(2);
  await expect(a.capabilities()).rejects.toBeDefined();
  b.close();
  sources.clear();
});
