import type { ComponentProps } from "svelte";
import type { FileListing } from "@lapismd/design-core/diff";
import { workspaceFileSelection, workspaceAggregateSource } from "./files.js";
import type { StackWorkspaceController } from "./controller.svelte.js";

/** Host policy for the public FileListing native row-drag callbacks. */
export function workspaceFileDrag(
  controller: StackWorkspaceController,
  source: () => string,
): Pick<
  ComponentProps<typeof FileListing>,
  | "fileDraggable"
  | "onFileDragStart"
  | "directoryDraggable"
  | "onDirectoryDragStart"
> {
  const file = (path: string) => {
    if (
      !(controller.can("create_commit") || controller.can("transfer")) ||
      !controller.snapshot
    )
      return null;
    const displayed = source();
    if (!controller.details.lazy)
      return workspaceFileSelection(controller.snapshot, displayed, path);
    const selected = controller
      .filesFor(displayed)
      .find((f) => f.path === path);
    const owner = selected?.selectionSource ?? displayed;
    if (workspaceAggregateSource(owner) || !selected) return null;
    if (
      owner !== "unassigned" &&
      !controller.snapshot.stacks.some((s) =>
        s.commits.some((c) => c.id === owner && !c.immutable),
      )
    )
      return null;
    return { source: owner, path };
  };
  const folder = (path: string) => {
    if (
      !controller.snapshot ||
      controller.locked ||
      (controller.details.lazy &&
        Boolean(controller.details.fileState(source()).nextCursor))
    )
      return [];
    const selections = controller
      .filesFor(source())
      .filter((item) => item.path.startsWith(`${path}/`))
      .map((item) => file(item.path));
    return selections.length && selections.every((item) => item !== null)
      ? selections.filter((item) => item !== null)
      : [];
  };
  return {
    fileDraggable: (context) => Boolean(file(context.path)),
    onFileDragStart: (event, context) => {
      const selection = file(context.path);
      if (selection)
        controller.drag.start(event, { kind: "selection", selection });
      else event.preventDefault();
    },
    directoryDraggable: (context) => folder(context.node.path).length > 0,
    onDirectoryDragStart: (event, context) => {
      const selections = folder(context.node.path);
      if (selections.length)
        controller.drag.start(event, { kind: "folder", selections });
      else event.preventDefault();
    },
  };
}
