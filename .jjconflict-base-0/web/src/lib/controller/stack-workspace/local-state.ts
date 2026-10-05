import { localFileSelections } from "./local-selections.js";
import { localMutationCommand, localMutationKinds } from "./local-mutations.js";
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
    mergeCommit: workspace.merge_commit || undefined,
    wipCommit: workspace.wip_commit || undefined,
    unavailableReason: workspace.unavailable_reason ?? undefined,
  }));
  snapshot.workspaceWarning =
    state.warning ??
    active?.unavailable_reason ??
    (active?.membership_changed
      ? "Native branch membership differs from saved configuration after a CLI/Undo change. Review it and explicitly reapply saved branches to reconcile; Refresh does not change the graph."
      : null) ??
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
            "update_workspace",
            "delete_workspace",
            "apply_stack",
            "unapply_stack",
            ...localMutationKinds,
          ]
        : [
            "create_workspace",
            ...(state.workspaces.length ? ["switch_workspace" as const] : []),
          ];
  snapshot.canUndo = state.can_undo ?? false;
  snapshot.canRedo = state.can_redo ?? false;
  snapshot.membershipChanged = active?.membership_changed ?? false;
  snapshot.conflicts = (state.conflicts ?? []).map((path) => ({
    path,
    left: "",
    base: "",
    right: "",
  }));
  for (const branch of active?.applied ?? []) {
    if (!snapshot.stacks.some((s) => s.name === branch)) {
      snapshot.stacks.push({
        id: `branch:${JSON.stringify([branch])}`,
        name: branch,
        base: active?.target ?? "",
        heads: [],
        commits: [],
        conflicts: false,
      });
      snapshot.branchUnavailableReasons = {
        ...snapshot.branchUnavailableReasons,
        [branch]:
          "This saved applied bookmark is missing or conflicted. Restore it with JJ or explicitly unapply it.",
      };
    }
  }
  if (active?.unavailable_reason || active?.membership_changed) {
    const recovery = [
      "update_workspace",
      "delete_workspace",
      "switch_workspace",
      "unapply_stack",
      "apply_stack",
      "undo",
      "redo",
      "reconcile_workspace",
    ];
    snapshot.capabilityReasons = Object.fromEntries(
      localMutationKinds
        .filter((k) => !recovery.includes(k))
        .map((k) => [k, snapshot.workspaceWarning]),
    );
    snapshot.capabilities = snapshot.capabilities.filter((k) =>
      recovery.includes(k),
    );
  }
  if (!snapshot.capabilities.includes("undo"))
    snapshot.capabilities.push("undo", "redo");
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
  const mutation = localMutationCommand(snapshot, command);
  if (mutation) return mutation;
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
  if (command.kind === "create_commit") {
    const workspace = snapshot.workspaces.find((w) => w.id === snapshot.active);
    const stack = snapshot.stacks.find((s) => s.id === command.stack);
    const selections =
      command.selections ?? (command.selection ? [command.selection] : []);
    if (!workspace?.wipCommit || !stack)
      throw new WorkspaceCommandError(
        "invalid",
        "Select an applied stack to create this commit.",
      );
    return {
      kind: "commit",
      branch: stack.name,
      message: command.message,
      selections: localFileSelections(snapshot, selections),
    };
  }
  if (command.kind === "update_workspace") {
    if (
      snapshot.active !== command.workspace ||
      !command.name.trim() ||
      new TextEncoder().encode(command.name).length > 128 ||
      /[\u0000-\u001f\u007f]/u.test(command.name) ||
      snapshot.workspaces.some(
        (w) => w.id !== command.workspace && w.name === command.name.trim(),
      )
    )
      throw new WorkspaceCommandError(
        "invalid",
        "Enter a unique workspace name up to 128 bytes.",
      );
    const target = findWorkspaceTarget(snapshot, command.targetBranch);
    return {
      kind: "update",
      workspace: command.workspace,
      name: command.name.trim(),
      target: target.name,
    };
  }
  if (command.kind === "delete_workspace") {
    if (!snapshot.workspaces.some((w) => w.id === command.workspace))
      throw new WorkspaceCommandError(
        "invalid",
        "This workspace is no longer available.",
      );
    return { kind: "delete", workspace: command.workspace };
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
