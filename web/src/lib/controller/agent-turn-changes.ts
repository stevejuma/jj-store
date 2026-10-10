import { ApiError, Client, type Model } from "../api/client.js";
import { LocalClient } from "../api/local-client.js";
import { object, text } from "./agent-execution-types.js";
import {
  snapshotOperation,
  snapshotRepository,
  type CodeSnapshot,
} from "./repository-profile.js";
import type {
  AgentChangeSelection,
  AgentChangeSummary,
  AgentTurnSelection,
} from "./agent-workspace-changes.js";
import type { WorkspaceRead } from "../conversations/workspace-browser.svelte.js";

/** A revision identity is read once at the same pin as the saved comparison. */
export async function readTurnRevisionId(
  selection: AgentChangeSelection,
  fetcher: typeof fetch,
  signal: AbortSignal,
) {
  const native = selection.profile === "native";
  const client = native
    ? new Client(selection.repository, null, () => {}, fetcher)
    : new LocalClient(selection.repository, fetcher);
  try {
    const page =
      client instanceof Client
        ? await client.request<Model<"Envelope_ContentObject">>(
            `objects/commit/${selection.commit}`,
            { at_op: selection.operation },
            { signal },
          )
        : await client.request<Model<"LocalCodeQueryResponse">>(
            "query",
            {
              operation: selection.operation,
              cursor: null,
              request: {
                kind: "commit_summary",
                query: { commit: selection.commit },
              },
            },
            signal,
          );
    const snapshot = page.snapshot as CodeSnapshot;
    if (
      snapshotRepository(snapshot) !== selection.repository ||
      snapshotOperation(snapshot) !== selection.operation ||
      native === "profile" in snapshot
    )
      throw new ApiError("snapshot_mismatch");
    const wrapper = native ? object(page.data) : object(object(page.data).data);
    if (
      object(wrapper.key).kind !== "commit" ||
      text(object(wrapper.key).id) !== selection.commit
    )
      throw new ApiError("invalid_commit_summary");
    const container = object(wrapper.data);
    if (container.kind !== "commit")
      throw new ApiError("invalid_commit_summary");
    const commit = object(container.data);
    if (commit.id && text(commit.id) !== selection.commit)
      throw new ApiError("invalid_commit_summary");
    return text(commit.change_id);
  } finally {
    client.close();
  }
}

/** Project saved immutable bytes into the shared rail; never observe today's checkout. */
export function savedTurnInspection(
  selection: AgentTurnSelection,
  summary: AgentChangeSummary,
  workspace: string,
  path: string | null = null,
): WorkspaceRead {
  const id = `turn:${selection.session}:${selection.repository}:${selection.beforeOperation}:${selection.base_commit}:${selection.operation}:${selection.commit}`;
  const file = path ? summary.files.find((f) => f.path === path) : undefined;
  if (path && !file) throw new Error("This file is not in the selected turn.");
  return {
    version: 1,
    session: selection.session,
    repository: selection.repository,
    workspace,
    location: "",
    operation: selection.operation,
    commit: selection.commit,
    selection: id,
    references: [],
    comparison: {
      range: "last_turn",
      before_operation: selection.beforeOperation,
      before_commit: selection.base_commit,
      after_operation: selection.operation,
      after_commit: selection.commit,
      current_files: false,
      request_id: selection.requestId,
      unavailable: null,
    },
    incomplete: summary.complete
      ? null
      : "Additional files are outside this bounded saved-turn preview.",
    files: summary.files.map((f) => ({
      path: f.path,
      kind: "file",
      bytes: 0,
      fingerprint: id,
      status: null,
      additions: f.additions,
      deletions: f.deletions,
    })),
    next: null,
    file: file
      ? {
          path: file.path,
          fingerprint: id,
          before: file.before ?? null,
          contents: file.after ?? null,
          unavailable:
            file.unavailable ??
            (file.before === undefined || file.after === undefined
              ? "Saved text is unavailable."
              : null),
        }
      : null,
  };
}
