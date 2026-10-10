import { ApiError } from "../api/client.js";
import { sharedApplicationEvents } from "../api/shared-application-events.js";
import {
  assertApplicationContinuity,
  type ApplicationEvent,
  type FreshnessChange,
} from "../api/application-events.js";
import { createAutomaticRefresh } from "./automatic-refresh.js";
import { FreshnessJournal } from "./freshness-journal.js";
export type FreshnessNotice = {
  changes: readonly FreshnessChange[];
  reset: boolean;
  status: string;
};
export class ApplicationFreshness {
  readonly journal = new FreshnessJournal();
  private lifetime = new AbortController();
  private credential: string | null | undefined;
  private advertised = false;
  private cursor: string | undefined;
  private witnesses = new Map<string, string>();
  private listeners = new Set<(notice: FreshnessNotice) => void>();
  private state = "stopped";
  private ready: Promise<void> = Promise.resolve();
  constructor(
    private fetcher?: typeof fetch,
    private denied: () => void = () => {},
  ) {}
  get healthy() {
    return this.state === "live" || this.state === "renewing";
  }
  get status() {
    return this.state;
  }
  get supported() {
    return this.advertised;
  }
  advertise(supported: boolean) {
    this.advertised = supported;
  }
  listen(listener: (notice: FreshnessNotice) => void) {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }
  private publish(changes: readonly FreshnessChange[] = [], reset = false) {
    this.journal.record({ changes, reset, status: this.state });
    for (const listener of this.listeners)
      listener({ changes, reset, status: this.state });
  }
  async configure(advertised: boolean, token: string | null) {
    if (
      this.credential === token &&
      this.advertised === advertised &&
      !this.lifetime.signal.aborted
    )
      return this.ready;
    this.stop();
    this.credential = token;
    this.advertised = advertised;
    this.cursor = undefined;
    this.witnesses.clear();
    this.journal.clear();
    this.lifetime = new AbortController();
    this.state = advertised ? "connecting" : "legacy";
    this.publish();
    if (!advertised) return;
    let resolve!: () => void;
    this.ready = new Promise<void>((done) => {
      resolve = done;
    });
    void this.follow(this.lifetime.signal, token, resolve);
    return this.ready;
  }
  private accept(event: ApplicationEvent, id: string) {
    assertApplicationContinuity(this.cursor, event, id);
    const reset =
      event.type === "reset" ||
      (event.type === "changed" &&
        event.change.repository === null &&
        event.change.domain === "repository");
    const captured =
      event.type === "changed" ? [event.change] : event.witnesses;
    const changes: FreshnessChange[] = [];
    const initial = this.cursor === undefined;
    if (reset) this.witnesses.clear();
    for (const change of captured) {
      const key = `${change.repository ?? "installation"}:${change.domain}`;
      const previous = this.witnesses.get(key);
      this.witnesses.set(key, change.witness);
      if ((event.type === "changed" || !initial) && previous !== change.witness)
        changes.push(change);
    }
    this.state = "live";
    // Consumers synchronously record dirtiness and invalidate aliases before continuity advances.
    this.publish(changes, reset);
    this.cursor = reset ? undefined : id;
  }
  private async follow(
    signal: AbortSignal,
    token: string | null,
    ready: () => void,
  ) {
    let failures = 0;
    while (!signal.aborted) {
      let reset = false,
        live = false;
      const started = Date.now();
      try {
        await sharedApplicationEvents(
          token,
          this.cursor,
          signal,
          (event, id) => {
            if (signal.aborted) return;
            this.accept(event, id);
            live = true;
            reset ||= event.type === "reset";
            failures = 0;
            ready();
          },
          this.fetcher,
        );
        if (signal.aborted) return;
        if (reset || (live && Date.now() - started >= 290000)) {
          this.state = "renewing";
          this.publish();
          continue;
        }
      } catch (cause) {
        if (signal.aborted) return;
        if (live && Date.now() - started >= 290000) {
          this.state = "renewing";
          this.publish();
          continue;
        }
        if (cause instanceof ApiError && cause.code === "access_denied") {
          this.stop();
          ready();
          this.denied();
          return;
        }
      }
      this.state = "offline";
      this.publish();
      ready();
      const delay = [5000, 10000, 20000, 30000][Math.min(failures++, 3)]!;
      await this.wait(delay * (0.9 + Math.random() * 0.2), signal);
    }
  }
  private wait(delay: number, signal: AbortSignal) {
    return new Promise<void>((resolve) => {
      const done = () => {
        clearTimeout(timer);
        signal.removeEventListener("abort", done);
        resolve();
      };
      const timer = setTimeout(done, delay);
      signal.addEventListener("abort", done, { once: true });
      if (signal.aborted) done();
    });
  }
  watch(options: {
    matches(change: FreshnessChange): boolean;
    active?(): boolean;
    refresh(): Promise<boolean>;
    initial?: boolean;
    reset?(): void;
    signal?: AbortSignal;
  }) {
    const abort = new AbortController();
    const automatic = createAutomaticRefresh({
      active: () =>
        (!this.supported || this.healthy) && (options.active?.() ?? true),
      refresh: options.refresh,

      report: () => {},
      signal: abort.signal,
    });
    const unlisten = this.listen((notice) => {
      automatic.reschedule();
      if (notice.reset) options.reset?.();
      const matched = notice.changes.map(options.matches).some(Boolean);
      if (notice.reset || matched) automatic.request();
    });
    const stop = () => {
      abort.abort();
      unlisten();
      options.signal?.removeEventListener("abort", stop);
    };
    options.signal?.addEventListener("abort", stop, { once: true });
    if (options.signal?.aborted) stop();
    else if (options.initial !== false) automatic.request();
    return { request: automatic.request, stop };
  }
  stop() {
    this.lifetime.abort();
    this.state = "stopped";
  }
  destroy() {
    this.stop();
    this.listeners.clear();
    this.witnesses.clear();
    this.cursor = undefined;
  }
}
