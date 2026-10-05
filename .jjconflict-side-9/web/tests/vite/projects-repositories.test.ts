import { expect, test } from "vitest";
import { ApiError, Client } from "../../src/lib/api/client.js";
import { createProjectsRepositories } from "../../src/lib/controller/projects-repositories.svelte.js";
import { readRepositoryDirectory } from "../../src/lib/controller/http-repository-directory.js";
import { readLocalRepositoryDirectory } from "../../src/lib/controller/local-repository-directory.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
import { repositoryActivity } from "../../src/lib/ui/store-work/repository-directory-presentation.js";
import {
  fixtureTransport,
  homeA,
  homeB,
  projectId,
} from "../../../catalog/fixtures/projects-hub-fixture.js";

const repositories = [
  { id: homeA, name: "Parser", permission: "admin" as const },
  { id: homeB, name: "Browser", permission: "read" as const },
];

test("native code card reads a bounded head summary without requesting history", async () => {
  const work = fixtureTransport(homeA);
  const seed = await work.directory!();
  const queries: string[] = [];
  const client = new Client(
    homeA,
    null,
    () => {},
    async (input) => {
      const url = String(input);
      queries.push(url);
      if (url.includes("/snapshot"))
        return new Response(
          JSON.stringify({ ...seed.code, heads: seed.code!.heads }),
          { headers: { "content-type": "application/json" } },
        );
      if (!url.includes("/code/summary"))
        throw new Error("History graph is unnecessary");
      return new Response(
        JSON.stringify({
          snapshot: seed.code,
          data: {
            version: 1,
            commits: 1,
            commits_partial: true,
            latest: {
              id: "71".repeat(64),
              description: "Newest selected head\nBody",
              author: seed.contributor,
              committer: seed.contributor,
            },
          },
          next_cursor: null,
        }),
        { headers: { "content-type": "application/json" } },
      );
    },
  );
  const summary = await readRepositoryDirectory(client, work);
  expect(summary.description).toBe("Newest selected head");
  expect(summary.commits).toBe(1);
  expect(summary.commitsPartial).toBe(true);
  expect(queries[1]).toContain("limit=50");
  expect(queries[1]).toContain(`at_op=${seed.code!.operation_id}`);
});

test("local code card consumes pinned head metadata without requesting a graph", async () => {
  const work = fixtureTransport(homeA);
  const seed = await work.directory!();
  const operation = "61".repeat(64);
  let reads = 0;
  const code = {
    version: 1,
    profile: "git_sha1_jj045" as const,
    repository: homeA,
    generation: 1,
    operation,
  };
  const source = {
    repository: homeA,
    profile: code.profile,
    snapshot: async () => ({ heads: [operation], generation: 1 }),
    reviewActivity: work.reviewActivity,
    graph: async () => {
      throw new Error("Graph is unnecessary");
    },
    codeSummary: async (selected: string) => {
      expect(selected).toBe(operation);
      reads++;
      return {
        snapshot: code,
        data: {
          version: 1,
          commits: 1,
          commits_partial: true,
          latest: {
            id: "71".repeat(20),
            description: "Selected local head",
            author: seed.contributor,
            committer: seed.contributor,
          },
        },
      };
    },
  } as unknown as StoreDataSource;
  const summary = await readLocalRepositoryDirectory(source, work);
  expect(reads).toBe(1);
  expect(summary.description).toBe("Selected local head");
  expect(summary.commitsPartial).toBe(true);
});

test("native card review activity avoids thread presentation and keeps sample uncertainty", async () => {
  const fixture = fixtureTransport(homeA);
  const seed = await fixture.directory!();
  const work = {
    ...fixture,
    reviews: async () => {
      throw new Error("Review presentation is unnecessary");
    },
    reviewActivity: async () => ({
      version: 1,
      snapshot: seed.review!,
      activity: { threads: 50, threads_partial: true, updated_at_millis: 9999 },
    }),
  };
  const client = new Client(
    homeA,
    null,
    () => {},
    async () =>
      new Response(
        JSON.stringify({
          ...seed.code,
          heads: ["61".repeat(64), "62".repeat(64)],
        }),
        { headers: { "content-type": "application/json" } },
      ),
  );
  const summary = await readRepositoryDirectory(client, work);
  expect(summary.reviews).toBe(50);
  expect(summary.reviewsPartial).toBe(true);
  expect(summary.updatedAtMillis).toBe(9999);
  expect(summary).not.toHaveProperty("threads");
});

test("local card review activity avoids presentation and retains its code task date policy", async () => {
  const fixture = fixtureTransport(homeA);
  const seed = await fixture.directory!();
  const source = {
    repository: homeA,
    snapshot: async () => ({ heads: ["61".repeat(64), "62".repeat(64)] }),
    reviewThreads: async () => {
      throw new Error("Review presentation is unnecessary");
    },
    reviewActivity: async () => ({
      version: 1,
      snapshot: seed.review!,
      activity: { threads: 3, threads_partial: false, updated_at_millis: 9999 },
    }),
  } as unknown as StoreDataSource;
  const summary = await readLocalRepositoryDirectory(source, fixture);
  expect(summary.reviews).toBe(3);
  expect(summary.reviewsPartial).toBe(false);
  expect(summary.updatedAtMillis).not.toBe(9999);
  expect(summary).not.toHaveProperty("threads");
});

