import type { StoreDataSource } from "./native-data-source.js";
import type { CodeSnapshot as Snapshot } from "./repository-profile.js";
import type { RepositoryBlob } from "./contracts.js";
import { assertPinned, assertSameSnapshot } from "./production-snapshot.js";
import { presentCommit } from "./native-presentation.js";

export async function readFileRevision(
  source: StoreDataSource,
  operation: string,
  id: string,
  signal?: AbortSignal,
): Promise<import("../projects/repository-log.js").RepositoryCommitSummary> {
  if (source.commitSummary) {
    const value = await source.commitSummary(id, operation, signal);
    assertPinned(value.snapshot, source, operation);
    if (value.data.data.kind !== "commit")
      throw new Error("Expected commit metadata.");
    const commit = value.data.data.data;
    return {
      eventId: id,
      commitId: id,
      changeId: commit.change_id,
      message: commit.description || "No description",
      createdAt: Number(BigInt(commit.committer.timestamp.millis) / 1000n),
      author: commit.author.name || commit.author.email,
      authorEmail: commit.author.email,
    };
  }
  const value = await source.commit(id, operation);
  assertPinned(value.snapshot, source, operation);
  return presentCommit({
    commit: value.data.commit,
    edges: value.data.commit.parents.map((target) => ({
      target,
      kind: "direct" as const,
    })),
  });
}

export async function readPaths(
  source: StoreDataSource,
  operation: string,
  commit: string,
  query: {
    path: string;
    recursive: boolean;
    includeLastChange?: boolean;
    lastChangeMode?: "quick" | "history";
    anchor?: string;
    signal?: AbortSignal;
    limit?: number;
    cursor?: string;
    expected?: Snapshot;
  },
): Promise<{
  entries: import("./repository-read-types.js").CodeEntry[];
  path: string;
  recursive: boolean;
  cursor: string | null;
  readSnapshot: Snapshot;
}> {
  const value = await source.paths(commit, operation, query);
  assertPinned(value.snapshot, source, operation);
  assertSameSnapshot(value.snapshot, query.expected);
  return {
    entries: value.data.entries,
    path: value.data.path,
    recursive: value.data.recursive,
    cursor: value.next_cursor,
    readSnapshot: value.snapshot,
  };
}

export async function readBlob(
  source: StoreDataSource,
  operation: string,
  path: string,
  fileId: string,
  commit: string,
  signal?: AbortSignal,
): Promise<RepositoryBlob> {
  const value = await source.file(fileId, operation, { commit, path, signal });
  return { path, contents: value.text, binary: value.text === undefined };
}
