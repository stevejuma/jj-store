import type { WorkspaceRead } from "./workspace-browser.svelte.js";
/** Bounded session-only LRU of observations, never source authority. */
export class WorkspaceReadCache extends Map<string, WorkspaceRead> {
  remember(key: string, value: WorkspaceRead) {
    this.delete(key);
    this.set(key, value);
    // Session-only LRU, bounded by entries and UTF-16 text storage.
    let bytes = 0;
    for (const entry of this.values())
      bytes +=
        2 *
        ((entry.file?.contents?.length ?? 0) +
          (entry.file?.before?.length ?? 0));
    while (this.size > 24 || bytes > 8 * 1024 * 1024) {
      const oldest = this.keys().next().value!;
      const entry = this.get(oldest)!;
      bytes -=
        2 *
        ((entry.file?.contents?.length ?? 0) +
          (entry.file?.before?.length ?? 0));
      this.delete(oldest);
    }
  }
}