test("native card compact review activity rejects a foreign repository or epoch", async () => {
  const fixture = fixtureTransport(homeA);
  const seed = await fixture.directory!();
  const client = new Client(
    homeA,
    null,
    () => {},
    async () =>
      new Response(
        JSON.stringify({
          ...seed.code,
          heads: ["61".repeat(64), "62".repeat(64)],
        }),
        { headers: { "content-type": "application/json" } },
      ),
  );
  for (const foreign of [{ repo_id: homeB }, { epoch: "ee".repeat(16) }]) {
    const work = {
      ...fixture,
      reviewActivity: async () => ({
        version: 1,
        snapshot: { ...seed.review!, ...foreign },
        activity: { threads: 1, threads_partial: false, updated_at_millis: 0 },
      }),
    };
    await expect(readRepositoryDirectory(client, work)).rejects.toMatchObject({
      code: "snapshot_mismatch",
    });
  }
});

test("native directory reads compact complete task groups without fetching task summaries", async () => {
  const source = fixtureTransport(homeA);
  const seed = await source.directory!();
  if (!("epoch" in seed.work))
    throw new Error("Expected native fixture snapshot");
  const workSnapshot = seed.work;
  let activityReads = 0;
  const work = {
    ...source,
    browse: async () => {
      throw new Error("Task summary hydration is unnecessary");
    },
    taskActivity: async () => {
      activityReads++;
      return {
        version: 1,
        snapshot: workSnapshot,
        total_groups: 1,
        next_cursor: null,
        groups: [
          {
            project: projectId,
            repository: homeB,
            tasks: 1200,
            updated_at_millis: 99,
          },
        ],
      };
    },
  };
  const client = new Client(
    homeA,
    null,
    () => {},
    async () =>
      new Response(
        JSON.stringify({
          ...seed.code,
          heads: ["61".repeat(64), "62".repeat(64)],
        }),
        { headers: { "content-type": "application/json" } },
      ),
  );
  const summary = await readRepositoryDirectory(client, work);
  expect(activityReads).toBe(1);
  const entries = [
    { repository: repositories[0]!, summary, loading: false, error: "" },
  ];
  expect(repositoryActivity(entries, homeB).tasks).toBe(1200);
  expect(
    repositoryActivity(entries, homeB, { home: homeA, id: projectId }).tasks,
  ).toBe(1200);
  expect(
    repositoryActivity(entries, homeB, { home: homeB, id: projectId }).tasks,
  ).toBe(0);
});

test("local directory uses compact task groups and retains unloaded-group partial state", async () => {
  const fixture = fixtureTransport(homeA);
  const snapshot = {
    profile: "local_jj_git" as const,
    repo_id: homeA,
    metadata_commit: "65".repeat(20),
    work_state_version: 1,
  };
  const source = {
    repository: homeA,
    snapshot: async () => ({ heads: ["61".repeat(64), "62".repeat(64)] }),
    reviewActivity: fixture.reviewActivity,
  } as unknown as StoreDataSource;
  let calls = 0;
  const summary = await readLocalRepositoryDirectory(source, {
    ...fixture,
    browse: async () => {
      throw new Error("Task browse should remain unused");
    },
    taskActivity: async () => {
      calls++;
      return {
        version: 2,
        snapshot,
        total_groups: 300,
        next_cursor: "more",
        groups: [
          {
            project: projectId,
            repository: homeB,
            tasks: 1200,
            updated_at_millis: 123,
          },
        ],
      };
    },
  });
  expect(calls).toBe(1);
  expect(summary.work).toEqual(snapshot);
  expect(summary.tasksPartial).toBe(true);
  const activity = repositoryActivity(
    [{ repository: repositories[0]!, summary, loading: false, error: "" }],
    homeB,
  );
  expect(activity.tasks).toBe(1200);
  expect(activity.partial).toBe(true);
  expect(activity.updatedAtMillis).toBe(123);
});

