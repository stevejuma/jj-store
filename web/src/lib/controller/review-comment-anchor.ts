import type { FileDiffLineContext } from "@lapismd/design-core/diff";
import type { Model } from "../api/client.js";
import type { StoreDataSource } from "./native-data-source.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import { assertPinned } from "./production-snapshot.js";

type NativeTreeValue = import("./repository-read-types.js").CodeTreeValue;
type FileValue = Extract<NativeTreeValue, { type: "file" }>;

interface SelectedFile {
  file: FileValue;
  path: string;
}

function selectedFile(
  terms: readonly (NativeTreeValue | null)[],
  path: string,
): SelectedFile | undefined {
  if (terms.length !== 1 || terms[0]?.type !== "file") return;
  return { file: terms[0], path };
}

function beforeFile(
  change: import("./repository-read-types.js").CodeDiffChange,
): SelectedFile | undefined {
  const direct = selectedFile(change.before, change.path);
  if (direct) return direct;
  const sources = change.sources
    .map((source) => selectedFile(source.terms, source.path))
    .filter((value): value is SelectedFile => value !== undefined);
  return sources.length === 1 ? sources[0] : undefined;
}

function lineSelection(text: string, context: FileDiffLineContext) {
  if (context.lineNumber === null || context.lineNumber < 1) return;
  let line = 1;
  let start = 0;
  while (line < context.lineNumber) {
    const newline = text.indexOf("\n", start);
    if (newline < 0) return;
    start = newline + 1;
    line += 1;
  }
  const newline = text.indexOf("\n", start);
  const contentEnd = newline < 0 ? text.length : newline;
  const visibleEnd =
    contentEnd > start && text[contentEnd - 1] === "\r"
      ? contentEnd - 1
      : contentEnd;
  const visible = text.slice(start, visibleEnd);
  if (visible !== context.text) return;
  const selectedEnd = visible.length > 0 ? visibleEnd : newline + 1;
  if (selectedEnd <= start) return;
  const encoder = new TextEncoder();
  const startByte = encoder.encode(text.slice(0, start)).byteLength;
  const bytes = encoder.encode(text.slice(start, selectedEnd));
  return { startByte, endByte: startByte + bytes.byteLength, bytes };
}

async function sha256(bytes: Uint8Array): Promise<string> {
  const input = new Uint8Array(bytes.byteLength);
  input.set(bytes);
  const digest = new Uint8Array(
    await crypto.subtle.digest("SHA-256", input.buffer),
  );
  return [...digest]
    .map((value) => value.toString(16).padStart(2, "0"))
    .join("");
}

export async function createCodeLineReviewAnchor(
  source: StoreDataSource,
  data: ProductionStoreState,
  context: FileDiffLineContext,
): Promise<Model<"ArtifactAnchorV1"> | undefined> {
  const comparison = data.comparisons.get(context.path);
  if (!comparison || context.lineNumber === null) return;
  if (source.profile === "git_sha1_jj045")
    throw new Error("Local reviews are not supported.");
  const response = await source.diff(
    comparison.afterOperation,
    comparison.from,
    comparison.to,
    context.path,
  );
  assertPinned(response.snapshot, source, comparison.afterOperation);
  const change = response.data.changes.find(
    (candidate) => candidate.path === context.path,
  );
  if (!change || change.content.status !== "text") return;
  const side = context.variant === "removed" ? "before" : "after";
  const before = beforeFile(change);
  const after = selectedFile(change.after, change.path);
  const selected = side === "before" ? before : after;
  if (!selected) return;
  const operationId =
    side === "before" ? comparison.beforeOperation : comparison.afterOperation;
  const commitId = side === "before" ? comparison.from : comparison.to;
  const file = await source.file(selected.file.id, operationId);
  if (file.text === undefined) return;
  const selection = lineSelection(file.text, context);
  if (!selection) return;
  return {
    schema_version: 1,
    repository_id: source.repository,
    target: {
      target: "code_text",
      value: {
        operation_id: operationId,
        commit_id: commitId,
        change_id: data.detail
          ? (data.graph.revisions.find(
              (revision) => revision.eventId === commitId,
            )?.changeId ?? null)
          : null,
        path: selected.path,
        file_id: selected.file.id,
        value_kind: "file",
        text: {
          encoding: "utf8",
          start_byte: String(selection.startByte),
          end_byte: String(selection.endByte),
          selected_sha256: await sha256(selection.bytes),
        },
        diff_context: {
          before_operation_id: comparison.beforeOperation,
          before_commit_id: comparison.from,
          before_path: before?.path ?? null,
          after_operation_id: comparison.afterOperation,
          after_commit_id: comparison.to,
          after_path: after?.path ?? null,
          side,
          comparison: "explicit_pair",
        },
      },
    },
    created_context: {
      publication_position:
        "published_seq" in response.snapshot
          ? response.snapshot.published_seq
          : (() => {
              throw new Error("Local reviews are not supported.");
            })(),
      code_state_version:
        "code_state_version" in response.snapshot
          ? response.snapshot.code_state_version
          : (() => {
              throw new Error("Local reviews are not supported.");
            })(),
      agent_root: data.agentSession?.snapshot.agent_root ?? null,
    },
    sensitivity: "repository",
  };
}

export function createAgentToolReviewAnchor(
  source: StoreDataSource,
  data: ProductionStoreState,
  toolCallId: string,
): Model<"ArtifactAnchorV1"> | undefined {
  const detail = data.agentSession;
  if (!detail) return;
  const reference = detail.tools.find(
    (candidate) => candidate.tool_call_id === toolCallId,
  );
  const activity = detail.activity.find(
    (candidate) =>
      candidate.type === "tool" && candidate.tool.tool_call_id === toolCallId,
  );
  const snapshotId =
    reference?.snapshot_id ??
    (activity?.type === "tool" ? activity.tool.snapshot_id : undefined);
  if (!snapshotId) return;
  return {
    schema_version: 1,
    repository_id: source.repository,
    target: {
      target: "agent_artifact",
      value: {
        session_id: detail.session.session_id,
        session_snapshot: {
          kind: "session_snapshot",
          id: detail.session.snapshot_id,
        },
        owner: {
          kind: "tool",
          tool_call_id: toolCallId,
          snapshot: { kind: "tool_snapshot", id: snapshotId },
          attempt_id: toolCallId,
        },
      },
    },
    created_context: {
      publication_position:
        "published_seq" in detail.snapshot
          ? detail.snapshot.published_seq
          : (() => {
              throw new Error("Local reviews are not supported.");
            })(),
      code_state_version:
        data.head && "code_state_version" in data.head
          ? data.head.code_state_version
          : null,
      agent_root: detail.snapshot.agent_root,
    },
    sensitivity: "repository",
  };
}
