import { expect, test, vi } from "vitest";
import { createProductionStacks } from "../../src/lib/controller/production-stacks.svelte.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import { initialStoreRoute } from "../../src/lib/controller/store-route-history.svelte.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
import { ApiError } from "../../src/lib/api/client.js";

const repository = "b".repeat(32),
  operation = "a".repeat(128);
function fixture() {
  const data = createProductionStoreState();
  data.accessMode = "local_trusted";
  data.capabilities = { read_api: true } as never;
  let route = {
    ...initialStoreRoute(),
    repository,
    page: "stacks" as const,
    snapshotOperationId: operation,
  };
  const binding = {
    repository,
    name: "Local",
    path: "/registered/local",
    availability: "ready",
    generation: 2,
  };
  const pin = {
    version: 1,
    profile: "git_sha1_jj045",
    repository,
    operation,
    generation: 2,
    heads: [operation],
  };
  const source = {
    repository,
    profile: "git_sha1_jj045",
    snapshot: vi.fn(async () => ({
      ...pin,
      metadata_commit: null,
      actor: null,
    })),
    references: vi.fn(async () => ({
      snapshot: pin,
      next_cursor: null,
      data: { version: 1, operation, references: [] },
    })),
    close: vi.fn(),
  } as unknown as StoreDataSource;
  let current: StoreDataSource | null = source;
  const fetcher = vi.fn<typeof fetch>(
    async () =>
      new Response(
        JSON.stringify({ version: 1, revision: 1, bindings: [binding] }),
        { headers: { "content-type": "application/json" } },
      ),
  );
  const stacks = createProductionStacks({
    data,
    source: () => current,
    route: () => route,
    fetcher,
  });
  return {
    data,
    source,
    binding,
    stacks,
    fetcher,
    setSource: (value: StoreDataSource | null) => (current = value),
    setOperation: (value: string) =>
      (route = { ...route, snapshotOperationId: value }),
  };
}
test("a verified binding exposes owned read-only Stacks without loading details on connect", async () => {
  const { stacks, source, fetcher } = fixture();
  await stacks.restore(false);
  expect(stacks.controller?.snapshot?.folder).toBe("/registered/local");
  expect(source.references).not.toHaveBeenCalled();
  await stacks.restore();
  expect(stacks.controller?.snapshot?.capabilities).toEqual([]);
  expect(source.references).toHaveBeenCalledTimes(1);
  expect(fetcher.mock.calls[0]?.[0]).toBe("/v1/local/repositories");
  stacks.clear();
  expect(stacks.controller).toBeUndefined();
  expect(source.close).not.toHaveBeenCalled();
});
test.each(["native", "authenticated", "missing", "unavailable", "generation"])(
  "%s cannot expose a local stacks controller",
  async (reason) => {
    const { stacks, source, data, binding, fetcher } = fixture();
    if (reason === "native") Object.assign(source, { profile: "native" });
    if (reason === "authenticated") data.accessMode = "authenticated";
    if (reason === "unavailable") binding.availability = "unavailable";
    if (reason === "generation") binding.generation = 3;
    if (reason === "missing")
      fetcher.mockImplementation(
        async () =>
          new Response(
            JSON.stringify({ version: 1, revision: 1, bindings: [] }),
            { headers: { "content-type": "application/json" } },
          ),
      );
    await stacks.restore();
    expect(stacks.controller).toBeUndefined();
    expect(source.references).not.toHaveBeenCalled();
    stacks.clear();
  },
);
test("clearing during discovery aborts it and late results cannot expose a controller", async () => {
  const { stacks, fetcher } = fixture();
  let finish!: (response: Response) => void;
  fetcher.mockImplementation(
    () => new Promise((resolve) => (finish = resolve)),
  );
  const read = stacks.restore();
  const signal = fetcher.mock.calls[0]?.[1]?.signal;
  stacks.clear();
  expect(signal?.aborted).toBe(true);
  finish(
    new Response(JSON.stringify({ version: 1, bindings: [] }), {
      headers: { "content-type": "application/json" },
    }),
  );
  await read;
  expect(stacks.controller).toBeUndefined();
});
test("missing graph index uses the existing preparation notice rather than a duplicate stack error", async () => {
  const { stacks, source, data } = fixture();
  source.references = vi.fn(async () => {
    throw new ApiError("index_rebuild_required");
  });
  await stacks.restore();
  expect(stacks.controller?.snapshot?.folder).toBe("/registered/local");
  expect(stacks.controller?.error).toBe("");
  expect(data.status.error).toContain("index rebuild required");
  stacks.clear();
});

test("binding revalidation preserves an in-flight canvas selection read", async () => {
  const { stacks, source } = fixture();
  await stacks.restore();
  const controller = stacks.controller!;
  let finish!: () => void;
  const held = new Promise<void>((resolve) => {
    finish = resolve;
  });
  const selection = controller.reads.run(async () => {
    await held;
    return { ...controller.snapshot!, base: "chosen-target" };
  });
  expect(controller.locked).toBe(true);
  const reads = vi.mocked(source.references).mock.calls.length;
  await stacks.restore();
  expect(source.references).toHaveBeenCalledTimes(reads);
  expect(stacks.controller).toBe(controller);
  finish();
  await selection;
  expect(controller.snapshot?.base).toBe("chosen-target");
  stacks.clear();
});
