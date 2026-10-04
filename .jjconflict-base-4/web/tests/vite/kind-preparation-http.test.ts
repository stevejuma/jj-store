import { expect, test } from "vitest";
import { HttpStoreDataSource } from "../../src/lib/controller/http-store-data-source.js";
import { createStoreQueryClient } from "../../src/lib/controller/store-query-client.js";
const repository = "ab".repeat(16),
  operation = "cd".repeat(64);
test("native kind transport retains the selected operation and bodyless exact cancellation", async () => {
  const requests: { url: string; init: RequestInit | undefined }[] = [];
  const source = new HttpStoreDataSource(
    repository,
    null,
    createStoreQueryClient(),
    "kind-session",
    () => {},
    async (input, init) => {
      requests.push({ url: String(input), init });
      return new Response(
        JSON.stringify({
          version: 1,
          id: "opaque/task",
          kind: "copies",
          status: "running",
        }),
        { status: 200, headers: { "content-type": "application/json" } },
      );
    },
  );
  const abort = new AbortController();
  await source.prepareKindIndex(operation, "copies", abort.signal);
  await source.kindIndexTask("opaque/task", abort.signal);
  await source.cancelKindIndexTask("opaque/task", abort.signal);
  expect(requests.map((r) => r.url)).toEqual([
    `/v1/repos/${repository}/index/kinds/tasks`,
    `/v1/repos/${repository}/index/kinds/tasks/opaque%2Ftask`,
    `/v1/repos/${repository}/index/kinds/tasks/opaque%2Ftask/cancel`,
  ]);
  expect(JSON.parse(String(requests[0]!.init?.body))).toEqual({
    operation,
    kind: "copies",
  });
  expect(requests[0]!.init?.method).toBe("POST");
  expect(requests[2]!.init?.method).toBe("POST");
  expect(requests[2]!.init?.body).toBeUndefined();
  for (const request of requests)
    expect(request.init?.signal).toBeInstanceOf(AbortSignal);
  source.close();
});
test("native kind task reset remains an explicit error without graph or inventory fallback", async () => {
  let requests = 0;
  const source = new HttpStoreDataSource(
    repository,
    null,
    createStoreQueryClient(),
    "kind-session",
    () => {},
    async () => {
      requests++;
      return new Response(JSON.stringify({ code: "index_task_expired" }), {
        status: 410,
        headers: { "content-type": "application/json" },
      });
    },
  );
  await expect(source.kindIndexTask("expired")).rejects.toMatchObject({
    code: "index_task_expired",
  });
  expect(requests).toBe(1);
  source.close();
});
