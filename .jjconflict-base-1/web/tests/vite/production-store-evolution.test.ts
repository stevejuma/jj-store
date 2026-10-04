import { expect, test, vi } from "vitest";
import type {
  Envelope,
  Graph,
  Model,
  Snapshot,
} from "../../src/lib/api/client.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
import { createProductionEvolutionReader } from "../../src/lib/controller/production-evolution-reader.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import { createStoreRouteHistory } from "../../src/lib/controller/store-route-history.svelte.js";

const repository = "ab".repeat(16),
  operation = "cd".repeat(64),
  commit = "12".repeat(64),
  predecessor = "34".repeat(64),
  commitParent = "56".repeat(64);
const snapshot: Snapshot = {
  version: 1,
  repo_id: repository,
  epoch: "epoch",
  published_seq: 1,
  manifest_revision: 1,
  operation_id: operation,
  heads: [operation],
  code_state_version: 1,
};
function node(
  id = commit,
  predecessors = [predecessor],
): Model<"GraphEvolutionNode"> {
  const author = {
    name: "Maya",
    email: "maya@example.invalid",
    timestamp: { millis: 1000, offset_minutes: 0 },
  };
  return {
    commit: {
      id,
      change_id: "78".repeat(16),
      description: "Native rewrite",
      parents: [commitParent],
      author,
      committer: author,
      tree_terms: [],
      conflicted: false,
      bookmarks: [],
      workspaces: [],
      visible: true,
      divergent: false,
    },
    operation,
    predecessors,
    predecessors_known: true,
    legacy_predecessors: [],
  };
}
function page(nodes = [node()], cursor: string | null = null): Envelope<Graph> {
  return {
    snapshot: { ...snapshot },
    next_cursor: cursor,
    data: {
      version: 1,
      operation,
      data: { graph: "evolution", nodes },
      next: null,
      cache: {
        state: "built",
        evicted: 0,
        retained_entries: 0,
        retained_bytes: 0,
      },
      stats: {
        visited_objects: nodes.length,
        visited_bytes: 1,
        elapsed_millis: 1,
        cache_hits: 0,
      },
    },
  };
}
function harness() {
  const data = createProductionStoreState(),
    history = createStoreRouteHistory(undefined);
  history.connect(repository, operation);
  history.replace({
    ...history.route,
    page: "log",
    commitId: commit,
    evolutionCommitId: commit,
  });
  const graph = vi.fn<StoreDataSource["graph"]>();
  const source = { repository, graph } as unknown as StoreDataSource;
  const reader = createProductionEvolutionReader({
    data,
    history,
    source: () => source,
    operation: () => history.route.snapshotOperationId,
    version: () => 1,
    current: (ticket, active) => ticket === 1 && active === source,
  });
  return { data, history, graph, reader };
}

test("evolution reads full pinned identity and preserves predecessor edges instead of commit parents", async () => {
  const h = harness();
  h.graph.mockResolvedValue(page());
  await h.reader.loadEvolution();
  expect(h.graph).toHaveBeenCalledWith("evolution", {
    operation,
    commit,
    cursor: undefined,
    limit: 50,
  });
  expect(h.data.evolution.status).toBe("ready");
  expect(h.data.evolution.graph.revisions[0]?.parents).toEqual([predecessor]);
  expect(h.data.evolution.graph.devices).toEqual([]);
  expect(h.history.route.commitId).toBe(commit);
});

test("opaque pages deduplicate commits, keep exact snapshot and retain incomplete-predecessor state", async () => {
  const h = harness();
  const unknown = { ...node(), predecessors_known: false };
  h.graph
    .mockResolvedValueOnce(page([unknown], "page-2"))
    .mockResolvedValueOnce(page([node(), node(predecessor, [])]));
  await h.reader.loadEvolution();
  expect(h.data.evolution.canLoadMore).toBe(true);
  await h.reader.loadEvolution(true);
  expect(h.graph.mock.calls[1]?.[1].cursor).toBe("page-2");
  expect(
    h.data.evolution.graph.revisions.map((revision) => revision.eventId),
  ).toEqual([commit, predecessor]);
  expect(h.data.evolution.predecessorsUnknown).toBe(true);
  expect(h.data.evolution.canLoadMore).toBe(false);
});

for (const fault of ["snapshot", "cursor", "operation"] as const) {
  test(`evolution rejects a ${fault} mismatch without replacing loaded history`, async () => {
    const h = harness();
    const next = page([node(predecessor, [])]);
    if (fault === "snapshot") next.snapshot.epoch = "other";
    if (fault === "operation") next.snapshot.operation_id = "ef".repeat(64);
    if (fault === "cursor") next.next_cursor = "page-2";
    h.graph
      .mockResolvedValueOnce(page([node()], "page-2"))
      .mockResolvedValueOnce(next);
    await h.reader.loadEvolution();
    await h.reader.loadEvolution(true);
    expect(h.data.evolution.status).toBe("failed");
    expect(h.data.evolution.error).not.toBe("");
    expect(
      h.data.evolution.graph.revisions.map((revision) => revision.eventId),
    ).toEqual([commit]);
  });
}

test("closed columns and newer selections ignore late evolution responses", async () => {
  const h = harness();
  let deliver!: (value: Envelope<Graph>) => void;
  h.graph.mockReturnValueOnce(
    new Promise((resolve) => {
      deliver = resolve;
    }),
  );
  const pending = h.reader.loadEvolution();
  h.history.replace({ ...h.history.route, evolutionCommitId: null });
  deliver(page());
  await pending;
  expect(h.data.evolution.graph.revisions).toHaveLength(0);
  h.history.replace({ ...h.history.route, evolutionCommitId: commit });
  h.graph
    .mockReturnValueOnce(
      new Promise((resolve) => {
        deliver = resolve;
      }),
    )
    .mockResolvedValueOnce(page([node(predecessor, [])]));
  const old = h.reader.loadEvolution();
  h.history.replace({ ...h.history.route, evolutionCommitId: predecessor });
  await h.reader.loadEvolution();
  deliver(page());
  await old;
  expect(
    h.data.evolution.graph.revisions.map((revision) => revision.eventId),
  ).toEqual([predecessor]);
});

test("empty evolution history is explicit and displayed history stays bounded", async () => {
  const h = harness();
  h.graph.mockResolvedValueOnce(page([]));
  await h.reader.loadEvolution();
  expect(h.data.evolution.status).toBe("empty");
  const nodes = Array.from({ length: 1000 }, (_, i) =>
    node(i.toString(16).padStart(128, "0"), []),
  );
  h.graph
    .mockResolvedValueOnce(page(nodes, "overflow"))
    .mockResolvedValueOnce(page([node(predecessor, [])]));
  await h.reader.loadEvolution();
  await h.reader.loadEvolution(true);
  expect(h.data.evolution.status).toBe("failed");
  expect(h.data.evolution.graph.revisions).toHaveLength(1000);
});
