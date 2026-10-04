import type { ApplicationFreshness } from "./application-freshness.js";
import type { CaptureManagementState } from "./capture-types.js";
export function captureConfigurationObserver(
  state: CaptureManagementState,
  observe: (signal?: AbortSignal) => Promise<void>,
  freshness?: ApplicationFreshness,
) {
  let revision = 0,
    accepted = -1;
  const close = freshness?.listen((notice) => {
    if (
      notice.reset ||
      notice.changes.some((change) => change.domain === "configuration")
    )
      revision++;
  });
  return {
    close,
    async observe(signal?: AbortSignal) {
      if (freshness && accepted === revision && state.settings.value) return;
      const captured = revision;
      await observe(signal);
      if (!state.settings.error) accepted = captured;
    },
  };
}
