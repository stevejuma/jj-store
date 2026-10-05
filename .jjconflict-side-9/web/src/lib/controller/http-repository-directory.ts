import {
  ApiError,
  Client,
  type Envelope,
  type Model,
  type Head,
} from "../api/client.js";
import type { RepositoryDirectorySummary } from "./projects-repository-types.js";
import type { WorkBrowseTransport } from "./projects-hub-types.js";

/** Independent native pins, bounded pages, no head reconciliation or authority writes. */
export async function readRepositoryDirectory(
  client: Client,
  work: Pick<WorkBrowseTransport, "taskActivity" | "reviewActivity">,
  signal?: AbortSignal,
): Promise<RepositoryDirectorySummary> {
  const [head, tasks, reviews] = await Promise.all([
    client.request<Head>("snapshot", {}, { signal }),
    work.taskActivity!({ limit: 256 }, signal),
    work.reviewActivity!(signal),
  ]);
  if (
    head.repo_id !== client.repository ||
    tasks.snapshot.repo_id !== client.repository ||
    reviews.snapshot.repo_id !== client.repository ||
    !("epoch" in tasks.snapshot) ||
    tasks.snapshot.epoch !== head.epoch ||
    !("epoch" in reviews.snapshot) ||
    reviews.snapshot.epoch !== head.epoch
  )
    throw new ApiError("snapshot_mismatch");
  let summary: Envelope<Model<"NativeCodeSummary">> | null = null;
  if (head.heads.length === 1) {
    summary = await client.request<Envelope<Model<"NativeCodeSummary">>>(
      "code/summary",
      {
        at_op: head.heads[0],
        limit: 50,
      },
      { signal },
    );
    if (
      summary.snapshot.operation_id !== head.heads[0] ||
      summary.snapshot.repo_id !== head.repo_id ||
      summary.snapshot.epoch !== head.epoch
    )
      throw new ApiError("snapshot_mismatch");
  }
  const latest = summary?.data.latest;
  const dates = [
    ...(latest ? [latest.committer.timestamp.millis] : []),
    ...(reviews.activity.updated_at_millis !== null
      ? [reviews.activity.updated_at_millis]
      : []),
    ...tasks.groups
      .filter(
        (task) => (task.repository ?? client.repository) === client.repository,
      )
      .map((group) => group.updated_at_millis),
  ];
  return {
    code: summary?.snapshot ?? null,
    work: tasks.snapshot,
    review: reviews.snapshot,
    description:
      latest?.description.trim().split("\n")[0] ||
      (head.heads.length > 1
        ? "Multiple operation heads"
        : "No change description"),
    contributor: latest?.author ?? null,
    updatedAtMillis: dates.length
      ? dates.reduce((max, value) => (value > max ? value : max))
      : null,
    commits: Number(summary?.data.commits ?? 0),
    commitsPartial: summary?.data.commits_partial ?? head.heads.length > 1,
    reviews: Number(reviews.activity.threads),
    reviewsPartial: reviews.activity.threads_partial,
    taskGroups: tasks.groups,
    tasksPartial: Boolean(tasks.next_cursor),
  };
}
