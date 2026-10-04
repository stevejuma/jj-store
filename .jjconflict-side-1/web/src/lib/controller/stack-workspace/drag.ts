import { workspaceSourceFiles } from "./files.js";
import { activeWorkspace } from "./presentation.js";
import { moveCommitProblem, sortCommitProblem } from "./commit-placement.js";
import type {
  WorkspaceCommand,
  WorkspaceSelection,
  WorkspaceSnapshot,
} from "./types.js";

/** Adapted from Changeyard vcs-workspace-dnd.ts, with an owned frontend contract. */
export type WorkspaceDrag =
  | { kind: "stack"; stack: string }
  | { kind: "commit"; commit: string }
  | { kind: "selection"; selection: WorkspaceSelection }
  | { kind: "folder"; selections: WorkspaceSelection[] };
export const WORKSPACE_DRAG_TYPE = "application/vnd.lapis.stack-workspace+json";
export function workspaceDrop(
  value: string,
  target: "workspace" | "stack" | "stage" | "commit" | "working",
  id = "",
  branch?: string,
  placement?: "above" | "below",
): WorkspaceCommand | null {
  try {
    const item = JSON.parse(value) as WorkspaceDrag;
    if (
      item.kind === "stack" &&
      typeof item.stack === "string" &&
      target === "workspace"
    )
      return { kind: "apply_stack", stack: item.stack };
    if (
      item.kind === "commit" &&
      typeof item.commit === "string" &&
      target === "stack" &&
      id
    )
      return {
        kind: "move_commit",
        commit: item.commit,
        stack: id,
        ...(branch !== undefined ? { branch } : {}),
      };
    if (
      item.kind === "commit" &&
      typeof item.commit === "string" &&
      target === "commit" &&
      id &&
      id !== item.commit
    )
      return placement
        ? { kind: "reorder_commit", commit: item.commit, target: id, placement }
        : { kind: "squash", commit: item.commit, target: id };
    if (
      item.kind === "folder" &&
      Array.isArray(item.selections) &&
      item.selections.length &&
      item.selections.every((selection) =>
        workspaceDrop(
          JSON.stringify({ kind: "selection", selection }),
          "stage",
          "probe",
        ),
      )
    ) {
      if ((target === "stage" || target === "stack") && id)
        return {
          kind: "create_commit",
          stack: id,
          message: "",
          selections: item.selections,
        };
      if ((target === "commit" && id) || target === "working")
        return {
          kind: "transfer",
          target: target === "working" ? "unassigned" : id,
          selection: item.selections[0]!,
          selections: item.selections,
        };
    }
    if (
      item.kind === "selection" &&
      item.selection &&
      typeof item.selection.source === "string" &&
      item.selection.source.length > 0 &&
      typeof item.selection.path === "string" &&
      item.selection.path.length > 0 &&
      (item.selection.hunks === undefined ||
        (Array.isArray(item.selection.hunks) &&
          item.selection.hunks.length > 0 &&
          item.selection.hunks.every((hunk) => typeof hunk === "string")))
    ) {
      if ((target === "commit" && id) || target === "working")
        return {
          kind: "transfer",
          target: target === "working" ? "unassigned" : id,
          selection: item.selection,
        };
      if ((target === "stack" || target === "stage") && id)
        return {
          kind: "create_commit",
          stack: id,
          message: "Selected changes",
          selection: item.selection,
        };
    }
  } catch {
    /* Foreign or malformed drags cannot become commands. */
  }
  return null;
}

export type WorkspaceDropTarget = {
  kind: "workspace" | "stack" | "stage" | "commit" | "working";
  id?: string;
  branch?: string;
  placement?: "above" | "below";
};
export type WorkspaceDropFeedback = {
  state: "valid" | "invalid";
  message: string;
  command?: WorkspaceCommand;
  placement?: "above" | "below";
};

