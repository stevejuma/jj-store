import type { Bookmark } from "../../web/src/lib/projects/presentation.js";
import type { RepositoryLogRevision } from "../../web/src/lib/projects/repository-log.js";
import type {
  StoreMutationCommand,
  StoreMutationPrepared,
} from "../../web/src/lib/controller/store-mutation-types.js";

export interface InMemoryMutationFixtureState {
  bookmarks: Bookmark[];
  revisions: RepositoryLogRevision[];
  operationHeads: string[];
}

export function commitInMemoryMutation(
  current: InMemoryMutationFixtureState,
  command: StoreMutationCommand,
  result: StoreMutationPrepared,
): InMemoryMutationFixtureState {
  let revisions = [...current.revisions];
  const replacement = new Map<string, string>();
  for (const rewrite of result.rewrites)
    for (const predecessor of rewrite.predecessors)
      replacement.set(predecessor, rewrite.commit);

  if (command.type === "new" && result.created[0]) {
    const eventId = result.created[0];
    revisions.unshift({
      eventId,
      commitId: eventId,
      changeId: eventId.slice(0, 32),
      parents: [...command.parents],
      message: command.description,
      reason: "checkpoint",
      createdAt: nextTimestamp(revisions),
      root: revisions.at(-1)?.root ?? eventId,
      author: "Maya Chen",
      authorEmail: "maya@example.test",
      empty: true,
      bookmarks: [],
      workspaces: ["default"],
      conflicted: false,
      divergent: false,
    });
  }
  if (command.type === "describe") {
    const original = revisions.find(
      (revision) => revision.eventId === command.commit,
    );
    const eventId = replacement.get(command.commit);
    if (original && eventId)
      revisions = [
        {
          ...original,
          eventId,
          commitId: eventId,
          message: command.description,
          createdAt: nextTimestamp(revisions),
        },
        ...revisions.filter((revision) => revision !== original),
      ];
  }
  if (command.type === "abandon") {
    const abandoned = new Set(command.commits);
    revisions = revisions.filter(
      (revision) => !abandoned.has(revision.eventId),
    );
  }

  let bookmarks = current.bookmarks
    .map((bookmark) => ({
      ...bookmark,
      targets: bookmark.targets
        .filter((target) => !result.abandoned?.includes(target))
        .map((target) => replacement.get(target) ?? target),
    }))
    .filter((bookmark) => bookmark.targets.length > 0);
  for (const [name, change] of Object.entries(result.bookmarks)) {
    bookmarks = bookmarks.filter((bookmark) => bookmark.name !== name);
    const targets = change.after.filter(
      (target): target is string => target !== null,
    );
    if (targets.length)
      bookmarks.push({
        name,
        targets,
        description:
          revisions.find((revision) => revision.eventId === targets[0])
            ?.message ?? "",
        author: "Maya Chen",
        authorEmail: "maya@example.test",
        updatedAt: nextTimestamp(revisions),
      });
  }
  return {
    bookmarks,
    revisions,
    operationHeads: [result.operation],
  };
}

function nextTimestamp(revisions: readonly RepositoryLogRevision[]) {
  return Math.max(0, ...revisions.map((revision) => revision.createdAt)) + 1;
}
