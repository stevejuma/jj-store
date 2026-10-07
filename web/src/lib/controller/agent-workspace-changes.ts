import { ApiError, Client, fullId, type Model } from "../api/client.js";
import { LocalClient } from "../api/local-client.js";
import {
  codeId,
  isLocal,
  sameCodeSnapshot,
  snapshotOperation,
  snapshotRepository,
  type CodeSnapshot,
} from "./repository-profile.js";
import type {
  CodeDiff,
  CodeDiffChange,
  CodeDiffPaths,
} from "./repository-read-types.js";
import type { AgentEvent, AgentSession } from "./agent-execution-types.js";
import { object, text } from "./agent-execution-types.js";
import { presentChangedFiles, presentPatch } from "./native-presentation.js";
import { unavailableDiffMessage } from "../projects/repository-diff.js";

export type AgentChangeSelection = AgentSession["repository_bindings"][number];
export interface AgentChangedFile {
  path: string;
  additions: number | null;
  deletions: number | null;
  patch?: string;
  unavailable?: string;
}
export interface AgentChangeSummary {
  files: AgentChangedFile[];
  complete: boolean;
}

/** Only a confirmed snapshot belonging to the retained completed turn is admitted. */
export function completedAgentChanges(
  session: AgentSession,
  events: readonly AgentEvent[],
) {
  if (["running", "preparing", "opening"].includes(session.state)) return [];
  const record = [...events]
    .reverse()
    .find((e) => object(e.payload).type === "record");
  const request = text(object(record?.payload).requestId);
  if (!request || object(object(record?.payload).record).status !== "completed")
    return [];
  return session.repository_bindings.flatMap((binding) => {
    const event = [...events]
      .reverse()
      .find(
        (e) =>
          object(e.payload).type === "repository_snapshot" &&
          object(e.payload).repository === binding.repository,
      );
    const value = object(event?.payload);
    if (
      value.state !== "completed" ||
      text(value.requestId) !== request ||
      !text(value.operation) ||
      !text(value.commit)
    )
      return [];
    return [
      {
        ...binding,
        operation: text(value.operation),
        commit: text(value.commit),
      },
    ];
  });
}

/** Reads immutable comparisons only; no execution, checkout or operation Undo. */
export async function readAgentWorkspaceChanges(
  selection: AgentChangeSelection,
  fetcher: typeof fetch,
  signal: AbortSignal,
): Promise<AgentChangeSummary> {
  signal.throwIfAborted();
  const native = selection.profile === "native";
  if (!native && selection.profile !== "local_jj_git")
    throw new ApiError("unsupported_agent_repository_profile");
  fullId(selection.repository, 32);
  fullId(selection.operation);
  codeId(selection.base_commit, native ? "native" : "git_sha1_jj045");
  codeId(selection.commit, native ? "native" : "git_sha1_jj045");
  const client = native
    ? new Client(selection.repository, null, () => {}, fetcher)
    : new LocalClient(selection.repository, fetcher);
  let pin: CodeSnapshot | undefined;
  async function query(
    kind: "diff_paths" | "diff",
    path = "",
    cursor?: string,
  ) {
    const page =
      client instanceof Client
        ? await client.request<
            Model<"Window4"> | Model<"Envelope_FileDiffResponse">
          >(
            kind === "diff" ? "diff" : "diff/paths",
            kind === "diff"
              ? {
                  at_op: selection.operation,
                  from: selection.base_commit,
                  to: selection.commit,
                  path,
                  content: true,
                }
              : {
                  at_op: selection.operation,
                  from: selection.base_commit,
                  to: selection.commit,
                  limit: 50,
                  cursor,
                },
            { signal },
          )
        : await client.request<Model<"LocalCodeQueryResponse">>(
            "query",
            {
              operation: selection.operation,
              cursor: cursor ?? null,
              request: {
                kind,
                query:
                  kind === "diff"
                    ? {
                        from: selection.base_commit,
                        to: selection.commit,
                        path,
                        content: true,
                      }
                    : {
                        from: selection.base_commit,
                        to: selection.commit,
                        path: "",
                        limit: 50,
                        anchor: null,
                        direction: "forward",
                      },
              },
            },
            signal,
          );
    signal.throwIfAborted();
    const snapshot = page.snapshot as CodeSnapshot;
    if (
      snapshotRepository(snapshot) !== selection.repository ||
      snapshotOperation(snapshot) !== selection.operation ||
      (native ? "profile" in snapshot : !isLocal(snapshot)) ||
      (pin && !sameCodeSnapshot(pin, snapshot))
    )
      throw new ApiError("snapshot_mismatch");
    pin ??= snapshot;
    const data =
      "kind" in page.data
        ? page.data.kind === kind
          ? page.data.data
          : null
        : page.data;
    if (
      !data ||
      !("changes" in data) ||
      data.from !== selection.base_commit ||
      data.to !== selection.commit ||
      (cursor && cursor === page.next_cursor)
    )
      throw new ApiError("invalid_agent_comparison");
    return { data: data as CodeDiff | CodeDiffPaths, next: page.next_cursor };
  }
  try {
    const paths = new Map<string, CodeDiffChange>();
    let cursor: string | undefined;
    let pages = 0;
    let truncated = false;
    const cursors = new Set<string>();
    do {
      if (cursor && cursors.has(cursor)) throw new ApiError("invalid_cursor");
      if (cursor) cursors.add(cursor);
      const page = await query("diff_paths", "", cursor);
      pages++;
      for (const change of page.data.changes) {
        if (!paths.has(change.path) && paths.size === 100) {
          truncated = true;
          break;
        }
        paths.set(change.path, change);
      }
      cursor = page.next ?? undefined;
      if (pages === 2) break;
    } while (cursor);
    const files = [...paths.values()].map<AgentChangedFile>((c) => ({
      path: c.path,
      additions: null,
      deletions: null,
    }));
    let index = 0;
    let retainedPatchBytes = 0;
    await Promise.all(
      Array.from({ length: Math.min(4, files.length) }, async () => {
        while (index < files.length) {
          const file = files[index++]!;
          const page = await query("diff", file.path);
          const change = page.data.changes.find((c) => c.path === file.path);
          if (!change || page.next)
            throw new ApiError("incomplete_agent_file_comparison");
          if (change.content.status === "text") {
            const stats = presentChangedFiles([change])[0]!;
            file.additions = stats.additions ?? 0;
            file.deletions = stats.deletions ?? 0;
            const patch = presentPatch(change);
            const bytes = new TextEncoder().encode(patch ?? "").byteLength;
            if (bytes + retainedPatchBytes <= 2 * 1024 * 1024) {
              file.patch = patch;
              retainedPatchBytes += bytes;
            } else
              file.unavailable =
                "This file exceeds the saved-change text preview budget. Its line counts remain available.";
          } else
            file.unavailable = unavailableDiffMessage(
              change.content.reason,
            ).description;
        }
      }),
    );
    return { files, complete: !cursor && !truncated };
  } finally {
    client.close();
  }
}
