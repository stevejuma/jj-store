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
    source.reviewThreads(undefined, signal),
  ]);
  signal?.throwIfAborted();
  const graph =
    head.heads.length === 1
      ? await source.graph("log", {
          operation: head.heads[0]!,
          limit: 50,
          signal,
        })
      : null;
  if (graph) assertPinned(graph.snapshot, source, head.heads[0]!);
  signal?.throwIfAborted();
  const commits =
    graph?.data.data.graph === "log"
      ? graph.data.data.nodes
          .map((n) => n.commit)
          .filter((c) => !/^0+$/.test(c.id))
      : [];
  const latest = commits
    .slice()
    .sort(
      (a, b) =>
        Number(b.committer.timestamp.millis) -
        Number(a.committer.timestamp.millis),
    )[0];
  const dates = [
    ...(latest ? [latest.committer.timestamp.millis] : []),
    ...tasks.groups
      .filter((t) => (t.repository ?? source.repository) === source.repository)
      .map((group) => group.updated_at_millis),
  ];
  return {
    code: graph?.snapshot ?? null,
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
    commits: commits.length,
    commitsPartial: Boolean(graph?.next_cursor),
    threads: reviews.threads,
    reviewsPartial: Boolean(reviews.next_cursor),
    taskGroups: tasks.groups,
    tasksPartial: Boolean(tasks.next_cursor),
  };
}
