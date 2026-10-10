import { assertPinned } from "../controller/production-snapshot.js";
import type { StoreDataSource } from "../controller/native-data-source.js";
import type { WorkspaceRead } from "./workspace-browser.svelte.js";
import type { Model } from "../api/client.js";

/** Compare exact repository commits without creating or reading an agent checkout. */
export async function repositoryComparison(
  source: StoreDataSource,
  operation: string,
  commit: string,
  path: string | null,
  cursor: string | null,
  range: Model<"ComparisonRange">,
  base: string | null,
  signal?: AbortSignal,
): Promise<
  Pick<
    WorkspaceRead,
    | "selection"
    | "references"
    | "comparison"
    | "incomplete"
    | "files"
    | "next"
    | "file"
  >
> {
  const refs = await source.references(operation, undefined, {
    kind: "bookmarks",
    signal,
  });
  assertPinned(refs.snapshot, source, operation);
  const references = refs.data.references.flatMap((r) =>
    r.kind === "bookmark" && !r.conflicted && r.terms.length === 1 && r.terms[0]
      ? [{ name: r.name, commit: r.terms[0] }]
      : [],
  );
  let unavailable: string | null = null;
  if (range === "last_turn")
    unavailable = "No agent turn exists for this repository snapshot.";
  else if (range === "branch" && !base)
    unavailable = "Choose a comparison base. No default branch is configured.";
  else if (range === "since_last_commit") {
    const detail = await source.commit(commit, operation);
    assertPinned(detail.snapshot, source, operation);
    base =
      detail.data.commit.parents.length === 1
        ? detail.data.commit.parents[0]!
        : null;
    if (!base)
      unavailable = "Choose an explicit base for a root or merge commit.";
  }
  const result = {
    selection: `repository:${source.repository}:${operation}:${base ?? ""}:${commit}:${range}`,
    references,
    comparison: {
      range,
      before_operation: base ? operation : null,
      before_commit: base,
      after_operation: operation,
      after_commit: commit,
      current_files: false,
      request_id: null,
      unavailable,
    },
    incomplete: refs.next_cursor
      ? "Only the loaded bookmark page is available."
      : null,
    files: [] as WorkspaceRead["files"],
    next: null as string | null,
    file: null as WorkspaceRead["file"],
  };
  if (unavailable || !base) return result;
  const page = path
    ? await source.diff(operation, base, commit, path, signal)
    : await source.diffPaths(
        operation,
        base,
        commit,
        cursor ?? undefined,
        signal,
      );
  assertPinned(page.snapshot, source, operation);
  if (!path) {
    result.files = page.data.changes.map((c) => ({
      path: c.path,
      kind: "file",
      bytes: 0,
      fingerprint: result.selection,
      status: c.before.every((t) => !t)
        ? "added"
        : c.after.every((t) => !t)
          ? "deleted"
          : "modified",
    }));
    result.next = page.next_cursor;
    return result;
  }
  const change = page.data.changes.find((c) => c.path === path);
  if (!change) throw new Error("This path is not in the selected comparison.");
  const read = async (terms: typeof change.before, revision: string) => {
    if (terms.every((t) => !t)) return "";
    const term = terms.length === 1 ? terms[0] : null;
    if (!term || term.type !== "file") return null;
    const value = await source.file(term.id, operation, {
      commit: revision,
      path,
      signal,
    });
    return value.text ?? null;
  };
  const [before, after] = await Promise.all([
    read(change.before, base),
    read(change.after, commit),
  ]);
  result.file = {
    path,
    fingerprint: result.selection,
    before,
    contents: after,
    unavailable:
      before === null || after === null
        ? "This comparison contains unsupported or unavailable text."
        : null,
  };
  return result;
}
