import type {
  CodeEntry,
  CodeDiffChange,
  CodeTreeValue,
  CodeGraph,
} from "./repository-read-types.js";
import type { FileListingFile } from "@lapismd/design-core/diff";
import type { Model } from "../api/client.js";
import { encodeJjChangeId } from "../projects/repository-log-graph.js";
import type {
  RepositoryLogGraph,
  RepositoryLogRevision,
  RepositoryWorkingCopy,
} from "../projects/repository-log.js";
import type { Bookmark } from "../projects/presentation.js";
import type { RepositoryChoice, RepositorySnapshot } from "./contracts.js";

type Reference = Model<"Reference">;
type GraphPage = CodeGraph;
type PathEntry = CodeEntry;
type DiffChange = CodeDiffChange;
type FileTreeValue = Extract<CodeTreeValue, { type: "file" }>;
type DirectoryTreeValue = Extract<CodeTreeValue, { type: "tree" }>;

export interface PresentedReferences {
  bookmarks: Bookmark[];
  choices: RepositoryChoice[];
  workingCopies: RepositoryWorkingCopy[];
}

export interface PresentedPaths {
  snapshot: RepositorySnapshot;
  files: Map<string, string>;
}

const seconds = (millis: number | bigint) => Number(BigInt(millis) / 1000n);

export function presentReferences(
  references: readonly Reference[],
): PresentedReferences {
  const bookmarks: Bookmark[] = [];
  const choices: RepositoryChoice[] = [];
  const workingCopies = new Map<string, RepositoryWorkingCopy>();
  for (const reference of references) {
    for (const target of reference.targets) {
      for (const workspace of target.workspaces) {
        workingCopies.set(workspace, {
          workspace,
          commitId: target.commit,
          changeId: target.change,
          message: target.description || "(empty) (no description set)",
          createdAt: seconds(target.committer.timestamp.millis),
          author: target.author.name || target.author.email,
          authorEmail: target.author.email,
        });
      }
    }
    if (reference.kind === "workspace") continue;
    if (reference.kind === "bookmark") {
      const target = reference.targets[0];
      const targets = reference.terms.filter((id): id is string => Boolean(id));
      bookmarks.push({
        name: reference.name,
        targets,
        workspace: reference.workspaces.length > 0,
        description: target?.description || undefined,
        author: target?.author.name,
        authorEmail: target?.author.email,
        updatedAt: target
          ? seconds(target.committer.timestamp.millis)
          : undefined,
      });
      choices.push({
        value: `bookmark:${reference.name}`,
        label: reference.name,
        kind: "bookmark",
        targets,
      });
      continue;
    }
    const targets = reference.targets.map((target) => target.commit);
    const target = reference.targets[0];
    choices.push({
      value: `change:${reference.change}`,
      label: target
        ? `${encodeJjChangeId(reference.change).slice(0, 12)} · ${target.description || "No description"}`
        : encodeJjChangeId(reference.change),
      kind: "change",
      searchTerms: [reference.change, encodeJjChangeId(reference.change)],
      targets,
    });
  }
  return { bookmarks, choices, workingCopies: [...workingCopies.values()] };
}

export function presentCommit(
  node: Model<"GraphCommitNode">,
): RepositoryLogRevision {
  const commit = node.commit;
  return {
    eventId: commit.id,
    commitId: commit.id,
    changeId: commit.change_id,
    parents: node.edges.map((edge) => edge.target),
    message: commit.description || "No description",
    reason: commit.parents.length > 1 ? "merge" : "checkpoint",
    createdAt: seconds(commit.committer.timestamp.millis),
    root: commit.tree_terms[0] ?? "",
    author: commit.author.name || commit.author.email,
    authorEmail: commit.author.email,
    empty: false,
    bookmarks: commit.bookmarks.map((bookmark) => ({
      ...bookmark,
      remote: bookmark.remote ?? undefined,
    })),
    workspaces: [...commit.workspaces],
    conflicted: commit.conflicted,
    divergent: commit.divergent,
  };
}

function presentOperation(
  node: Model<"GraphOperationNode">,
): RepositoryLogRevision {
  return {
    eventId: node.id,
    changeId: "",
    parents: node.parents,
    message: node.description || "No description",
    reason: node.parents.length > 1 ? "merge" : "checkpoint",
    createdAt: seconds(node.end.millis),
    root: node.view,
    author: node.user || "Unknown user",
    authorEmail: node.host,
    empty: false,
    bookmarks: [],
    workspaces: node.workspace ? [node.workspace] : [],
    conflicted: false,
    divergent: false,
    operationSnapshot: node.is_snapshot,
  };
}

