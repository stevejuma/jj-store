import { stackGroups } from "./presentation.js";
import type {
  WorkspaceCommand,
  WorkspaceSnapshot,
  WorkspaceStack,
} from "./types.js";

/** Exact header destination checks, shared by feedback and the isolated mock. */
export function moveCommitProblem(
  snapshot: WorkspaceSnapshot,
  command: Extract<WorkspaceCommand, { kind: "move_commit" }>,
) {
  const source = snapshot.stacks.find((stack) =>
    stack.commits.some((commit) => commit.id === command.commit),
  );
  const destination = snapshot.stacks.find(
    (stack) => stack.id === command.stack,
  );
  if (!source || !destination)
    return "The source commit or destination stack is unavailable.";
  const branch = destination.heads.find((head) => head.name === command.branch);
  if (command.branch !== undefined && !branch)
    return "The destination branch is unavailable.";
  if (source === destination) {
    if (!branch) return "Choose another stack.";
    if (
      stackGroups(source).some(
        (group) =>
          group.head.name === branch.name &&
          group.commits.some((commit) => commit.id === command.commit),
      )
    )
      return "Choose another branch.";
  }
  const sourceIndex = source.commits.findIndex(
    (commit) => commit.id === command.commit,
  );
  const insertion = branch
    ? destination.commits.findIndex((commit) => commit.id === branch.commit) + 1
    : destination.commits.length;
  if (
    source.commits.slice(sourceIndex).some((commit) => commit.immutable) ||
    destination.commits.slice(insertion).some((commit) => commit.immutable)
  )
    return "This move would rewrite immutable history.";
  return null;
}

export function placedCommits(
  stack: WorkspaceStack,
  source: string,
  target: string,
  placement: "above" | "below",
) {
  const commit = stack.commits.find((commit) => commit.id === source)!;
  const result = stack.commits.filter((commit) => commit.id !== source);
  result.splice(
    result.findIndex((commit) => commit.id === target) +
      (placement === "above" ? 1 : 0),
    0,
    commit,
  );
  return result;
}

export function sortCommitProblem(
  snapshot: WorkspaceSnapshot,
  source: string,
  target: string,
  placement: "above" | "below",
) {
  const stack = snapshot.stacks.find((stack) =>
    stack.commits.some((commit) => commit.id === source),
  );
  if (!stack || !stack.commits.some((commit) => commit.id === target))
    return "Choose a sorting target in the same stack.";
  if (source === target) return "Choose a different commit.";
  const sorted = placedCommits(stack, source, target, placement);
  if (sorted.every((commit, index) => commit === stack.commits[index]))
    return "The commit is already at this position.";
  const firstChanged = sorted.findIndex(
    (commit, index) => commit !== stack.commits[index],
  );
  if (stack.commits.slice(firstChanged).some((commit) => commit.immutable))
    return "This reorder would rewrite immutable history.";
  return null;
}

/** Native commit drags use the outer quarters for insertion; files keep their amend target. */
export function commitDropPlacement(
  payload: string | null,
  bounds: { top: number; height: number },
  clientY: number,
) {
  try {
    if (!payload || JSON.parse(payload).kind !== "commit" || bounds.height <= 0)
      return undefined;
    const ratio = (clientY - bounds.top) / bounds.height;
    return ratio < 0.25
      ? ("above" as const)
      : ratio > 0.75
        ? ("below" as const)
        : undefined;
  } catch {
    return undefined;
  }
}
