import { expect, test } from "vitest";
import {
  readNativeAgentContext,
  readAnchoredAgentContext,
  createDeferredAgentContexts,
} from "../../src/lib/controller/native-agent-context.js";
import { readNativeActivityPage } from "../../src/lib/controller/native-activity-reader.js";
import { readNativeReviewContext } from "../../src/lib/controller/native-review-context.js";
import { projectsActivityScope } from "../../src/lib/controller/projects-activity-scope.js";
import { createProjectsActivity } from "../../src/lib/controller/projects-activity.svelte.js";
import {
  projectsRouteFromHash,
  projectsRouteToHash,
} from "../../src/lib/controller/projects-route-codec.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
import type { WorkBrowseTransport } from "../../src/lib/controller/projects-hub-types.js";
import { activityCommit } from "../../../catalog/fixtures/native-activity.js";
import {
  agentDetail,
  agentRelations,
  reviewThreads,
} from "../../../catalog/fixtures/collaboration.js";
import { initialStoreRoute } from "../../src/lib/controller/store-route-history.svelte.js";
import { ApiError } from "../../src/lib/api/client.js";

const repository = agentDetail.snapshot.repo_id,
  operation = "61".repeat(64);
const snapshot = {
  version: 1,
  repo_id: repository,
  epoch: agentDetail.snapshot.epoch,
  published_seq: 1,
  manifest_revision: 1,
  code_state_version: 1,
  heads: [operation],
  operation_id: operation,
};
const relation = agentRelations[0]!;
const commit = relation.association!.after_commits[0]!;
function source(overrides: Partial<StoreDataSource> = {}): StoreDataSource {
  return {
    repository,
    agentSessions: async () => ({
      version: 1,
      snapshot: agentDetail.snapshot,
      sessions: [agentDetail.session],
      next_cursor: null,
    }),
    agentRelations: async () => ({
      version: 1,
      snapshot: agentDetail.snapshot,
      subject: { subject: "revision", commit_id: commit },
      relations: [relation],
      next_cursor: null,
    }),
    agentSession: async () => agentDetail,
    ...overrides,
  } as StoreDataSource;
}

test("virtual activity defers associations until mounted and session details until opened", async () => {
  let relations = 0,
    details = 0;
  const context = createDeferredAgentContexts(
    source({
      async agentRelations() {
        relations++;
        return {
          version: 1,
          snapshot: agentDetail.snapshot,
          subject: { subject: "revision", commit_id: commit },
          relations: [relation],
          next_cursor: null,
        };
      },
      async agentSession() {
        details++;
        return agentDetail;
      },
    }),
    agentDetail.snapshot,
  )({ commit });
  expect([relations, details]).toEqual([0, 0]);
  const discovered = await context.discover!();
  expect(discovered.relatedSessionIds).toEqual([
    agentDetail.session.session_id,
  ]);
  expect([relations, details]).toEqual([1, 0]);
  expect(await context.discover!()).toBe(discovered);
  expect((await discovered.load!()).sessions).toHaveLength(1);
  await discovered.load!();
  expect([relations, details]).toEqual([1, 1]);
});

test("lazy context refuses to advance the captured AgentStore root", async () => {
  const context = createDeferredAgentContexts(source(), {
    ...agentDetail.snapshot,
    agent_root: "99".repeat(32),
  })({ commit });
  const discovered = await context.discover!();
  expect(discovered.error).toContain("snapshot changed");
  expect(discovered.sessions).toEqual([]);
  expect(discovered.load).toBeUndefined();
});

test("a virtual page bounds concurrent association readers to two", async () => {
  let active = 0,
    maximum = 0;
  const make = createDeferredAgentContexts(
    source({
      async agentRelations() {
        maximum = Math.max(maximum, ++active);
        await new Promise((resolve) => setTimeout(resolve, 5));
        active--;
        return {
          version: 1,
          snapshot: agentDetail.snapshot,
          subject: { subject: "revision", commit_id: commit },
          relations: [],
          next_cursor: null,
        };
      },
    }),
  );
  await Promise.all(
    Array.from({ length: 12 }, () => make({ commit }).discover!()),
  );
  expect(maximum).toBe(2);
  expect(active).toBe(0);
});

