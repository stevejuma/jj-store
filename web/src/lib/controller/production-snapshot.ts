import { ApiError, type Snapshot } from "../api/client.js";
import type { StoreDataSource } from "./native-data-source.js";

export function assertPinned(
  value: Snapshot,
  source: StoreDataSource,
  operation: string,
) {
  if (value.repo_id !== source.repository || value.operation_id !== operation)
    throw new ApiError("snapshot_mismatch");
}

export function assertSameSnapshot(value: Snapshot, expected?: Snapshot) {
  if (!expected) return;
  const same =
    value.repo_id === expected.repo_id &&
    value.epoch === expected.epoch &&
    String(value.published_seq) === String(expected.published_seq) &&
    String(value.manifest_revision) === String(expected.manifest_revision) &&
    value.operation_id === expected.operation_id &&
    value.heads.length === expected.heads.length &&
    value.heads.every((head, index) => head === expected.heads[index]);
  if (!same) throw new ApiError("pagination_snapshot_mismatch");
}