export function presentGraph(
  graph: GraphPage,
  operationHeads: readonly string[],
): RepositoryLogGraph {
  let revisions: RepositoryLogRevision[];
  if (graph.data.graph === "operations")
    revisions = graph.data.nodes.map(presentOperation);
  else if (graph.data.graph === "log")
    revisions = graph.data.nodes.map(presentCommit);
  else
    revisions = graph.data.nodes.map((node) => ({
      ...presentCommit({
        commit: node.commit,
        edges: node.commit.parents.map((target) => ({
          target,
          kind: "direct" as const,
        })),
      }),
      parents: node.predecessors,
    }));
  return {
    revisions,
    devices: operationHeads.map((id, index) => ({
      publicKey: id,
      alias: `head ${index + 1}`,
      revisionEventId: id,
      lastSeen:
        revisions.find((revision) => revision.eventId === id)?.createdAt ?? 0,
    })),
  };
}

function soleFile(entry: PathEntry): FileTreeValue | undefined {
  return entry.terms.length === 1 && entry.terms[0]?.type === "file"
    ? entry.terms[0]
    : undefined;
}

function soleDirectory(entry: PathEntry): DirectoryTreeValue | undefined {
  return entry.terms.length === 1 && entry.terms[0]?.type === "tree"
    ? entry.terms[0]
    : undefined;
}

export function presentPaths(
  commit: string,
  entries: readonly PathEntry[],
): PresentedPaths {
  const files = new Map<string, string>();
  const directories = entries.flatMap((entry) =>
    soleDirectory(entry) ? [{ path: entry.path }] : [],
  );
  const blobs = entries.flatMap((entry) => {
    const file = soleFile(entry);
    if (!file) return [];
    files.set(entry.path, file.id);
    return [{ path: entry.path }];
  });
  const pathChanges: NonNullable<RepositorySnapshot["pathChanges"]> =
    Object.fromEntries(
      entries.flatMap<
        [
          string,
          (
            | import("../projects/repository-log.js").RepositoryCommitSummary
            | undefined
          ),
        ]
      >((entry) => {
        const value = entry.last_change;
        if (value.status === "unavailable")
          return value.reason === "not_requested"
            ? []
            : [[entry.path, undefined]];
        return [
          [
            entry.path,
            {
              eventId: value.commit,
              commitId: value.commit,
              changeId: value.change,
              message: value.description || "No description",
              createdAt: seconds(value.committer.timestamp.millis),
              author: value.author.name || value.author.email,
              authorEmail: value.author.email,
            },
          ],
        ];
      }),
    );
  return {
    snapshot: { id: commit, files: blobs, directories, pathChanges },
    files,
  };
}

const lineCount = (value: string) =>
  value ? value.replace(/\n$/u, "").split("\n").length : 0;

export function presentChangedFiles(
  changes: readonly DiffChange[],
): FileListingFile[] {
  return changes.map((change) => ({
    path: change.path,
    additions:
      change.content.status === "text"
        ? change.content.hunks.reduce(
            (sum, hunk) =>
              sum + (hunk.kind === "different" ? lineCount(hunk.after) : 0),
            0,
          )
        : 0,
    deletions:
      change.content.status === "text"
        ? change.content.hunks.reduce(
            (sum, hunk) =>
              sum + (hunk.kind === "different" ? lineCount(hunk.before) : 0),
            0,
          )
        : 0,
  }));
}

function patchLines(value: string, prefix: string): string {
  const lines = value.replace(/\n$/u, "").split("\n");
  return value ? lines.map((line) => `${prefix}${line}`).join("\n") + "\n" : "";
}

export function presentPatch(change: DiffChange): string | undefined {
  if (change.content.status !== "text") return undefined;
  let patch = `diff --git a/${change.path} b/${change.path}\n--- a/${change.path}\n+++ b/${change.path}\n`;
  let beforeLine = 1;
  let afterLine = 1;
  for (const hunk of change.content.hunks) {
    const before = lineCount(hunk.before);
    const after = lineCount(hunk.after);
    const beforeStart = before === 0 ? beforeLine - 1 : beforeLine;
    const afterStart = after === 0 ? afterLine - 1 : afterLine;
    patch += `@@ -${beforeStart},${before} +${afterStart},${after} @@\n`;
    if (hunk.kind === "matching") patch += patchLines(hunk.before, " ");
    else patch += patchLines(hunk.before, "-") + patchLines(hunk.after, "+");
    beforeLine += before;
    afterLine += after;
  }
  return patch;
}
