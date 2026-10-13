import { untrack } from "svelte";
import type { StoreDataSource } from "../../controller/native-data-source.js";
import {
  isLocal,
  snapshotRepository,
  snapshotOperation,
} from "../../controller/repository-profile.js";
import type { CodeSnapshot } from "../../controller/repository-profile.js";

export type WorkflowSourceChoice = {
  id: string;
  name: string;
  description: string;
  disabled?: boolean;
  commit?: string;
};
const empty = () => ({
  rows: [] as WorkflowSourceChoice[],
  cursor: null as string | null,
  loading: false,
  error: "",
});

/** Native revision choices retain the exact source operation across pages. */
export function createWorkflowRunSource(
  source: () => StoreDataSource | undefined,
  selected: () => string,
) {
  const operations = $state(empty()),
    commits = $state(empty()),
    bookmarks = $state(empty());
  let sourceGeneration = 0,
    revisionGeneration = 0,
    anchor = "",
    heads: string[] = [],
    atTimeMillis = 0;
  let revisionAbort = new AbortController();
  const merge = (rows: WorkflowSourceChoice[]) => [
    ...new Map(rows.map((row) => [row.id, row])).values(),
  ];
  function verify(
    snapshot: CodeSnapshot,
    repository: string,
    operation: string,
  ) {
    if (
      isLocal(snapshot) ||
      snapshotRepository(snapshot) !== repository ||
      snapshotOperation(snapshot) !== operation
    )
      throw Error("Source snapshot changed. Retry to reload revision choices.");
  }
  async function loadOperations(more = false) {
    const reader = source(),
      ticket = sourceGeneration;
    if (!reader || operations.loading || (more && !operations.cursor)) return;
    const current = () => ticket === sourceGeneration && reader === source();
    operations.loading = true;
    try {
      if (!more) {
        const head = await reader.snapshot(true);
        if (!current()) return;
        if (isLocal(head) || snapshotRepository(head) !== reader.repository)
          throw Error("Native source repository unavailable.");
        heads = head.heads;
        anchor = heads[0] ?? "";
        if (!anchor) throw Error("No published source operation is available.");
      }
      const page = await reader.graph("operations", {
        operation: anchor,
        limit: 50,
        cursor: more ? (operations.cursor ?? undefined) : undefined,
      });
      if (!current()) return;
      verify(page.snapshot, reader.repository, anchor);
      if (page.data.data.graph !== "operations")
        throw Error("Operation choices unavailable.");
      operations.rows = merge([
        ...(more
          ? operations.rows
          : heads.map((id) => ({
              id,
              name: "Published operation",
              description: id.slice(0, 12),
            }))),
        ...page.data.data.nodes.map((op) => ({
          id: op.id,
          name: op.description || "Recorded operation",
          description: `${op.id.slice(0, 12)} · ${op.user}`,
        })),
      ]);
      operations.cursor = page.next_cursor;
      operations.error = "";
    } catch (cause) {
      if (current())
        operations.error =
          cause instanceof Error
            ? cause.message
            : "Operation choices unavailable.";
    } finally {
      if (current()) operations.loading = false;
    }
  }
  async function loadRevisions(kind: "commits" | "bookmarks", more = false) {
    const reader = source(),
      operation = selected(),
      ticket = revisionGeneration,
      state = kind === "commits" ? commits : bookmarks;
    if (!reader || !operation || state.loading || (more && !state.cursor))
      return;
    const current = () =>
      ticket === revisionGeneration &&
      reader === source() &&
      operation === selected();
    state.loading = true;
    try {
      let rows: WorkflowSourceChoice[], cursor: string | null;
      if (kind === "commits") {
        const page = await reader.graph("log", {
          operation,
          limit: 50,
          atTimeMillis,
          cursor: more ? (state.cursor ?? undefined) : undefined,
          signal: revisionAbort.signal,
        });
        if (!current()) return;
        verify(page.snapshot, reader.repository, operation);
        if (page.data.data.graph !== "log")
          throw Error("Commit choices unavailable.");
        rows = page.data.data.nodes.map(({ commit }) => ({
          id: commit.id,
          name: commit.description.split("\n")[0] || "No description",
          description: `${commit.id.slice(0, 12)} · ${commit.author.name}`,
        }));
        cursor = page.next_cursor;
      } else {
        const page = await reader.references(
          operation,
          more ? (state.cursor ?? undefined) : undefined,
          { kind: "bookmarks", signal: revisionAbort.signal },
        );
        if (!current()) return;
        verify(page.snapshot, reader.repository, operation);
        rows = page.data.references.flatMap((ref) =>
          ref.kind === "bookmark"
            ? [
                {
                  id: ref.name,
                  name: ref.name,
                  description: ref.conflicted
                    ? "Conflicted bookmark"
                    : ref.targets[0]?.description.split("\n")[0] || "No target",
                  disabled: ref.conflicted || ref.targets.length !== 1,
                  commit:
                    !ref.conflicted && ref.targets.length === 1
                      ? ref.targets[0]!.commit
                      : undefined,
                },
              ]
            : [],
        );
        cursor = page.next_cursor;
      }
      state.rows = merge([...(more ? state.rows : []), ...rows]);
      state.cursor = cursor;
      state.error = "";
    } catch (cause) {
      if (current())
        state.error =
          cause instanceof Error
            ? cause.message
            : "Source choices unavailable.";
    } finally {
      if (current()) state.loading = false;
    }
  }
  function resetRevisions() {
    revisionGeneration++;
    revisionAbort.abort();
    revisionAbort = new AbortController();
    Object.assign(commits, empty());
    Object.assign(bookmarks, empty());
    atTimeMillis = Date.now();
  }
  $effect(() => {
    const reader = source();
    untrack(() => {
      sourceGeneration++;
      Object.assign(operations, empty());
      anchor = "";
      heads = [];
      if (reader) void loadOperations();
    });
    return () => {
      sourceGeneration++;
    };
  });
  $effect(() => {
    const reader = source(),
      operation = selected();
    untrack(() => {
      resetRevisions();
      if (reader && operation) {
        void loadRevisions("commits");
        void loadRevisions("bookmarks");
      }
    });
    return () => revisionAbort.abort();
  });
  return {
    operations,
    commits,
    bookmarks,
    loadOperations,
    loadRevisions,
    resetRevisions,
  };
}
