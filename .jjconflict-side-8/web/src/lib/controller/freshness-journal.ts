import type { FreshnessNotice } from "./application-freshness.js";
import type { FreshnessChange } from "../api/application-events.js";
/** Retains bounded settlement hints so completion between command and observation cannot be missed. */
export class FreshnessJournal {
  revision = 0;
  private reset = 0;
  private records = new Map<
    string,
    { revision: number; change: FreshnessChange }
  >();
  record(notice: FreshnessNotice) {
    if (notice.reset) this.reset = ++this.revision;
    for (const change of notice.changes) {
      const key = `${change.repository}:${change.domain}:${change.identity ?? ""}`;
      this.records.delete(key);
      this.records.set(key, { revision: ++this.revision, change });
      if (this.records.size > 1024) {
        this.records.delete(this.records.keys().next().value!);
        this.reset = this.revision;
      }
    }
  }
  clear() {
    this.records.clear();
    this.reset = ++this.revision;
  }
  wait(options: {
    after: number;
    matches(change: FreshnessChange): boolean;
    healthy(): boolean;
    listen(listener: (notice: FreshnessNotice) => void): () => void;
    signal: AbortSignal;
    delay: number;
  }) {
    return new Promise<void>((resolve) => {
      let timer: ReturnType<typeof setTimeout> | undefined;
      let unlisten = () => {};
      const dirty = () =>
        this.reset > options.after ||
        [...this.records.values()].some(
          (record) =>
            record.revision > options.after && options.matches(record.change),
        );
      const visible = () => typeof document === "undefined" || !document.hidden;
      const done = () => {
        clearTimeout(timer);
        unlisten();
        options.signal.removeEventListener("abort", done);
        if (typeof document !== "undefined")
          document.removeEventListener("visibilitychange", update);
        resolve();
      };
      const update = () => {
        if (options.signal.aborted || (visible() && dirty())) return done();
        clearTimeout(timer);
        if (visible() && !options.healthy())
          timer = setTimeout(done, options.delay);
      };
      unlisten = options.listen(update);
      options.signal.addEventListener("abort", done, { once: true });
      if (typeof document !== "undefined")
        document.addEventListener("visibilitychange", update);
      update();
    });
  }
}
