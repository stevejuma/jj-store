import type { LocalStacksCommand } from "./local-transport.js";
import {
  WorkspaceCommandError,
  type WorkspaceSnapshot,
  type WorkspaceSelection,
} from "./types.js";
type NativeSelection = Extract<
  LocalStacksCommand,
  { kind: "commit" }
>["selections"][number];
export function localCommitSource(snapshot: WorkspaceSnapshot, source: string) {
  if (source === "unassigned") {
    const workspace = snapshot.workspaces.find((w) => w.id === snapshot.active);
    if (workspace?.wipCommit) return workspace.wipCommit;
  } else if (
    snapshot.stacks.some((s) => s.commits.some((c) => c.id === source))
  )
    return source;
  throw new WorkspaceCommandError(
    "invalid",
    "Choose Unstaged or a displayed commit. Net branch diffs cannot be moved as a commit delta.",
  );
}
export function localFileSelections(
  snapshot: WorkspaceSnapshot,
  selections: WorkspaceSelection[],
): NativeSelection[] {
  return selections.map((selection) => {
    if (
      selection.hunks &&
      (!selection.hunks.length ||
        selection.hunks.some((id) => !/^native:[0-9a-f]{64}$/u.test(id)))
    )
      throw new WorkspaceCommandError(
        "invalid",
        "Choose whole files or freshly loaded native hunks.",
      );
    return {
      source: localCommitSource(snapshot, selection.source),
      path: selection.path,
      ...(selection.hunks ? { hunks: selection.hunks } : {}),
    };
  });
}
