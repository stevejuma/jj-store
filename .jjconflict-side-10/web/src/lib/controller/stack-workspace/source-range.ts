import {
  WorkspaceCommandError,
  type WorkspaceSnapshot,
  type StackCommit,
} from "./types.js";
import type { StackTreeRange } from "./reads.js";
export function stackSourceRange(
  snapshot: WorkspaceSnapshot,
  commits: Map<string, StackCommit>,
  source: string,
): StackTreeRange {
  if (source.startsWith("branch:")) {
    let identity: unknown;
    try {
      identity = JSON.parse(source.slice(7));
    } catch {
      throw new WorkspaceCommandError("invalid", "Unknown stack source.");
    }
    if (!Array.isArray(identity) || identity.length !== 2)
      throw new WorkspaceCommandError("invalid", "Unknown stack source.");
    const [id, name] = identity;
    const head = snapshot.stacks
      .find((stack) => stack.id === id)
      ?.heads.find((head) => head.name === name);
    if (!head || !snapshot.target)
      throw new WorkspaceCommandError(
        "invalid",
        "Choose a target branch first.",
      );
    return { from: snapshot.target.commit, to: head.commit };
  }
  const commit = commits.get(source);
  if (!commit)
    throw new WorkspaceCommandError(
      "invalid",
      "This commit is outside the loaded read scope.",
    );
  if (commit.graph?.parents.length !== 1)
    throw new WorkspaceCommandError(
      "invalid",
      "Choose a comparison parent to read changes in this merge or root commit.",
    );
  return { from: commit.graph.parents[0]!, to: commit.id };
}
