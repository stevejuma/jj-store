import { ApiError } from "../api/client.js";
import {
  sameCodeSnapshot,
  snapshotRepository,
  snapshotOperation,
  type CodeSnapshot as Snapshot,
} from "./repository-profile.js";
import type { StoreDataSource } from "./native-data-source.js";
export function assertPinned(
  value: Snapshot,
  source: StoreDataSource,
  operation: string,
) {
  if (
    snapshotRepository(value) !== source.repository ||
    snapshotOperation(value) !== operation ||
    (source.profile === "git_sha1_jj045") !== "profile" in value
  )
    throw new ApiError("snapshot_mismatch");
}
export function assertSameSnapshot(value: Snapshot, expected?: Snapshot) {
  if (expected && !sameCodeSnapshot(value, expected))
    throw new ApiError("pagination_snapshot_mismatch");
}
