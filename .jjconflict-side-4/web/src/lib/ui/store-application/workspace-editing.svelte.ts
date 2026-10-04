import { getContext, onDestroy } from "svelte";
import {
  workspaceNavigationKey,
  type WorkspaceNavigationGuard,
} from "./workspace-navigation.js";

export function guardWorkspaceEditing(editing: () => boolean) {
  const guard = getContext<WorkspaceNavigationGuard | undefined>(
    workspaceNavigationKey,
  );
  $effect(() => {
    if (guard) guard.editing = editing();
  });
  onDestroy(() => {
    if (guard) guard.editing = false;
  });
}
