import { expect, test } from "vitest";
import { HttpStoreDataSource } from "../../src/lib/controller/http-store-data-source.js";
import { HttpLocalStoreDataSource } from "../../src/lib/controller/http-local-store-data-source.js";
import { createStoreQueryClient } from "../../src/lib/controller/store-query-client.js";

const repository = "11".repeat(16),
  operation = "22".repeat(64);
const localHead = {
  version: 1,
  profile: "git_sha1_jj045",
  repository,
  generation: 1,
  heads: [operation],
  metadata_commit: "44".repeat(20),
  actor: "55".repeat(16),
};
const json = (value: unknown) =>
  new Response(JSON.stringify(value), {
    headers: { "content-type": "application/json" },
  });

for (const profile of ["native", "local"] as const) {
  test(`${profile} retrying identical query text gets a fresh request after cancellation`, async () => {
    let reads = 0;
    const signals: AbortSignal[] = [];
    const fetcher: typeof fetch = async (url, init) => {
      if (String(url).endsWith("snapshot")) return json(localHead);
      reads++;
      const signal = init!.signal!;
      signals.push(signal);
      if (reads === 1)
        return new Promise<Response>((_resolve, reject) => {
          signal.addEventListener(
            "abort",
            () => reject(new DOMException("Aborted", "AbortError")),
            { once: true },
          );
        });
      return json(
        profile === "native"
          ? { data: { version: 1 } }
          : {
              snapshot: {
                version: 1,
                profile: localHead.profile,
                repository,
                generation: 1,
                operation,
              },
              data: { kind: "graph", data: { version: 1 } },
              next_cursor: null,
              previous_cursor: null,
            },
      );
    };
    const source =
      profile === "native"
        ? new HttpStoreDataSource(
            repository,
            null,
            createStoreQueryClient(),
            "test",
            () => {},
            fetcher,
          )
        : new HttpLocalStoreDataSource(repository, fetcher);
    if (profile === "local") await source.snapshot();
    const first = new AbortController();
    const pending = source.graph("log", {
      operation,
      revset: "all()",
      atTimeMillis: 12345,
      signal: first.signal,
    });
    const rejected = expect(pending).rejects.toBeDefined();
    await new Promise((resolve) => setTimeout(resolve, 0));
    first.abort();
    const result = await source.graph("log", {
      operation,
      revset: "all()",
      atTimeMillis: 12345,
      signal: new AbortController().signal,
    });
    await rejected;
    expect(reads).toBe(2);
    expect(signals[0].aborted).toBe(true);
    expect(signals[1].aborted).toBe(false);
    expect(result.data.version).toBe(1);
    source.close();
  });
}

test("native language metadata rejects a different operation pin", async () => {
  const source = new HttpStoreDataSource(
    repository,
    null,
    createStoreQueryClient(),
    "test",
    () => {},
    async () =>
      json({
        snapshot: { operation_id: "33".repeat(64) },
        data: { version: 1 },
      }),
  );
  await expect(source.queryInfo(operation)).rejects.toThrow(
    "snapshot mismatch",
  );
  source.close();
});
