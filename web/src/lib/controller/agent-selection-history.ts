import type { AgentExecution } from "./agent-execution.svelte.js";
import type { AgentEvent } from "./agent-execution-types.js";
import { AgentHistoryCache } from "./agent-history-cache.js";
import {
  consumeAgentEvents,
  readAgentEventHistory,
} from "./agent-execution-stream.js";
import { watchAgentChanges } from "./agent-freshness.js";
/** Selected inbox ownership is independent from application defaults and drafts. */
export class AgentSelectionHistory {
  private history = new AgentHistoryCache();
  private stream?: AbortController;
  private publication = "unknown";
  private alive = true;
  private reset = false;
  private updates?: ReturnType<typeof watchAgentChanges>;
  constructor(private manager: AgentExecution) {}
  private retainHistory() {
    const m = this.manager;
    if (m.selected && !m.eventsLoading)
      this.history.set(m.selected, this.publication, m.events);
  }
  pause() {
    this.retainHistory();
    this.stream?.abort();
  }
  destroy() {
    this.alive = false;
    this.updates?.stop();
    this.stream?.abort();
    this.history.clear();
  }
  invalidate() {
    this.updates?.request();
  }
  private followSharedFeed() {
    this.updates ??= watchAgentChanges(this.manager.freshness, {
      active: () =>
        this.alive &&
        this.manager.available() &&
        this.manager.open &&
        !!this.manager.selected,
      matches: () => false,
      reset: () => {
        this.history.clear();
        this.reset = true;
      },
      refresh: async () => {
        if (this.manager.eventsLoading) return false;
        const last = Number(this.manager.events.at(-1)?.sequence ?? 0);
        const publication = String(
          this.manager.session?.published_sequence ?? "unknown",
        );
        if (
          !this.reset &&
          publication === this.publication &&
          last >= Number(this.manager.session?.last_sequence ?? 0)
        )
          return true;
        await this.select(this.manager.selected, true);
        return !this.manager.activityError;
      },
    });
  }
  async select(id: string, force = false) {
    if (
      !force &&
      this.manager.selected === id &&
      this.stream &&
      !this.stream.signal.aborted
    )
      return;
    this.retainHistory();
    this.stream?.abort();
    const sameSelection = this.manager.selected === id;
    this.manager.drafts[id] ??= "";
    this.manager.selected = id;
    this.manager.runtimeObservation.notify();
    void this.manager.discussions.ensure(id);
    const publication = String(
      this.manager.session?.published_sequence ?? "unknown",
    );
    const cached = this.reset ? undefined : this.history.get(id, publication);
    if (this.reset) this.history.clear();
    let recapture = !cached && (sameSelection || this.reset);
    if (!sameSelection) this.manager.events = cached ?? [];
    let captured: AgentEvent[] = cached ? [...cached] : [];
    this.reset = false;
    this.manager.eventsLoading = true;
    const abort = (this.stream = new AbortController());
    try {
      const start = Number(captured.at(-1)?.sequence ?? 0);
      const receive = (events: AgentEvent[]) => {
        if (!this.alive || abort.signal.aborted || this.stream !== abort)
          return;
        captured = [...captured, ...events];
        if (!recapture) this.manager.events = captured;
      };
      const after =
        cached &&
        this.manager.session &&
        start >= Number(this.manager.session.last_sequence)
          ? start
          : await readAgentEventHistory(
              this.manager.client,
              id,
              abort.signal,
              receive,
              start,
            );
      if (abort.signal.aborted || !this.alive) return;
      this.manager.events = captured;
      recapture = false;
      this.publication = publication;
      this.manager.activityError = "";
      // A cold route can capture history before installation status arrives.
      // Event commitment is captured authority; do not adopt a later status blindly.
      if (
        publication === "unknown" &&
        this.manager.events.every(
          (event) => typeof event.committed === "boolean",
        )
      )
        this.publication = String(
          [...this.manager.events].reverse().find((event) => event.committed)
            ?.sequence ?? 0,
        );
      this.history.set(id, this.publication, this.manager.events);
      if (this.manager.freshness?.supported) {
        this.followSharedFeed();
        return;
      }
      void consumeAgentEvents(this.manager.client, id, after, abort.signal, {
        alive: () => this.alive,
        receive: (events) => {
          this.manager.activityError = "";
          if (events.length) receive(events);
        },
        failed: (message) => {
          this.manager.activityError = message;
        },
      });
    } catch (cause) {
      if (!abort.signal.aborted) {
        this.reset = true;
        this.manager.activityError = String(cause);
      }
      abort.abort();
    } finally {
      if (this.stream === abort) this.manager.eventsLoading = false;
    }
  }
}