test("directory retries retain an error episode until successful recovery, including queued reads", async () => {
  const targets = [
    ...repositories,
    { ...repositories[0]!, id: "cc".repeat(16) },
  ];
  let fail = true;
  let release: (() => void) | undefined;
  let gate = Promise.resolve();
  const directory = createProjectsRepositories({
    source() {
      const source = fixtureTransport(homeA);
      const original = source.directory!;
      source.directory = async () => {
        await gate;
        if (fail) throw new Error("Summary unavailable");
        return original();
      };
      return source;
    },
  });
  targets.forEach((repository) => directory.watch(repository.id));
  await directory.load(targets);
  gate = new Promise<void>((resolve) => (release = resolve));
  const retry = directory.load(targets, true);
  expect(directory.entries.every((entry) => entry.loading)).toBe(true);
  expect(directory.entries.map((entry) => entry.error)).toEqual(
    targets.map(() => "Summary unavailable"),
  );
  release!();
  await retry;
  expect(
    directory.entries.every((entry) => entry.error === "Summary unavailable"),
  ).toBe(true);
  fail = false;
  await directory.load(targets, true);
  expect(
    directory.entries.every(
      (entry) => !entry.error && !entry.loading && entry.summary,
    ),
  ).toBe(true);
});

test("repository activity uses independent unfiltered pins and stays until explicit refresh", async () => {
  let calls = 0;
  const directory = createProjectsRepositories({
    source(id) {
      const source = fixtureTransport(id),
        original = source.directory!;
      source.directory = async (signal) => {
        calls++;
        return original(signal);
      };
      return source;
    },
  });
  repositories.forEach((repository) => directory.watch(repository.id));
  await directory.load(repositories);
  expect(calls).toBe(2);
  const activity = repositoryActivity(directory.entries, homeB);
  expect(activity.tasks).toBe(2);
  expect(activity.partial).toBe(false);
  expect(
    repositoryActivity(directory.entries, homeB, { home: homeA, id: projectId })
      .tasks,
  ).toBe(1);
  await directory.load(repositories);
  expect(calls).toBe(2);
  await directory.load(repositories, true);
  expect(calls).toBe(4);
  await directory.load(repositories.slice(0, 1));
  expect(directory.entries.map((entry) => entry.repository.id)).toEqual([
    homeA,
  ]);
});

test("revoked directory reads clear old facts while failures remain separate", async () => {
  let denied = false;
  const directory = createProjectsRepositories({
    source(id) {
      const source = fixtureTransport(id),
        original = source.directory!;
      source.directory = async (signal) => {
        if (denied && id === homeA) throw new ApiError("access_denied");
        return original(signal);
      };
      return source;
    },
  });
  repositories.forEach((repository) => directory.watch(repository.id));
  await directory.load(repositories);
  denied = true;
  await directory.load(repositories, true);
  expect(directory.entries[0]?.summary).toBeNull();
  expect(directory.entries[1]?.summary?.description).toBe(
    "Community browser interface",
  );
  expect(repositoryActivity(directory.entries, homeB).partial).toBe(true);
});

test("late directory results cannot replace a newer authorized view", async () => {
  const seed = await fixtureTransport(homeA).directory!();
  let release!: (value: typeof seed) => void;
  const pending = new Promise<typeof seed>((resolve) => {
    release = resolve;
  });
  const directory = createProjectsRepositories({
    source: (id) => ({ ...fixtureTransport(id), directory: () => pending }),
  });
  directory.watch(homeA);
  const old = directory.load(repositories.slice(0, 1));
  await directory.load([]);
  release(seed);
  await old;
  expect(directory.entries).toHaveLength(0);
});

test("native directory pins the exact operation, marks count uncertainty and rejects foreign snapshots", async () => {
  const source = fixtureTransport(homeA),
    seed = await source.directory!();
  const queries: string[] = [];
  let foreign = false,
    multiple = false;
  const client = new Client(
    homeA,
    null,
    () => {},
    async (input) => {
      const url = String(input);
      queries.push(url);
      const value = url.includes("/snapshot")
        ? {
            ...seed.code,
            heads: multiple
              ? ["61".repeat(64), "62".repeat(64)]
              : seed.code!.heads,
          }
        : {
            snapshot: {
              ...seed.code,
              operation_id: foreign ? "62".repeat(64) : seed.code!.operation_id,
            },
            data: {
              version: 1,
              commits: 1,
              commits_partial: true,
              latest: {
                id: "71".repeat(64),
                description: "Latest native change\nBody",
                author: seed.contributor,
                committer: seed.contributor,
              },
            },
            next_cursor: null,
          };
      return new Response(JSON.stringify(value), {
        headers: { "content-type": "application/json" },
      });
    },
  );
  const read = await readRepositoryDirectory(client, source);
  expect(read.description).toBe("Latest native change");
  expect(read.commitsPartial).toBe(true);
  expect(queries[1]).toContain(`at_op=${seed.code!.operation_id}`);
  foreign = true;
  await expect(readRepositoryDirectory(client, source)).rejects.toMatchObject({
    code: "snapshot_mismatch",
  });
  multiple = true;
  const ambiguous = await readRepositoryDirectory(client, source);
  expect(ambiguous.code).toBeNull();
  expect(ambiguous.description).toBe("Multiple operation heads");
  expect(ambiguous.commitsPartial).toBe(true);
  expect(
    queries.filter((query) => query.includes("/code/summary")),
  ).toHaveLength(2);
  expect(queries.some((query) => query.includes("/log"))).toBe(false);
});
