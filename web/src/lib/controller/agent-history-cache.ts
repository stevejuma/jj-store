import { pretty } from "../api/client.js";
import type { AgentEvent } from "./agent-execution-types.js";
/** Bounded session memory only. A changed publication boundary requires recapture. */
export class AgentHistoryCache {
  private entries = new Map<
    string,
    { publication: string; events: AgentEvent[]; size: number }
  >();
  get(id: string, publication: string) {
    const entry = this.entries.get(id);
    if (!entry || entry.publication !== publication) return undefined;
    this.entries.delete(id);
    this.entries.set(id, entry);
    return entry.events;
  }
  set(id: string, publication: string, events: AgentEvent[]) {
    this.entries.delete(id);
    const size = pretty(events).length;
    if (size > 8 * 1024 * 1024) return;
    this.entries.set(id, { publication, events, size });
    while (
      this.entries.size > 8 ||
      [...this.entries.values()].reduce((sum, entry) => sum + entry.size, 0) >
        8 * 1024 * 1024
    )
      this.entries.delete(this.entries.keys().next().value!);
  }
  clear() {
    this.entries.clear();
  }
}
