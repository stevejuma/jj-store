export type RepositoryLogReason = "checkpoint" | "merge";

export type RepositoryLogBookmark = {
  name: string;
  remote?: string;
  added: boolean;
  removed: boolean;
  conflicted: boolean;
};

export type RepositoryLogRevision = {
  eventId: string;
  changeId: string;
  commitId?: string;
  parents: string[];
  message: string;
  reason: RepositoryLogReason;
  createdAt: number;
  committedAtMs?: number;
  root: string;
  author: string;
  authorEmail?: string;
  authorAvatarUrl?: string;
  empty: boolean;
  bookmarks: RepositoryLogBookmark[];
  workspaces: string[];
  conflicted: boolean;
  divergent: boolean;
  operationSnapshot?: boolean;
};

/** Immutable commit fields used without loading graph decorations. */
export type RepositoryCommitSummary = Pick<
  RepositoryLogRevision,
  | "eventId"
  | "commitId"
  | "changeId"
  | "message"
  | "createdAt"
  | "committedAtMs"
  | "author"
  | "authorEmail"
>;

export type RepositoryWorkingCopy = {
  workspace: string;
  commitId: string;
  changeId: string;
  message: string;
  createdAt: number;
  author: string;
  authorEmail?: string;
};

export type RepositoryLogDevice = {
  publicKey: string;
  alias: string;
  revisionEventId: string;
  lastSeen: number;
};

export type RepositoryLogGraph = {
  revisions: RepositoryLogRevision[];
  devices: RepositoryLogDevice[];
};

export function orderNewestFirst(
  revisions: RepositoryLogRevision[],
): RepositoryLogRevision[] {
  const byId = new Map(
    revisions.map((revision) => [revision.eventId, revision]),
  );
  const remaining = new Set(byId.keys());
  const childCount = new Map<string, number>();
  for (const revision of revisions) {
    for (const parent of revision.parents) {
      if (byId.has(parent)) {
        childCount.set(parent, (childCount.get(parent) ?? 0) + 1);
      }
    }
  }
  const tips = revisions.filter((revision) =>
    revisions.every((other) => !other.parents.includes(revision.eventId)),
  );
  const queue = sortRevisions(tips);
  const ordered: RepositoryLogRevision[] = [];
  while (queue.length > 0) {
    const next = queue.shift()!;
    if (!remaining.delete(next.eventId)) continue;
    ordered.push(next);
    for (const parentId of next.parents) {
      const parent = byId.get(parentId);
      if (!parent || !remaining.has(parentId)) continue;
      const left = (childCount.get(parentId) ?? 1) - 1;
      childCount.set(parentId, left);
      if (left <= 0) queue.push(parent);
    }
    queue.sort(compareRevisions);
  }
  for (const revision of sortRevisions(revisions)) {
    if (remaining.delete(revision.eventId)) ordered.push(revision);
  }
  return ordered;
}

function sortRevisions(
  revisions: RepositoryLogRevision[],
): RepositoryLogRevision[] {
  return revisions.slice().sort(compareRevisions);
}

function compareRevisions(
  left: RepositoryLogRevision,
  right: RepositoryLogRevision,
): number {
  return (
    right.createdAt - left.createdAt ||
    left.eventId.localeCompare(right.eventId)
  );
}
