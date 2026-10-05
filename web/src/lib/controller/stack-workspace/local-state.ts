import type { LocalStacksCommand } from "./local-transport.js";
import type { Model } from "../../api/client.js";
import {
  WorkspaceCommandError,
  type WorkspaceSnapshot,
  type WorkspaceRequest,
  type WorkspaceCommand,
} from "./types.js";
export function projectLocalStacks(
  snapshot: WorkspaceSnapshot,
  state: Model<"LocalStacksState">,
) {
  const active = state.workspaces.find(
    (workspace) => workspace.id === state.active,
  );
  snapshot.active = state.active;
  snapshot.workspaces = state.workspaces.map((workspace) => ({
    id: workspace.id,
    name: workspace.name,
    applied: workspace.applied.map(
      (branch) =>
        snapshot.stacks.find((stack) => stack.name === branch)?.id ??
        `branch:${JSON.stringify([branch])}`,
    ),
    unassigned: [],
    mergeCommit: workspace.merge_commit,
    wipCommit: workspace.wip_commit,
  }));
  snapshot.workspaceWarning =
    state.warning ??
    (active?.conflicted
      ? "This workspace contains native JJ conflicts. Inspect and resolve them with JJ; your branch membership and unfinished work are retained."
      : "");
  snapshot.capabilities =
    state.warning && active
      ? []
      : active
        ? [
            "create_workspace",
            "switch_workspace",
            "apply_stack",
            "unapply_stack",
          ]
        : [
            "create_workspace",
            ...(state.workspaces.length ? ["switch_workspace" as const] : []),
          ];
  if (active) {
    snapshot.target =
      snapshot.targetOptions?.find(
        (target) =>
          target.name === active.target &&
          target.commit === active.target_commit,
      ) ?? null;
    snapshot.base = active.target;
    snapshot.stacks = snapshot.stacks
      .filter((stack) => stack.name !== active.target)
      .map((stack) => ({ ...stack, base: active.target }));
    snapshot.bookmarks = snapshot.bookmarks?.filter(
      (bookmark) => bookmark.name !== active.target,
    );
  }
  snapshot.readKey = JSON.stringify([
    snapshot.readKey,
    String(state.revision),
    state.active,
  ]);
  return snapshot;
}
export function localStacksCommand(
  snapshot: WorkspaceSnapshot,
  command: WorkspaceCommand,
): LocalStacksCommand {
  if (command.kind === "create_workspace") {
    if (!snapshot.target)
      throw new WorkspaceCommandError(
        "invalid",
        "Choose a workspace target branch first.",
      );
    return snapshot.workspaces.length
      ? { kind: "create", name: command.name }
      : { kind: "setup", name: command.name, target: snapshot.target.name };
  }
  if (command.kind === "switch_workspace")
    return { kind: "switch", workspace: command.workspace };
  if (command.kind === "apply_stack" || command.kind === "unapply_stack") {
    const stack = snapshot.stacks.find((stack) => stack.id === command.stack);
    if (!stack)
      throw new WorkspaceCommandError(
        "invalid",
        "The selected branch is unavailable.",
      );
    return {
      kind: command.kind === "apply_stack" ? "apply" : "unapply",
      branch: stack.name,
    };
  }
  throw new WorkspaceCommandError(
    "invalid",
    "This command is not implemented for local workspaces yet.",
  );
}

export function outcomeText(status: "committed" | "refreshed") {
  return status === "refreshed"
    ? "Current JJ state refreshed. The command outcome was not confirmed; inspect operation history before trying again."
    : "Command completed";
}

export function findWorkspaceTarget(
  snapshot: WorkspaceSnapshot,
  requested: { name: string; commit: string },
) {
  const target = snapshot.targetOptions?.find(
    (target) =>
      target.name === requested.name && target.commit === requested.commit,
  );
  if (!target)
    throw new WorkspaceCommandError(
      "invalid",
      "Choose an exact available target branch.",
    );
  return target;
}
export function blocksRetry(
  adapter: { readonly retryPending?: boolean },
  pending: WorkspaceRequest | null,
) {
  return pending !== null && adapter.retryPending === false;
}