test("agent context follows every pinned relation page and excludes unrelated sessions", async () => {
  const calls: (string | undefined)[] = [],
    sessions: string[] = [];
  const value = await readNativeAgentContext(
    source({
      async agentRelations(subject, cursor) {
        calls.push(cursor);
        return {
          version: 1,
          snapshot: agentDetail.snapshot,
          subject: { subject: "revision", commit_id: commit },
          relations: cursor
            ? [relation]
            : [
                {
                  ...relation,
                  association: {
                    ...relation.association!,
                    session_id: "99".repeat(16),
                    before_commits: [],
                    after_commits: [],
                  },
                },
              ],
          next_cursor: cursor ? null : "next",
        };
      },
      async agentSession(id) {
        sessions.push(id);
        return agentDetail;
      },
    }),
    { commit },
  );
  expect(calls).toEqual([undefined, "next"]);
  expect(sessions).toEqual([agentDetail.session.session_id]);
  expect(value.sessions).toHaveLength(1);
});
test("agent context rejects changed roots and repeated cursors", async () => {
  const changed = await readNativeAgentContext(
    source({
      async agentSession() {
        return {
          ...agentDetail,
          snapshot: { ...agentDetail.snapshot, agent_root: "00".repeat(32) },
        };
      },
    }),
    { commit },
  );
  expect(changed.sessions).toEqual([]);
  expect(changed.error).toContain("snapshot changed");
  const repeated = await readNativeAgentContext(
    source({
      async agentRelations() {
        return {
          version: 1,
          snapshot: agentDetail.snapshot,
          subject: { subject: "revision", commit_id: commit },
          relations: [],
          next_cursor: "same",
        };
      },
    }),
    { commit },
  );
  expect(repeated.error).toContain("Repeated");
});
test("anchored agent activity never substitutes a newer session snapshot", async () => {
  const anchor = reviewThreads.find(
    (thread) => thread.anchor.target.target === "agent_artifact",
  )!.anchor;
  const value = await readAnchoredAgentContext(
    source({
      async agentSession() {
        return {
          ...agentDetail,
          session: { ...agentDetail.session, snapshot_id: "00".repeat(32) },
        };
      },
    }),
    anchor,
  );
  expect(value.sessions).toEqual([]);
  expect(value.error).toContain("original anchored agent snapshot");
  expect(
    (await readAnchoredAgentContext(source(), anchor)).sessions,
  ).toHaveLength(1);
});
test("code-less reviews do not read unanchored code or unrelated agents", async () => {
  const anchor = {
    ...reviewThreads[0]!.anchor,
    target: { target: "repository" as const },
  };
  const value = await readNativeReviewContext(
    source({
      commit: async () => {
        throw new Error("unexpected code read");
      },
    }),
    anchor,
  );
  expect(value.commits).toEqual([]);
  expect(value.files).toEqual([]);
  expect(value.codeError).toBe("");
});
test("activity graph retains the operation and fixed query time across continuations", async () => {
  const queries: unknown[] = [];
  const value = await readNativeActivityPage(
    source({
      async snapshot() {
        throw new Error("must not reconcile heads");
      },
      async graph(kind, query) {
        queries.push(query);
        return {
          snapshot,
          data: { data: { graph: "log", nodes: [] } },
          next_cursor: null,
        };
      },
    }),
    { operation, cursor: "next", atTimeMillis: 123 },
  );
  expect(queries).toEqual([
    { operation, cursor: "next", atTimeMillis: 123, limit: 20 },
  ]);
  expect(value.snapshot).toEqual(snapshot);
});
test("activity graph fails closed for a mismatched repository or operation", async () => {
  await expect(
    readNativeActivityPage(
      source({
        async graph() {
          return {
            snapshot: { ...snapshot, operation_id: "99".repeat(64) },
            data: { data: { graph: "log", nodes: [] } },
            next_cursor: null,
          };
        },
      }),
      { operation },
    ),
  ).rejects.toThrow("snapshot mismatch");
});
test("project and task activity scope requires linked-code authorization", () => {
  const repositories = [
    { id: repository, name: "Home" },
    { id: "11".repeat(16), name: "Linked" },
    { id: "22".repeat(16), name: "Unrelated" },
  ];
  const route = { ...initialStoreRoute(), projectId: "33".repeat(16) };
  const actor = {
    created_by: "local",
    updated_by: "local",
    created_at_millis: 0,
    updated_at_millis: 0,
  };
  const project = {
    schema_version: 1,
    id: route.projectId,
    title: "Native",
    body: "",
    version: 1,
    actor,
    repositories: {
      [repository]: true,
      [repositories[1]!.id]: false,
      ["44".repeat(16)]: true,
    },
  };
  expect(
    projectsActivityScope(repositories, route, project).map((repo) => repo.id),
  ).toEqual([repository, repositories[1]!.id]);
  const task = {
    schema_version: 1,
    id: "55".repeat(16),
    project: project.id,
    title: "Task",
    body: "",
    state: "open" as const,
    version: 1,
    actor,
    repository: repositories[1]!.id,
    parent: null,
  };
  expect(
    projectsActivityScope(repositories, route, project, task).map(
      (repo) => repo.id,
    ),
  ).toEqual([repositories[1]!.id]);
  expect(() => projectsActivityScope(repositories, route)).toThrow(
    "Project unavailable",
  );
});
test("Activity hash routes restore the native project scope", () => {
  const hash = `#/projects?home=${repository}&project=${"33".repeat(16)}&tab=activity`;
  const route = projectsRouteFromHash(hash)!;
  expect(route.workTab).toBe("activity");
  expect(projectsRouteFromHash(projectsRouteToHash(route)!)?.projectId).toBe(
    route.projectId,
  );
});
test("activity refresh is explicit; access revocation clears protected rows", async () => {
  let calls = 0,
    deny = false;
  const page = {
    snapshot,
    atTimeMillis: 123,
    commits: [activityCommit(repository)],
    cursor: null,
  };
  const activity = createProjectsActivity({
    source: () =>
      ({
        async activity() {
          calls++;
          if (deny) throw new ApiError("access_denied");
          return page;
        },
      }) as WorkBrowseTransport,
  });
  const repositories = [{ id: repository, name: "Native" }];
  await activity.load(repositories);
  await activity.load(repositories);
  expect(calls).toBe(1);
  deny = true;
  await activity.load(repositories, true);
  expect(activity.homes[0]!.page).toBeNull();
  activity.reset();
  expect(activity.homes).toEqual([]);
});
test("late activity reads cannot overwrite a newly selected repository scope", async () => {
  let release!: (page: {
    snapshot: typeof snapshot;
    atTimeMillis: number;
    commits: never[];
    cursor: null;
  }) => void;
  const activity = createProjectsActivity({
    source: () =>
      ({
        activity: () =>
          new Promise((resolve) => {
            release = resolve;
          }),
      }) as WorkBrowseTransport,
  });
  const loading = activity.load([{ id: repository, name: "Native" }]);
  activity.reset();
  release({ snapshot, atTimeMillis: 123, commits: [], cursor: null });
  await loading;
  expect(activity.homes).toEqual([]);
});

