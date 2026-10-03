import { createAutomaticRefresh } from "./automatic-refresh.js";

export function createCaptureRefresh(options: {
  available(): boolean;
  refresh(): Promise<boolean>;
  cancel(): void;
}) {
  let lifetime: AbortController | null = null;
  let automatic: ReturnType<typeof createAutomaticRefresh> | null = null;
  return {
    setActive(active: boolean) {
      if (active && options.available()) {
        if (lifetime) return;
        lifetime = new AbortController();
        automatic = createAutomaticRefresh({
          signal: lifetime.signal,
          active: options.available,
          poll: async () => true,
          refresh: options.refresh,
          report: () => {},
        });
        const reconnect = () => automatic?.request();
        if (typeof window !== "undefined") {
          window.addEventListener("online", reconnect);
          lifetime.signal.addEventListener(
            "abort",
            () => window.removeEventListener("online", reconnect),
            { once: true },
          );
        }
        automatic.request();
      } else {
        lifetime?.abort();
        lifetime = null;
        automatic = null;
        options.cancel();
      }
    },
    request() {
      automatic?.request();
    },
  };
}
