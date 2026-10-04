import type {
  Bookmark,
  RevisionDetail,
} from "../../web/src/lib/projects/presentation.js";
import type {
  RepositoryLogGraph,
  RepositoryLogRevision,
} from "../../web/src/lib/projects/repository-log.js";
import { orderNewestFirst } from "../../web/src/lib/projects/repository-log.js";
import {
  ancestorsOf,
  collapseRepositoryLogRevisions,
} from "../../web/src/lib/projects/repository-log-collapse.js";
export const NOW = 1790506800;
export const IDS = {
  root: "01".repeat(64),
  base: "12".repeat(64),
  left: "23".repeat(64),
  right: "34".repeat(64),
  merge: "45".repeat(64),
  current: "56".repeat(64),
  opRoot: "67".repeat(64),
  opLeft: "78".repeat(64),
  opRight: "89".repeat(64),
};
function revision(
  id: string,
  parents: string[],
  message: string,
  age: number,
  changeId = id.slice(0, 32),
): RepositoryLogRevision {
  return {
    eventId: id,
    commitId: id,
    changeId,
    parents,
    message,
    reason: parents.length > 1 ? "merge" : "checkpoint",
    createdAt: NOW - age,
    root: IDS.root,
    author: "Maya Chen",
    authorEmail: "maya@example.test",
    empty: false,
    bookmarks: [],
    workspaces: [],
    conflicted: false,
    divergent: false,
  };
}
export const revisions: RepositoryLogRevision[] = [
  {
    ...revision(IDS.current, [IDS.merge], "Working copy", 30),
    empty: true,
    bookmarks: [
      {
        name: "main",
        added: true,
        removed: false,
        conflicted: false,
      },
    ],
    workspaces: ["default"],
  },
  revision(
    IDS.merge,
    [IDS.left, IDS.right],
    "Merge native storage and publication",
    180,
  ),
  {
    ...revision(
      IDS.left,
      [IDS.base],
      "Stream immutable native objects",
      360,
      "ab".repeat(16),
    ),
    bookmarks: [
      {
        name: "feature/native-objects",
        added: true,
        removed: false,
        conflicted: false,
      },
      {
        name: "feature/native-objects",
        remote: "origin",
        added: true,
        removed: false,
        conflicted: false,
      },
    ],
    conflicted: true,
    divergent: true,
  },
  {
    ...revision(
      IDS.right,
      [IDS.base],
      "Preserve concurrent publication history",
      400,
      "ab".repeat(16),
    ),
    bookmarks: [
      {
        name: "feature/publication",
        added: true,
        removed: false,
        conflicted: false,
      },
    ],
    divergent: true,
  },
  revision(
    IDS.base,
    [IDS.root],
    "Describe a long change with <script>alert('source')</script> displayed safely as text and retain the complete original description for keyboard and assistive technology inspection.",
    600,
  ),
  revision(IDS.root, [], "Initialize native repository", 1200),
];
export const bookmarks: Bookmark[] = [
  {
    name: "main",
    targets: [IDS.current],
    default: true,
    workspace: true,
    description: "Working copy",
    author: "Maya Chen",
    authorEmail: "maya@example.test",
    updatedAt: NOW - 30,
  },
  {
    name: "feature/native-objects",
    description: "Stream immutable native objects",
    author: "Maya Chen",
    authorEmail: "maya@example.test",
    updatedAt: NOW - 360,
    targets: [IDS.left],
  },
  {
    name: "feature/publication",
    description: "Preserve concurrent publication history",
    author: "Maya Chen",
    authorEmail: "maya@example.test",
    updatedAt: NOW - 400,
    targets: [IDS.right],
  },
  {
    name: "conflicted",
    description: "Concurrent bookmark updates require an explicit target",
    author: "Maya Chen",
    authorEmail: "maya@example.test",
    updatedAt: NOW - 400,
    targets: [IDS.left, IDS.right],
  },
  {
    name: "retired",
    description: "Deleted bookmark with no target",
    author: "Maya Chen",
    authorEmail: "maya@example.test",
    updatedAt: NOW - 1200,
    targets: [],
  },
];
export const workingCopies = [
  {
    workspace: "default",
    commitId: IDS.current,
    changeId: revisions[0]!.changeId,
    message: revisions[0]!.message,
    createdAt: revisions[0]!.createdAt,
    author: revisions[0]!.author,
    authorEmail: revisions[0]!.authorEmail,
  },
];
export function fixtureGraph(
  bookmark?: string,
  sourceBookmarks: readonly Bookmark[] = bookmarks,
  sourceRevisions: readonly RepositoryLogRevision[] = revisions,
  sourceTargets: readonly string[] = [IDS.current],
): RepositoryLogGraph {
  const selected = sourceBookmarks.find((b) => b.name === bookmark);
  const visible = ancestorsOf(
    collapseRepositoryLogRevisions(sourceRevisions),
    selected?.targets ?? sourceTargets,
  );
  return {
    revisions: orderNewestFirst(visible),
    devices: [],
  };
}
export const operations: RepositoryLogGraph = {
  revisions: [
    {
      ...revision(
        IDS.opLeft,
        [IDS.opRoot],
        "describe: native object history",
        90,
        "",
      ),
      workspaces: ["default"],
    },
    revision(
      IDS.opRight,
      [IDS.opRoot],
      "bookmark set feature/publication",
      120,
      "",
    ),
    revision(IDS.opRoot, [], "initialize repository", 1400, ""),
  ],
  devices: [IDS.opLeft, IDS.opRight].map((id, i) => ({
    publicKey: id,
    alias: `head ${i + 1}`,
    revisionEventId: id,
    lastSeen: NOW,
  })),
};
export const sampleDetail: RevisionDetail = {
  files: [
    { path: "src/lib.rs", additions: 3, deletions: 1 },
    { path: "README.md", additions: 1, deletions: 0 },
    { path: "assets/logo.bin", additions: 0, deletions: 0 },
  ],
  patches: {
    "src/lib.rs":
      "diff --git a/src/lib.rs b/src/lib.rs\n--- a/src/lib.rs\n+++ b/src/lib.rs\n@@ -1,2 +1,4 @@\n-pub fn store() {}\n+pub fn store() -> Result<(), Error> {\n+    publish_native_objects()?;\n+    Ok(())\n }\n",
    "README.md":
      "diff --git a/README.md b/README.md\n--- a/README.md\n+++ b/README.md\n@@ -1 +1,2 @@\n # Native storage\n+<script>alert('source')</script>\n",
  },
};

export const SOURCE_TEXT = "<script>alert('source')</script>";
