import type {
  RepositoryLogGraph,
  RepositoryLogRevision,
} from "../../web/src/lib/projects/repository-log.js";
import type {
  Bookmark,
  RevisionDetail,
} from "../../web/src/lib/projects/presentation.js";
import type {
  RepositoryChoice,
  RepositorySnapshot,
} from "../../web/src/lib/controller/contracts.js";
import { NOW } from "./repository.js";

const count = 600;
const id = (index: number, width = 128) =>
  index.toString(16).padStart(width, "0");

export const largeRevisions: RepositoryLogRevision[] = Array.from(
  { length: count },
  (_, index) => ({
    eventId: id(count - index),
    commitId: id(count - index),
    changeId: id(count - index, 32),
    parents: index === count - 1 ? [] : [id(count - index - 1)],
    message: `Repository revision ${String(count - index).padStart(3, "0")}`,
    reason: "checkpoint",
    createdAt: NOW - index * 60,
    root: id(0),
    author: "Maya Chen",
    authorEmail: "maya@example.test",
    empty: false,
  }),
);

export const largeBookmarks: Bookmark[] = Array.from(
  { length: count },
  (_, index) => ({
    name:
      index === 0
        ? "main"
        : `feature/virtual-${String(index).padStart(3, "0")}`,
    targets: [largeRevisions[index]!.eventId],
    default: index === 0,
    workspace: index === 0,
    description: `Virtualized bookmark ${index}`,
    author: "Maya Chen",
    authorEmail: "maya@example.test",
    updatedAt: NOW - index * 60,
  }),
);

export const largeGraph: RepositoryLogGraph = {
  revisions: largeRevisions,
  devices: [
    {
      publicKey: "main",
      alias: "main",
      revisionEventId: largeRevisions[0]!.eventId,
      lastSeen: NOW,
    },
  ],
};

export const largeChoices: RepositoryChoice[] = largeBookmarks.map(
  (bookmark) => ({
    value: `bookmark:${bookmark.name}`,
    label: bookmark.name,
    kind: "bookmark",
    targets: bookmark.targets,
  }),
);

export const largeDetail: RevisionDetail = {
  files: Array.from({ length: count }, (_, index) => ({
    path: `file-${String(index).padStart(3, "0")}.rs`,
    additions: index % 7,
    deletions: index % 3,
  })),
  patches: {},
};

export function largeRepositorySnapshot(
  repository: string,
  commit: string,
): RepositorySnapshot {
  return {
    id: commit,
    files: Array.from({ length: count }, (_, index) => ({
      path: `file-${String(index).padStart(3, "0")}.rs`,
      contents: `pub const FILE_${index}: &str = "${repository}";\n`,
    })),
  };
}