test("revision review files and commit activity retain the exact anchor comparison", async () => {
  const parent = "70".repeat(64),
    calls: unknown[] = [];
  const author = {
    name: "Native author",
    email: "native@example.invalid",
    timestamp: { millis: 1000, offset_minutes: 0 },
  };
  const nativeCommit = {
    id: commit,
    change_id: "71".repeat(16),
    description: "Anchored native commit",
    parents: [parent],
    author,
    committer: author,
    tree_terms: [],
    conflicted: false,
    bookmarks: [],
    workspaces: [],
    visible: true,
    divergent: false,
  };
  const change = {
    path: "src/lib.rs",
    before: [],
    after: [],
    sources: [],
    content: {
      status: "text" as const,
      hunks: [
        {
          kind: "different" as const,
          before_start: 0,
          after_start: 0,
          before: "old\n",
          after: "new\n",
        },
      ],
    },
  };
  const native = source({
    async commit(id, op) {
      calls.push(["commit", id, op]);
      return { snapshot, data: { commit: nativeCommit } } as Awaited<
        ReturnType<StoreDataSource["commit"]>
      >;
    },
    async conflicts(op, id) {
      calls.push(["conflicts", op, id]);
      return { snapshot, data: { conflicts: [] } } as Awaited<
        ReturnType<StoreDataSource["conflicts"]>
      >;
    },
    async diffPaths(op, from, to) {
      calls.push(["paths", op, from, to]);
      return {
        snapshot,
        data: {
          changes: [
            {
              ...change,
              content: { status: "unavailable", reason: "not_requested" },
            },
          ],
        },
        next_cursor: null,
      } as Awaited<ReturnType<StoreDataSource["diffPaths"]>>;
    },
    async diff(op, from, to, path) {
      calls.push(["patch", op, from, to, path]);
      return { snapshot, data: { changes: [change] } } as Awaited<
        ReturnType<StoreDataSource["diff"]>
      >;
    },
  });
  const anchor = {
    ...reviewThreads[0]!.anchor,
    target: {
      target: "revision" as const,
      value: { operation_id: operation, commit_id: commit, change_id: null },
    },
  };
  const context = await readNativeReviewContext(native, anchor);
  expect(context.codeError).toBe("");
  expect(context.files).toEqual([
    { path: "src/lib.rs", additions: 1, deletions: 1 },
  ]);
  expect(context.patches["src/lib.rs"]).toContain("-old\n+new");
  expect(context.commits[0]!.operation).toBe(operation);
  expect(context.commits[0]!.revision.eventId).toBe(commit);
  expect(context.commits[0]!.agents.sessions).toHaveLength(0);
  const associated = await context.commits[0]!.agents.discover!();
  expect((await associated.load!()).sessions).toHaveLength(1);
  expect(calls).toContainEqual(["paths", operation, parent, commit]);
  expect(calls).toContainEqual([
    "patch",
    operation,
    parent,
    commit,
    "src/lib.rs",
  ]);
});

