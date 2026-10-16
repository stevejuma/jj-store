import { workspaceSourceStack } from "./files.js";
import { stackGroups, stackLaneMembers } from "./presentation.js";
import type { StackWorkspaceController } from "./controller.svelte.js";
import type { WorkspaceSnapshot, WorkspaceStack } from "./types.js";

/** Resolve the bookmark segment without changing the operation-pinned diff source. */
export function workspaceSourceBranch(
  snapshot: WorkspaceSnapshot,
  source: string,
  preferredStack?: string,
) {
  const stack = workspaceSourceStack(snapshot, source, preferredStack);
  if (!stack) return undefined;
  if (source.startsWith("branch:"))
    return (JSON.parse(source.slice(7)) as [string, string])[1];
  return source.startsWith("stack:")
    ? stack.name
    : stackGroups(stack).find((group) =>
        group.commits.some((commit) => commit.id === source),
      )?.head.name;
}

/** PR-only views and file views share the existing per-origin column identity. */
export function stackDetailsColumns(
  controller: StackWorkspaceController,
  stack: WorkspaceStack,
) {
  const members = stackLaneMembers(controller.snapshot!, stack);
  const diffs = controller.diffs.filter((diff) =>
    members.some((member) => diff.origin === `stack-${member.id}`),
  );
  const forge = controller.forge;
  const selected = forge?.surface === "inline" ? forge.selected : undefined;
  const owner = selected
    ? (members.find((member) => member.name === selected.head) ??
      members.find((member) =>
        member.heads.some((head) => head.name === selected.head),
      ))
    : undefined;
  const selectedOrigin = owner ? `stack-${owner.id}` : undefined;
  const origins = [
    ...new Set([
      ...diffs.map((diff) => diff.origin),
      ...(selectedOrigin ? [selectedOrigin] : []),
    ]),
  ];
  return origins.map((origin) => {
    const diff = diffs.find((item) => item.origin === origin);
    return {
      origin,
      diff,
      branch:
        origin === selectedOrigin
          ? selected?.head
          : diff &&
            workspaceSourceBranch(
              controller.snapshot!,
              diff.source,
              origin.slice(6),
            ),
    };
  });
}
