import { workspaceSourceFiles } from "./files.js";
import {
  cloneWorkspace,
  type WorkspaceSelection,
  type WorkspaceSnapshot,
  type WorkspaceCommand,
} from "./types.js";

export class WorkspaceCommitDrafts {
  drafts = $state<
    Record<
      string,
      {
        title: string;
        body: string;
        selections: WorkspaceSelection[];
        commit?: string;
      }
    >
  >({});
  key(workspace: string, stack: string) {
    return JSON.stringify([workspace, stack]);
  }
  get(workspace: string, stack: string) {
    return this.drafts[this.key(workspace, stack)];
  }
  open(
    snapshot: WorkspaceSnapshot,
    stack: string,
    selection?: WorkspaceSelection,
  ) {
    if (
      snapshot.availability !== "ready" ||
      !snapshot.folder ||
      !snapshot.workspaces
        .find((item) => item.id === snapshot.active)
        ?.applied.includes(stack) ||
      !snapshot.stacks.some((item) => item.id === stack)
    )
      return false;
    if (selection) {
      const file = workspaceSourceFiles(snapshot, selection.source).find(
        (file) => file.path === selection.path,
      );
      const commit = snapshot.stacks
        .flatMap((stack) => stack.commits)
        .find((commit) => commit.id === selection.source);
      if (
        !file ||
        (selection.source !== "unassigned" && (!commit || commit.immutable)) ||
        (selection.hunks &&
          (!selection.hunks.length ||
            selection.hunks.some(
              (id) => !file.hunks.some((hunk) => hunk.id === id),
            )))
      )
        return false;
    }
    const key = this.key(snapshot.active, stack);
    const current = this.drafts[key] ?? { title: "", body: "", selections: [] };
    const selections = cloneWorkspace(current.selections);
    if (selection) {
      const index = selections.findIndex(
        (item) =>
          item.source === selection.source && item.path === selection.path,
      );
      const previous = selections[index];
      const next = previous
        ? {
            source: selection.source,
            path: selection.path,
            ...(previous.hunks && selection.hunks
              ? { hunks: [...new Set([...previous.hunks, ...selection.hunks])] }
              : {}),
          }
        : cloneWorkspace(selection);
      if (index < 0) selections.push(next);
      else selections[index] = next;
    }
    this.drafts = { ...this.drafts, [key]: { ...current, selections } };
    return true;
  }
  edit(snapshot: WorkspaceSnapshot, id: string) {
    const stack = snapshot.stacks.find((stack) =>
      stack.commits.some((commit) => commit.id === id),
    );
    const commit = stack?.commits.find((commit) => commit.id === id);
    if (
      !stack ||
      !commit ||
      commit.immutable ||
      this.get(snapshot.active, stack.id)
    )
      return false;
    if (!this.open(snapshot, stack.id)) return false;
    const [title, ...body] = commit.message.split("\n");
    this.drafts[this.key(snapshot.active, stack.id)] = {
      title: title ?? "",
      body: body.join("\n").trim(),
      commit: id,
      selections: commit.files.map((file) => ({ source: id, path: file.path })),
    };
    return true;
  }
  created(snapshot: WorkspaceSnapshot, command: WorkspaceCommand) {
    if (command.kind !== "create_stack" || !command.selections?.length) return;
    const name = command.name.trim();
    const destination = snapshot.stacks.find((stack) =>
      stack.heads.some((head) => head.name === name),
    );
    if (!destination) return;
    for (const selection of command.selections)
      this.open(snapshot, destination.id, selection);
    return { id: destination.id, name };
  }
  committed(snapshot: WorkspaceSnapshot, command: WorkspaceCommand) {
    const stack =
      command.kind === "create_commit"
        ? command.stack
        : command.kind === "edit_commit"
          ? snapshot.stacks.find((stack) =>
              stack.commits.some((commit) => commit.id === command.commit),
            )?.id
          : null;
    if (stack) this.clear(snapshot.active, stack);
  }
  clear(workspace: string, stack: string) {
    const key = this.key(workspace, stack);
    this.drafts = Object.fromEntries(
      Object.entries(this.drafts).filter(([id]) => id !== key),
    );
  }
}
