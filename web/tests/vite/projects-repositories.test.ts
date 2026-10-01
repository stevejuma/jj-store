import { expect, test } from "vitest";
import { ApiError, Client } from "../../src/lib/api/client.js";
import { createProjectsRepositories } from "../../src/lib/controller/projects-repositories.svelte.js";
import { readRepositoryDirectory } from "../../src/lib/controller/http-repository-directory.js";
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
  const old = directory.load(repositories.slice(0, 1));
  await directory.load([]);
  release(seed);
  await old;
  expect(directory.entries).toHaveLength(0);
});

test("native directory pins the exact operation, marks paging and rejects foreign snapshots", async () => {
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
              data: {
                graph: "log",
                nodes: [
                  {
                    commit: {
                      id: "71".repeat(64),
                      description: "Latest native change\nBody",
                      author: seed.contributor,
                      committer: seed.contributor,
                    },
                  },
                ],
              },
            },
            next_cursor: "more",
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
  expect(queries.filter((query) => query.includes("/log"))).toHaveLength(2);
});
