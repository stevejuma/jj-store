import { expect, test, vi } from "vitest";
import { createProjectsRepositories } from "../../src/lib/controller/projects-repositories.svelte.js";
import { readLocalRepositoryDirectory } from "../../src/lib/controller/local-repository-directory.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
import {
  fixtureTransport,
  homeA,
} from "../../../catalog/fixtures/projects-hub-fixture.js";

const repositories = Array.from({ length: 100 }, (_, index) => ({
  id: index.toString(16).padStart(32, "0"),
  name: `Repository ${index}`,
  permission: "read" as const,
}));

test("directory discovery shows every repository without reading hidden card metadata", async () => {
  let calls = 0;
  const directory = createProjectsRepositories({
    source() {
      const source = fixtureTransport(homeA);
      const original = source.directory!;
      source.directory = async (signal) => {
        calls++;
        return original(signal);
      };
      return source;
    },
  });
  await directory.load(repositories);
  expect(directory.entries).toHaveLength(100);
  expect(calls).toBe(0);
  expect(
    directory.entries.every((entry) => !entry.loading && !entry.summary),
  ).toBe(true);
});

test("visible card reads stay at two while canceled transports settle and late output is discarded", async () => {
  const seed = await fixtureTransport(homeA).directory!();
  const calls: {
    id: string;
    signal: AbortSignal;
    resolve(value: typeof seed): void;
  }[] = [];
  let active = 0,
    maximum = 0;
  const directory = createProjectsRepositories({
    source(id) {
      return {
        ...fixtureTransport(homeA),
        directory: async (signal) => {
          maximum = Math.max(maximum, ++active);
          try {
            return await new Promise<typeof seed>((resolve) => {
              calls.push({ id, signal: signal!, resolve });
            });
          } finally {
            active--;
          }
        },
      };
    },
  });
  await directory.load(repositories);
  const releases = repositories
    .slice(0, 3)
    .map((repository) => directory.watch(repository.id));
  expect(calls.map((call) => call.id)).toEqual(
    repositories.slice(0, 2).map((repository) => repository.id),
  );
  expect(directory.entries[2]!.loading).toBe(true);
  const refresh = directory.load(repositories, true);
  releases[2]!();
  expect(calls).toHaveLength(2);
  expect(calls.every((call) => call.signal.aborted)).toBe(true);
  calls[0]!.resolve({ ...seed, description: "Obsolete result" });
  calls[1]!.resolve({ ...seed, description: "Obsolete result" });
  await vi.waitFor(() => expect(calls).toHaveLength(4));
  expect(
    directory.entries.slice(0, 3).every((entry) => entry.summary === null),
  ).toBe(true);
  calls[2]!.resolve({ ...seed, description: "Current result" });
  calls[3]!.resolve({ ...seed, description: "Current result" });
  await refresh;
  expect(maximum).toBe(2);
  expect(directory.entries[0]!.summary?.description).toBe("Current result");
  expect(directory.entries[1]!.summary?.description).toBe("Current result");
  expect(directory.entries[2]!.loading).toBe(false);
  releases.forEach((release) => release());
});

test("refresh reads the shown window and defers hidden stale rows until they return", async () => {
  const calls: string[] = [];
  const directory = createProjectsRepositories({
    source(id) {
      const source = fixtureTransport(homeA),
        original = source.directory!;
      return {
        ...source,
        directory: async (signal) => {
          calls.push(id);
          return original(signal);
        },
      };
    },
  });
  const first = directory.watch(repositories[0]!.id),
    second = directory.watch(repositories[1]!.id);
  await directory.load(repositories);
  first();
  await directory.load(repositories, true);
  expect(calls).toEqual([
    repositories[0]!.id,
    repositories[1]!.id,
    repositories[1]!.id,
  ]);
  const again = directory.watch(repositories[0]!.id);
  await vi.waitFor(() => expect(calls).toHaveLength(4));
  expect(calls[3]).toBe(repositories[0]!.id);
  await vi.waitFor(() => expect(directory.entries[0]!.loading).toBe(false));
  again();
  second();
});

test("overlapping viewport leases deduplicate reads and retired leases cannot cancel a new session", async () => {
  const signals: AbortSignal[] = [];
  const directory = createProjectsRepositories({
    source() {
      const source = fixtureTransport(homeA),
        original = source.directory!;
      return {
        ...source,
        directory: async (signal) => {
          signals.push(signal!);
          return original(signal);
        },
      };
    },
  });
  const first = directory.watch(repositories[0]!.id),
    duplicate = directory.watch(repositories[0]!.id);
  await directory.load(repositories);
  expect(signals).toHaveLength(1);
  first();
  expect(signals[0]!.aborted).toBe(false);
  directory.reset();
  const fresh = directory.watch(repositories[0]!.id);
  await directory.load(repositories);
  duplicate();
  expect(signals).toHaveLength(2);
  expect(signals[1]!.aborted).toBe(false);
  fresh();
});

test("canceling a local card during a shared snapshot read starts no collaboration or graph work", async () => {
  let settle!: () => void;
  const snapshot = new Promise<void>((resolve) => {
    settle = resolve;
  });
  const reviews = vi.fn(),
    graph = vi.fn(),
    browse = vi.fn();
  const source = {
    snapshot: async () => {
      await snapshot;
      return { heads: ["61".repeat(64)] };
    },
    reviewThreads: reviews,
    graph,
  } as unknown as StoreDataSource;
  const canceled = new AbortController();
  const read = readLocalRepositoryDirectory(
    source,
    { ...fixtureTransport(homeA), browse },
    canceled.signal,
  );
  canceled.abort();
  settle();
  await expect(read).rejects.toMatchObject({ name: "AbortError" });
  expect(browse).not.toHaveBeenCalled();
  expect(reviews).not.toHaveBeenCalled();
  expect(graph).not.toHaveBeenCalled();
});

test("a failed directory read cancels its remaining metadata transports and retains the error", async () => {
  let signal!: AbortSignal;
  const directory = createProjectsRepositories({
    source() {
      return {
        ...fixtureTransport(homeA),
        directory: async (current) => {
          signal = current!;
          throw new Error("Metadata unavailable");
        },
      };
    },
  });
  const release = directory.watch(repositories[0]!.id);
  await directory.load(repositories);
  expect(signal.aborted).toBe(true);
  expect(directory.entries[0]!.error).toBe("Metadata unavailable");
  expect(directory.entries[0]!.loading).toBe(false);
  release();
});
