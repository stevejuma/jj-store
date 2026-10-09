import type { ApplicationFreshness } from "../controller/application-freshness.js";
import type { FreshnessChange } from "../api/application-events.js";
import { watchAgentChanges } from "../controller/agent-freshness.js";
import type { WorkspaceBrowser } from "./workspace-browser.svelte.js";
export function watchWorkspace(
  browser: WorkspaceBrowser,
  freshness: ApplicationFreshness | undefined,
  active: () => boolean,
) {
  const matches = (change: FreshnessChange) => {
    const relevant =
      ["code", "workspaces", "repository", "discovery"].includes(
        change.domain,
      ) &&
      browser.bindings.some(
        (b) => !change.repository || b.repository === change.repository,
      );
    // Ready/reset witnesses coalesce by repository/domain, not workspace identity.
    if (relevant) browser.invalidate(change.repository ?? undefined);
    return relevant;
  };
  // Invalidation happens synchronously even when this session is not visible.
  const reset = freshness?.listen((notice) => {
    if (notice.reset) browser.invalidate();
  });
  const watch = watchAgentChanges(freshness, {
    active,
    matches,
    refresh: async () => {
      if (!freshness?.healthy) browser.invalidate();
      return browser.refreshVisible();
    },
  });
  return {
    request: watch.request,
    stop() {
      reset?.();
      watch.stop();
    },
  };
}
