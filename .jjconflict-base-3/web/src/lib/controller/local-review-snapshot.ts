import { ApiError, type Model } from "../api/client.js";

export function assertLocalReviewSnapshot(
  snapshot: Model<"LocalReviewSnapshot">,
  repository: string,
  pin: string,
) {
  if (
    snapshot.repo_id !== repository ||
    snapshot.metadata_commit !== pin ||
    snapshot.profile !== "local_jj_git"
  )
    throw new ApiError("snapshot_mismatch");
}
