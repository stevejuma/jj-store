import type { Model } from "../api/client.js";
import type { StoreDataSource } from "./native-data-source.js";
import { assertPinned } from "./production-snapshot.js";
import { readPatch } from "./production-store-loaders.js";

export interface ReviewInlinePatch {
  path: string;
  patch: string;
  lineNumber?: number;
  side?: "before" | "after";
}

function lineNumberAtByte(text: string, byte: number): number | undefined {
  if (!Number.isSafeInteger(byte) || byte < 0) return;
  const encoder = new TextEncoder();
  let offset = 0;
  let line = 1;
  for (const character of text) {
    if (offset === byte) return line;
    offset += encoder.encode(character).byteLength;
    if (offset > byte) return;
    if (character === "\n") line += 1;
  }
  return offset === byte ? line : undefined;
}

export async function readReviewAnchorPatch(
  source: StoreDataSource,
  anchor: Model<"ArtifactAnchorV1">,
): Promise<ReviewInlinePatch | undefined> {
  const target = anchor.target;
  if (target.target !== "code_text" && target.target !== "code_artifact")
    return;
  const { operation_id: operation, commit_id: commit, path } = target.value;
  const context =
    target.target === "code_text" ? target.value.diff_context : null;
  const comparisonPath =
    context?.after_path ?? context?.before_path ?? target.value.path;
  let from = context?.before_commit_id;
  let to = context?.after_commit_id;
  let comparisonOperation = context?.after_operation_id ?? operation;
  if (!from || !to) {
    const detail = await source.commit(commit, operation);
    assertPinned(detail.snapshot, source, operation);
    from = detail.data.commit.parents[0];
    to = commit;
    comparisonOperation = operation;
  }
  if (!from) return;
  const { patch } = await readPatch(
    source,
    comparisonOperation,
    comparisonPath,
    {
      from,
      to,
      beforeOperation: context?.before_operation_id ?? comparisonOperation,
      afterOperation: context?.after_operation_id ?? comparisonOperation,
    },
  );
  if (!patch) return;
  if (target.target !== "code_text") return { path, patch };
  const file = await source.file(
    target.value.file_id,
    target.value.operation_id,
  );
  const start = Number(target.value.text.start_byte);
  const lineNumber =
    file.text === undefined ? undefined : lineNumberAtByte(file.text, start);
  return {
    path: comparisonPath,
    patch,
    ...(lineNumber === undefined ? {} : { lineNumber }),
    ...(context ? { side: context.side } : {}),
  };
}
