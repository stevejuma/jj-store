import type { StackWorkspaceController } from "./controller.svelte.js";
import { workspaceSourceFiles } from "./files.js";

/** Lazy provider files always win over presentation unions. */
export function workspaceControllerFiles(
  controller: Pick<
    StackWorkspaceController,
    "details" | "targetHistory" | "snapshot"
  >,
  source: string,
) {
  if (controller.details.lazy) return controller.details.files[source] ?? [];
  return (
    controller.targetHistory.commits.find((commit) => commit.id === source)
      ?.files ??
    (controller.snapshot
      ? workspaceSourceFiles(controller.snapshot, source)
      : [])
  );
}
