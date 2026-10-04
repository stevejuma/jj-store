import type { LocalClient } from "../api/local-client.js";
import type { Model } from "../api/client.js";
import { assertLocalReviewSnapshot } from "./local-review-snapshot.js";

async function selection(
  metadata: () => Promise<string>,
  signal?: AbortSignal,
) {
  signal?.throwIfAborted();
  const pin = await metadata();
  signal?.throwIfAborted();
  return pin;
}
export async function readLocalReviewActivity(
  repository: string,
  metadata: () => Promise<string>,
  read: LocalClient["request"],
  signal?: AbortSignal,
) {
  const pin = await selection(metadata, signal);
  const summary = await read<Model<"LocalReviewActivitySummary">>(
    "reviews/activity",
    { metadata_commit: pin, limit: 50 },
    signal,
  );
  assertLocalReviewSnapshot(summary.snapshot, repository, pin);
  return summary;
}
export async function readLocalReviewPage(
  repository: string,
  metadata: () => Promise<string>,
  read: LocalClient["request"],
  cursor?: string,
  signal?: AbortSignal,
) {
  const pin = await selection(metadata, signal);
  const page = await read<Model<"LocalReviewPageResponse">>(
    "reviews/page",
    { metadata_commit: pin, limit: 50, cursor: cursor ?? null },
    signal,
  );
  assertLocalReviewSnapshot(page.data.snapshot, repository, pin);
  return { ...page.data, next_cursor: page.next_cursor };
}
