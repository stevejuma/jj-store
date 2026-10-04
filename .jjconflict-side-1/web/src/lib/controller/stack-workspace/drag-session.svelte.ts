import {
  WORKSPACE_DRAG_TYPE,
  workspaceDropFeedback,
  type WorkspaceDrag,
  type WorkspaceDropTarget,
  type WorkspaceDropFeedback,
} from "./drag.js";
import type { WorkspaceSnapshot } from "./types.js";

/** Copied Changeyard active-payload/target lifecycle; one guide owns each drag. */
export class WorkspaceDragSession {
  payload = $state<string | null>(null);
  feedback = $state<(WorkspaceDropFeedback & { key: string }) | null>(null);
  key(target: WorkspaceDropTarget) {
    return `${target.kind}:${target.id ?? ""}`;
  }
  start(event: DragEvent, payload: WorkspaceDrag) {
    event.stopPropagation();
    this.payload = JSON.stringify(payload);
    this.feedback = null;
    event.dataTransfer?.setData(WORKSPACE_DRAG_TYPE, this.payload);
    if (event.dataTransfer) event.dataTransfer.effectAllowed = "move";
  }
  state(target: WorkspaceDropTarget) {
    return this.for(target)?.state ?? "idle";
  }
  for(target: WorkspaceDropTarget) {
    return this.feedback?.key === this.key(target) ? this.feedback : null;
  }
  over(
    event: DragEvent,
    target: WorkspaceDropTarget,
    snapshot: WorkspaceSnapshot | null,
    locked: boolean,
  ) {
    if (!event.dataTransfer?.types.includes(WORKSPACE_DRAG_TYPE)) return;
    event.preventDefault();
    event.stopPropagation();
    const payload =
      this.payload ?? event.dataTransfer.getData(WORKSPACE_DRAG_TYPE);
    const feedback = workspaceDropFeedback(snapshot, payload, target, locked);
    this.feedback = { ...feedback, key: this.key(target) };
    event.dataTransfer.dropEffect =
      feedback.state === "valid" ? "move" : "none";
  }
  leave(
    event: DragEvent,
    target: WorkspaceDropTarget,
    node = event.currentTarget,
  ) {
    if (
      node instanceof Node &&
      event.relatedTarget instanceof Node &&
      node.contains(event.relatedTarget)
    )
      return;
    if (this.feedback?.key === this.key(target)) this.feedback = null;
  }
  drop(
    event: DragEvent,
    target: WorkspaceDropTarget,
    snapshot: WorkspaceSnapshot | null,
    locked: boolean,
  ) {
    if (!event.dataTransfer?.types.includes(WORKSPACE_DRAG_TYPE)) return null;
    event.preventDefault();
    event.stopPropagation();
    const feedback = workspaceDropFeedback(
      snapshot,
      event.dataTransfer.getData(WORKSPACE_DRAG_TYPE),
      target,
      locked,
    );
    this.clear();
    return feedback.state === "valid" ? feedback.command! : null;
  }
  clear() {
    this.payload = null;
    this.feedback = null;
  }
}
