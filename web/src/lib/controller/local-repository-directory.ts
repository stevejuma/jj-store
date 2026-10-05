import type { StoreDataSource } from "./native-data-source.js";
import type { WorkBrowseTransport } from "./projects-hub-types.js";
import type { RepositoryDirectorySummary } from "./projects-repository-types.js";
import { assertPinned } from "./production-snapshot.js";
export async function readLocalRepositoryDirectory(
  source: StoreDataSource,
  work: WorkBrowseTransport,
  signal?: AbortSignal,
): Promise<RepositoryDirectorySummary> {
  signal?.throwIfAborted();
  const head = await source.snapshot();
  signal?.throwIfAborted();
  const [tasks, reviews] = await Promise.all([
    work.taskActivity!({ limit: 256 }, signal),
    source.reviewActivity!(signal),
  ]);
  signal?.throwIfAborted();
  const summary =
    head.heads.length === 1
      ? await source.codeSummary!(head.heads[0]!, signal)
      : null;
  if (summary) assertPinned(summary.snapshot, source, head.heads[0]!);
  signal?.throwIfAborted();
  const latest = summary?.data.latest;
  const dates = [
    ...(latest ? [latest.committer.timestamp.millis] : []),
    ...tasks.groups
      .filter((t) => (t.repository ?? source.repository) === source.repository)
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
      ? dates.reduce((max, v) => (v > max ? v : max))
      : null,
    commits: Number(summary?.data.commits ?? 0),
    commitsPartial: summary?.data.commits_partial ?? head.heads.length > 1,
    reviews: Number(reviews.activity.threads),
    reviewsPartial: reviews.activity.threads_partial,
    taskGroups: tasks.groups,
    tasksPartial: Boolean(tasks.next_cursor),
  };
}
