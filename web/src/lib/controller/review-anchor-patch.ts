import type { Model } from "../api/client.js";
import type { StoreDataSource } from "./native-data-source.js";
import { assertPinned } from "./production-snapshot.js";
import { readPatch } from "./production-store-loaders.js";

export interface ReviewInlinePatch {
  path: string;
  patch: string;
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
  const patch = await readPatch(source, comparisonOperation, path, {
    from,
    to,
  });
  return patch ? { path, patch } : undefined;
}