/** Changeyard-style preview feedback, based only on the currently pinned UI snapshot. */
export function workspaceDropFeedback(
  snapshot: WorkspaceSnapshot | null,
  payload: string,
  target: WorkspaceDropTarget,
  locked = false,
): WorkspaceDropFeedback {
  const invalid = (message: string): WorkspaceDropFeedback => ({
    state: "invalid",
    message,
  });
  if (
    !snapshot ||
    locked ||
    snapshot.availability !== "ready" ||
    !snapshot.folder
  )
    return invalid("Local changes are unavailable while a command is pending.");
  const command = workspaceDrop(
    payload,
    target.kind,
    target.id,
    target.branch,
    target.placement,
  );
  if (!command) return invalid("This target does not accept the dragged item.");
  if (!snapshot.capabilities.includes(command.kind))
    return invalid("This operation is unavailable in this repository.");
  if (snapshot.conflicts.length)
    return invalid("Resolve workspace conflicts before moving changes.");
  const commits = snapshot.stacks.flatMap((stack) => stack.commits);
  const commit = (id: string) => commits.find((item) => item.id === id);
  const stack =
    "stack" in command
      ? snapshot.stacks.find((item) => item.id === command.stack)
      : undefined;
  if ("stack" in command && !stack)
    return invalid("The destination stack is unavailable.");
  if (
    command.kind === "apply_stack" &&
    activeWorkspace(snapshot).applied.includes(command.stack)
  )
    return invalid("This stack is already applied.");
  if (
    "commit" in command &&
    (!commit(command.commit) || commit(command.commit)?.immutable)
  )
    return invalid("The source commit is unavailable or immutable.");
  if (
    "target" in command &&
    command.target !== "unassigned" &&
    (!commit(command.target) ||
      (command.kind !== "reorder_commit" && commit(command.target)?.immutable))
  )
    return invalid("The destination commit is unavailable or immutable.");
  if (command.kind === "move_commit") {
    const problem = moveCommitProblem(snapshot, command);
    if (problem) return invalid(problem);
  }
  if (command.kind === "reorder_commit" && "target" in command) {
    const problem = sortCommitProblem(
      snapshot,
      command.commit,
      command.target,
      command.placement,
    );
    if (problem) return invalid(problem);
  }
  if (
    command.kind === "squash" &&
    !snapshot.stacks.some(
      (item) =>
        item.commits.some((c) => c.id === command.commit) &&
        item.commits.some((c) => c.id === command.target),
    )
  )
    return invalid("Choose a squash destination in the same stack.");
  const selections =
    command.kind === "create_commit"
      ? (command.selections ?? (command.selection ? [command.selection] : []))
      : command.kind === "transfer"
        ? (command.selections ?? [command.selection])
        : [];
  for (const selection of selections) {
    const file = workspaceSourceFiles(snapshot, selection.source).find(
      (item) => item.path === selection.path,
    );
    if (
      !file ||
      (selection.source !== "unassigned" &&
        (!commit(selection.source) || commit(selection.source)?.immutable)) ||
      selection.hunks?.some((id) => !file.hunks.some((hunk) => hunk.id === id))
    )
      return invalid("The dragged changes are stale, ambiguous or immutable.");
    if (command.kind === "transfer" && selection.source === command.target)
      return invalid("Choose a different destination.");
  }
  if (
    command.kind === "create_commit" &&
    !activeWorkspace(snapshot).applied.includes(command.stack)
  )
    return invalid("Apply the stack before staging changes.");
  const message =
    command.kind === "create_commit"
      ? "Stage changes for a new commit"
      : command.kind === "transfer"
        ? command.target === "unassigned"
          ? "Move changes back to Unstaged"
          : "Move changes into this commit"
        : command.kind === "move_commit"
          ? command.branch
            ? `Move commit to branch ${command.branch}`
            : "Move commit to this stack"
          : command.kind === "reorder_commit" && "target" in command
            ? `Place commit ${command.placement} this row`
            : command.kind === "squash"
              ? "Squash into this commit"
              : "Apply this stack";
  return {
    state: "valid",
    message,
    command,
    ...(target.placement ? { placement: target.placement } : {}),
  };
}
