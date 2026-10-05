import { presentPatch } from "../native-presentation.js";
import type { Model } from "../../api/client.js";
import type { CodeDiffChange } from "../repository-read-types.js";
import type { StackCommit, WorkspaceFile } from "./types.js";

/** Keep the provider's real parent boundaries even outside the loaded window. */
export function presentStackCommit(
  node: Model<"LocalGraphCommitNode">,
): StackCommit {
  const commit = node.commit;
  return {
    id: commit.id,
    changeId: commit.change_id,
    message: commit.description || "No description",
    author: commit.author.name || commit.author.email,
    email: commit.author.email,
    createdAt: Number(BigInt(commit.committer.timestamp.millis) / 1000n),
    immutable: true,
    files: [],
    graph: {
      parents: [...commit.parents],
      edges: node.edges.map((edge) => ({ ...edge })),
      bookmarks: commit.bookmarks.map((bookmark) => ({
        ...bookmark,
        remote: bookmark.remote ?? undefined,
      })),
      workspaces: [...commit.workspaces],
      conflicted: commit.conflicted,
      divergent: commit.divergent,
    },
  };
}

/** These presentation IDs never authorize a file/hunk transfer. */
export function presentStackFile(
  change: CodeDiffChange,
  identity: readonly string[],
): WorkspaceFile {
  const patch = presentPatch(change);
  const sections = patch?.split(/(?=^@@ )/mu) ?? [];
  const header = sections.shift() ?? "";
  return {
    path: change.path,
    ...(patch ? { patch } : {}),
    change: structuredClone({
      before: change.before,
      after: change.after,
      sources: change.sources,
    }),
    content:
      change.content.status === "text"
        ? { status: "ready" }
        : change.content.reason === "not_requested"
          ? { status: "unloaded" }
          : { status: "unavailable", reason: change.content.reason },
    hunks: sections.flatMap((section, index) =>
      /^[-+]/mu.test(section)
        ? [
            {
              id: `presentation:${JSON.stringify([...identity, change.path, index])}`,
              label: section.split("\n")[0]!,
              patch: header + section,
            },
          ]
        : [],
    ),
  };
}
