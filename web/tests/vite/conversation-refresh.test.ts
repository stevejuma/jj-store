import { test, expect } from "vitest";
import { QueryClient } from "@tanstack/svelte-query";
import { HttpCollaborationDataSource } from "../../src/lib/controller/http-collaboration-data-source.js";
import type { Client } from "../../src/lib/api/client.js";
import { storeQueryKeys } from "../../src/lib/controller/store-query-keys.js";
test("committed conversation refresh invalidates only the selected detail and retains pinned code reads", async () => {
  const scope = { session: "device", repository: "repo" },
    queries = new QueryClient({
      defaultOptions: { queries: { staleTime: Infinity, retry: false } },
    });
  const id = "a".repeat(32),
    other = "b".repeat(32),
    requests: string[] = [];
  const client = {
    request: async (path: string) => {
      requests.push(path);
      return { revision: requests.length };
    },
  } as unknown as Client;
  const source = new HttpCollaborationDataSource(
    client,
    queries,
    scope,
    () => true,
  );
  const pin = storeQueryKeys.resource(scope, "blob", "explicit-operation", {
    path: "file",
  });
  queries.setQueryData(pin, { content: "pinned" });
  const initial = await source.agentSession(id);
  await source.agentSession(other);
  expect(await source.agentSession(id)).toEqual(initial);
  expect(requests).toHaveLength(2);
  expect(await source.agentSession(id, true)).not.toEqual(initial);
  expect(requests).toHaveLength(3);
  await source.agentSession(other);
  expect(requests).toHaveLength(3);
  expect(queries.getQueryData(pin)).toEqual({ content: "pinned" });
  queries.clear();
});
