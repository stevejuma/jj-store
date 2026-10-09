import type { WorkspaceRead } from "./workspace-browser.svelte.js";
export class Listing {
  value = $state<WorkspaceRead>();
  loading = $state(false);
  error = $state("");
  selected = $state("");
  collapsed = $state<ReadonlySet<string>>(new Set());
  scroll = 0;
  generation = 0;
  dirty = true;
  abort?: AbortController;
}