test("activity continuation rejects another snapshot and preserves displayed rows", async () => {
  const original = {
    snapshot,
    atTimeMillis: 123,
    commits: [activityCommit(repository)],
    cursor: "next",
  };
  let calls = 0;
  const activity = createProjectsActivity({
    source: () =>
      ({
        async activity(query: unknown) {
          calls++;
          if (calls === 1) return original;
          expect(query).toEqual({
            operation,
            cursor: "next",
            atTimeMillis: 123,
          });
          return {
            ...original,
            snapshot: { ...snapshot, manifest_revision: 2 },
            commits: [],
          };
        },
      }) as WorkBrowseTransport,
  });
  await activity.load([{ id: repository, name: "Native" }]);
  await activity.more(repository);
  expect(activity.homes[0]!.error).toContain("snapshot mismatch");
  expect(activity.homes[0]!.page).toBe(original);
});

test("an explicit home operation replaces a cached activity capture", async () => {
  const queries: unknown[] = [];
  const pinned = "88".repeat(64);
  const activity = createProjectsActivity({
    source: () =>
      ({
        async activity(query: { operation?: string }) {
          queries.push(query);
          return {
            snapshot: {
              ...snapshot,
              operation_id: query.operation ?? operation,
            },
            atTimeMillis: 123,
            commits: [],
            cursor: null,
          };
        },
      }) as WorkBrowseTransport,
  });
  const repositories = [{ id: repository, name: "Native" }];
  await activity.load(repositories);
  await activity.load(repositories, false, { [repository]: pinned });
  expect(queries).toEqual([
    { refresh: false },
    { refresh: false, operation: pinned },
  ]);
  expect(activity.homes[0]!.page!.snapshot.operation_id).toBe(pinned);
});

test("portable reviews never send Git identities to the native context reader", async () => {
  let reads = 0;
  const value = await readNativeReviewContext(
    source({
      commit: async () => {
        reads++;
        throw new Error("native read");
      },
    }),
    {
      ...reviewThreads[0]!.anchor,
      target: {
        target: "portable_code_artifact",
        value: {
          revision: {
            evidence: {
              repository,
              profile: "git_sha1_jj045",
              operation,
              capsule: "32".repeat(32),
            },
            commit: { profile: "git_sha1_jj045", id: "33".repeat(20) },
            change: null,
          },
          path: "code.txt",
          object: {
            profile: "git_sha1_jj045",
            kind: "blob",
            id: "34".repeat(20),
          },
          value_kind: "file",
        },
      },
    },
  );
  expect(reads).toBe(0);
  expect(value.files).toEqual([]);
  expect(value.codeError).toContain("retained portable-file reader");
});
