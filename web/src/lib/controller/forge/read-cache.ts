import type { ForgeAction, ForgeResponse } from "./types.js";
/** Repository-owner memory only. Never stores previews, effects or recovery proofs. */
export class ForgeReadCache {
  private entries = new Map<string, ForgeResponse>();
  private bytes = 0;
  clear() {
    this.entries.clear();
    this.bytes = 0;
  }
  key(action: ForgeAction, revision: string) {
    if (action.kind === "pull_request")
      return `pr:${action.number}:${revision}`;
    if (action.kind !== "pr") return undefined;
    const a = action.action;
    if (a.kind === "checks") return `checks:${action.number}:${revision}`;
    if (a.kind === "jobs") return `jobs:${action.number}:${revision}:${a.run}`;
    if (a.kind === "logs")
      return `logs:${action.number}:${revision}:${a.run}:${a.job}`;
    return undefined;
  }
  get(key: string | undefined) {
    const entry = key ? this.entries.get(key) : undefined;
    return entry ? structuredClone(entry) : undefined;
  }
  put(key: string | undefined, value: ForgeResponse) {
    if (!key) return;
    // Journal/guard-bearing fields must always come from a fresh backend response.
    const entry = structuredClone(value);
    entry.plan =
      entry.receipt =
      entry.restack =
      entry.operation =
      entry.merge =
        null;
    entry.merges = {};
    const bytes =
      JSON.stringify(entry, (_, v) => (typeof v === "bigint" ? String(v) : v))
        .length * 2;
    if (bytes > 4 * 1024 * 1024) return;
    if (this.entries.size >= 64 || this.bytes + bytes > 4 * 1024 * 1024)
      this.clear();
    this.entries.set(key, entry);
    this.bytes += bytes;
  }
}
