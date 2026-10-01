import { test, expect, vi } from "vitest";
import { Client } from "../../src/lib/api/client.js";
import { HttpWorkspaceLinks } from "../../src/lib/controller/http-workspace-links.js";

test("workspace transport preserves revisions and uses protected local requests", async () => {
  const fetcher = vi.fn<typeof fetch>(
    async () =>
      new Response('{"version":1,"revision":9007199254740993,"links":[]}', {
        headers: { "content-type": "application/json" },
      }),
  );
  const client = new Client("ab".repeat(16), null, () => {}, fetcher);
  const transport = new HttpWorkspaceLinks(client);
  const result = await transport.list();
  await transport.link({
    version: 1,
    path: "/workspace",
    expected_revision: result.revision,
  });
  expect(fetcher.mock.calls[0]?.[0]).toBe(
    `/v1/repos/${"ab".repeat(16)}/local/workspaces`,
  );
  expect(fetcher.mock.calls[1]?.[0]).toBe(
    `/v1/repos/${"ab".repeat(16)}/local/workspaces/link`,
  );
  const init = fetcher.mock.calls[1]?.[1] as RequestInit;
  expect(init).toMatchObject({
    method: "POST",
    credentials: "omit",
    redirect: "error",
    cache: "no-store",
    headers: { "x-jj-store-protocol": "1", "content-type": "application/json" },
  });
  expect(init.headers).not.toHaveProperty("authorization");
  expect(init.body).toContain('"expected_revision": 9007199254740993');
  client.close();
  await expect(transport.list()).rejects.toMatchObject({ name: "AbortError" });
  expect(fetcher).toHaveBeenCalledTimes(2);
});
