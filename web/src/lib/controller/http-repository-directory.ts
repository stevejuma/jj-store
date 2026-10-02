import {
  ApiError,
  Client,
  type Envelope,
  type Graph,
  type Head,
} from "../api/client.js";
import type { RepositoryDirectorySummary } from "./projects-repository-types.js";
import type { WorkBrowseTransport } from "./projects-hub-types.js";

/** Independent native pins, bounded pages, no head reconciliation or authority writes. */
export async function readRepositoryDirectory(
  client: Client,
  work: Pick<WorkBrowseTransport, "browse" | "reviews">,
  signal?: AbortSignal,
): Promise<RepositoryDirectorySummary> {
  const [head, tasks, reviews] = await Promise.all([
    client.request<Head>("snapshot", {}, { signal }),
    work.browse({ kind: "tasks", limit: 256 }, signal),
    work.reviews!(undefined, signal),
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
  let graph: Envelope<Graph> | null = null;
  if (head.heads.length === 1) {
    graph = await client.request<Envelope<Graph>>(
      "log",
      {
        at_op: head.heads[0],
        revset: "all()",
        limit: 50,
        at_time_millis: Date.now(),
        user_email: "",
      },
      { signal },
    );
    if (
      graph.snapshot.operation_id !== head.heads[0] ||
      graph.snapshot.repo_id !== head.repo_id ||
      graph.snapshot.epoch !== head.epoch
    )
      throw new ApiError("snapshot_mismatch");
  }
  const commits =
    graph?.data.data.graph === "log"
      ? graph.data.data.nodes
          .map((node) => node.commit)
          .filter((commit) => !/^0+$/.test(commit.id))
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
    ...reviews.threads.flatMap((thread) =>
      thread.presentation ? [thread.presentation.updated_at_millis] : [],
    ),
    ...tasks.tasks
      .filter(
        (task) => (task.repository ?? client.repository) === client.repository,
      )
      .map((task) => task.actor.updated_at_millis),
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
      ? dates.reduce((max, value) => (value > max ? value : max))
      : null,
    commits: commits.length,
    commitsPartial: Boolean(graph?.next_cursor),
    threads: reviews.threads,
    reviewsPartial: Boolean(reviews.next_cursor),
    tasks: tasks.tasks,
    tasksPartial: Boolean(tasks.next_cursor),
  };
}
