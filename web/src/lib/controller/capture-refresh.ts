import { createAutomaticRefresh } from "./automatic-refresh.js";

export function createCaptureRefresh(options: {
  available(): boolean;
  refresh(interests?: ReadonlySet<string>): Promise<boolean>;
  cancel(): void;
  freshness?: import("./application-freshness.js").ApplicationFreshness;
  configurationChanged?(): void;
}) {
  let revision = 0,
    interests: Set<string> | undefined;
  let lifetime: AbortController | null = null;
  let automatic: ReturnType<typeof createAutomaticRefresh> | null = null;
  let subscription: ReturnType<
    import("./application-freshness.js").ApplicationFreshness["watch"]
  > | null = null;
  return {
    setActive(active: boolean) {
      if (active && options.available()) {
        if (lifetime) return;
        interests = undefined;
        revision++;
        lifetime = new AbortController();
        if (options.freshness) {
          subscription = options.freshness.watch({
            reset: () => {
              interests = undefined;
              revision++;
            },
            matches: (change) => {
              if (change.repository !== null) return false;
              if (!["capture", "configuration"].includes(change.domain))
                return false;
              revision++;
              if (change.domain === "configuration")
                options.configurationChanged?.();
              if (change.identity === "status") {
                interests?.add("status");
                interests?.add("import");
              } else if (change.identity === "sources") {
                interests?.add("sources");
                interests?.add("status");
              } else if (change.identity) interests?.add("import");
              else interests = undefined;
              return true;
            },
            active: options.available,
            refresh: async () => {
              if (!options.freshness!.healthy) interests = undefined;
              const ticket = revision;
              const accepted = await options.refresh(interests);
              if (accepted && ticket === revision) interests = new Set();
              return accepted;
            },
            signal: lifetime.signal,
          });
          return;
        }
        automatic = createAutomaticRefresh({
          signal: lifetime.signal,
          active: options.available,

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
        subscription?.stop();
        subscription = null;
        options.cancel();
      }
    },
    request() {
      interests = undefined;
      revision++;
      automatic?.request();
      subscription?.request();
    },
  };
}
