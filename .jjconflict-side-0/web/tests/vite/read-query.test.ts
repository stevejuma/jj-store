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

test("an immediate same-key retry cannot inherit released transport", async () => {
  const queries = createStoreQueryClient();
  const abort = new AbortController();
  const signals: AbortSignal[] = [];
  const options = {
    queryKey: ["immediate-retry"],
    staleTime: 0,
    queryFn: ({ signal }: { signal: AbortSignal }) => {
      signals.push(signal);
      return signals.length === 1
        ? new Promise<string>(() => {})
        : Promise.resolve("fresh");
    },
  };
  const pending = readQuery(queries, options, abort.signal);
  const rejected = expect(pending).rejects.toBeDefined();
  abort.abort();
  expect(signals[0].aborted).toBe(true);
  const retry = readQuery(queries, options);
  await rejected;
  expect(await retry).toBe("fresh");
  expect(signals).toHaveLength(2);
  expect(signals[1].aborted).toBe(false);
  queries.clear();
});
