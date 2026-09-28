import { expect, test } from "vitest";
import { ApiError } from "../../src/lib/api/client.js";
import { HttpStoreDataSource } from "../../src/lib/controller/http-store-data-source.js";
import { createStoreQueryClient } from "../../src/lib/controller/store-query-client.js";

const repository = "ab".repeat(16);

function json(value: unknown, status = 200) {
  return new Response(JSON.stringify(value), {
    status,
    headers: { "content-type": "application/json" },
  });
}

test("query cache reuses reads within one authentication session", async () => {
  const queryClient = createStoreQueryClient();
  let requests = 0;
  const source = new HttpStoreDataSource(
    repository,
    "token-a",
    queryClient,
    "session-a",
    () => {
      throw new Error("credential should remain authorized");
    },
    async () => {
      requests += 1;
      return json({ version: 1, operations: true });
    },
  );

  const first = await source.capabilities();
  const second = await source.capabilities();

  expect(requests).toBe(1);
  expect(second).toEqual(first);
  source.close();
});

test("query keys isolate credentials that reconnect to the same repository", async () => {
  const queryClient = createStoreQueryClient();
  const credentials: string[] = [];
  const fetcher: typeof fetch = async (_input, init) => {
    credentials.push(new Headers(init?.headers).get("authorization") ?? "");
    return json({ version: 1, operations: true });
  };
  const first = new HttpStoreDataSource(
    repository,
    "token-a",
    queryClient,
    "session-a",
    () => {},
    fetcher,
  );
  const second = new HttpStoreDataSource(
    repository,
    "token-b",
    queryClient,
    "session-b",
    () => {},
    fetcher,
  );

  await first.capabilities();
  await second.capabilities();

  expect(credentials).toEqual(["Bearer token-a", "Bearer token-b"]);
  first.close();
  second.close();
});

test("authorization and nontransient API failures are never retried", async () => {
  const queryClient = createStoreQueryClient();
  let attempts = 0;

  await expect(
    queryClient.fetchQuery({
      queryKey: ["api-error"],
      queryFn: () => {
        attempts += 1;
        throw new ApiError("access_denied");
      },
    }),
  ).rejects.toThrow("access denied");

  expect(attempts).toBe(1);
  queryClient.clear();
});

test("temporary repository busy responses are retried", async () => {
  const queryClient = createStoreQueryClient();
  let attempts = 0;

  const result = await queryClient.fetchQuery({
    queryKey: ["busy-then-ready"],
    queryFn: () => {
      attempts += 1;
      if (attempts === 1) throw new ApiError("busy");
      return "ready";
    },
  });

  expect(result).toBe("ready");
  expect(attempts).toBe(2);
  queryClient.clear();
});

test("closing a source aborts requests and clears protected cache entries", async () => {
  const queryClient = createStoreQueryClient();
  let observedSignal: AbortSignal | undefined;
  const source = new HttpStoreDataSource(
    repository,
    "token-a",
    queryClient,
    "session-a",
    () => {},
    async (_input, init) => {
      observedSignal = init?.signal as AbortSignal;
      return await new Promise<Response>((_resolve, reject) => {
        observedSignal!.addEventListener(
          "abort",
          () => reject(new DOMException("Aborted", "AbortError")),
          { once: true },
        );
      });
    },
  );

  const pending = source.capabilities();
  await new Promise((resolve) => setTimeout(resolve, 0));
  source.close();

  await expect(pending).rejects.toBeDefined();
  expect(observedSignal?.aborted).toBe(true);
  expect(queryClient.getQueryCache().getAll()).toHaveLength(0);
});

test("path reads preserve directory, recursive, and opaque cursor queries", async () => {
  const queryClient = createStoreQueryClient();
  const requests: URL[] = [];
  const source = new HttpStoreDataSource(
    repository,
    "token-a",
    queryClient,
    "session-a",
    () => {},
    async (input) => {
      requests.push(
        new URL(
          typeof input === "string"
            ? input
            : input instanceof URL
              ? input
              : input.url,
          "http://jj-store.test",
        ),
      );
      return json({});
    },
  );
  const operation = "cd".repeat(64);
  const commit = "ef".repeat(64);

  await source.paths(commit, operation);
  await source.paths(commit, operation, { path: "src", recursive: false });
  await source.paths(commit, operation, {
    recursive: true,
    cursor: "opaque-cursor",
  });

  expect(requests.map((request) => request.pathname)).toEqual([
    `/v1/repos/${repository}/commits/${commit}/paths`,
    `/v1/repos/${repository}/commits/${commit}/paths`,
    `/v1/repos/${repository}/commits/${commit}/paths`,
  ]);
  expect(
    requests.map((request) => Object.fromEntries(request.searchParams)),
  ).toEqual([
    { at_op: operation, path: "", recursive: "false", limit: "50" },
    { at_op: operation, path: "src", recursive: "false", limit: "50" },
    {
      at_op: operation,
      path: "",
      recursive: "true",
      limit: "50",
      cursor: "opaque-cursor",
    },
  ]);
  source.close();
});

test("search pages preserve the snapshot query and opaque cursor", async () => {
  const queryClient = createStoreQueryClient();
  const requests: URL[] = [];
  const source = new HttpStoreDataSource(
    repository,
    "token-a",
    queryClient,
    "session-a",
    () => {},
    async (input) => {
      requests.push(new URL(String(input), "http://jj-store.test"));
      return json({ results: [], next_cursor: null });
    },
  );
  const operation = "cd".repeat(64);
  await source.search({ query: "native storage", operation });
  await source.search({
    query: "native storage",
    operation,
    cursor: "opaque-search-cursor",
  });

  expect(requests.map((request) => request.pathname)).toEqual([
    `/v1/repos/${repository}/search`,
    `/v1/repos/${repository}/search`,
  ]);
  expect(
    requests.map((request) => Object.fromEntries(request.searchParams)),
  ).toEqual([
    {
      q: "native storage",
      mode: "visible_commits",
      at_op: operation,
      page_size: "50",
    },
    {
      q: "native storage",
      mode: "visible_commits",
      at_op: operation,
      page_size: "50",
      cursor: "opaque-search-cursor",
    },
  ]);
  source.close();
});
