import { expect, test } from "vitest";
import { createStoreQueryClient } from "../../src/lib/controller/store-query-client.js";
import { readQuery } from "../../src/lib/controller/read-query.js";

test("one observer cancellation cannot abort another consumer's shared read", async () => {
  const queries = createStoreQueryClient(),
    first = new AbortController(),
    second = new AbortController();
  let shared: AbortSignal | undefined, finish!: (value: string) => void;
  let calls = 0;
  const options = {
    queryKey: ["shared"],
    queryFn: ({ signal }: { signal: AbortSignal }) => {
      shared = signal;
      calls++;
      return new Promise<string>((resolve) => {
        finish = resolve;
      });
    },
  };
  const a = readQuery(queries, options, first.signal),
    b = readQuery(queries, options, second.signal);
  first.abort();
  await expect(a).rejects.toBeDefined();
  expect(shared?.aborted).toBe(false);
  finish("ready");
  expect(await b).toBe("ready");
  expect(calls).toBe(1);
  queries.clear();
});
test("the final observer release aborts obsolete transport", async () => {
  const queries = createStoreQueryClient(),
    abort = new AbortController();
  let shared: AbortSignal | undefined;
  const pending = readQuery(
    queries,
    {
      queryKey: ["abandoned"],
      queryFn: ({ signal }) => {
        shared = signal;
        return new Promise<string>(() => {});
      },
    },
    abort.signal,
  );
  abort.abort();
  await expect(pending).rejects.toBeDefined();
  expect(shared?.aborted).toBe(true);
  queries.clear();
});
