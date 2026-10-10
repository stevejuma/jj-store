import { isAgentCommandRefusal } from "./agent-execution-errors.js";
import type { paths } from "../api/generated.js";
import type { Model } from "../api/client.js";
import type { LocalClient } from "../api/local-client.js";
import type { ApplicationFreshness } from "./application-freshness.js";
import { RetainedReads } from "./retained-reads.js";
export type MetadataQuery = NonNullable<
  paths["/v1/local/agents/metadata"]["get"]["parameters"]["query"]
>;
export type MetadataAction = Command["action"];
export type MetadataView = Model<"ConversationMetadataView">;
type Command = NonNullable<
  paths["/v1/local/agents/metadata"]["post"]["requestBody"]
>["content"]["application/json"];
type Job = Model<"ConversationMetadataJob">;
export type MetadataEntry = {
  value: MetadataView | null;
  loading: boolean;
  submitting: boolean;
  error: string;
  rename: string;
  pending: Command | null;
  job: Job | null;
};
export function metadataKey(q: MetadataQuery) {
  return `${q.repository ?? "live"}:${q.session}`;
}
export class ConversationMetadata {
  entries = $state<Record<string, MetadataEntry>>({});
  details = $state<Record<string, boolean>>({});
  readonly #reads = new Map<string, AbortController>();
  readonly #jobs = new Map<string, AbortController>();
  readonly #queries = new Map<string, MetadataQuery>();
  readonly #retained = new RetainedReads((key) =>
    this.read(this.#queries.get(key)!),
  );
  readonly #unlisten?: () => void;
  constructor(
    readonly client: LocalClient,
    readonly changed: () => void,
    readonly freshness?: ApplicationFreshness,
  ) {
    this.#unlisten = freshness?.listen(({ changes, reset }) => {
      for (const [key, query] of this.#queries)
        if (reset || changes.some((c) => this.matches(query, c)))
          this.#retained.invalidate(key);
    });
  }
  private matches(
    q: MetadataQuery,
    c: import("../api/application-events.js").FreshnessChange,
  ) {
    const repository = q.repository ?? this.entry(q).value?.repository;
    return (
      ["threads", "agents", "repository"].includes(c.domain) &&
      (c.repository === null || !repository || c.repository === repository)
    );
  }
  ensure(q: MetadataQuery) {
    this.#queries.set(metadataKey(q), q);
    return this.#retained.ensure(metadataKey(q));
  }
  watch(q: MetadataQuery) {
    let active = true;
    const update = async () => {
      const accepted = await this.ensure(q);
      if (
        accepted !== false &&
        active &&
        ["prepared", "generating", "publishing"].includes(
          this.entry(q).job?.state ?? "",
        )
      )
        void this.followJob(q);
      return accepted !== false;
    };
    const watch = this.freshness?.watch({
      matches: (c) => this.matches(q, c),
      refresh: update,
    });
    if (!watch) void update();
    return () => {
      active = false;
      watch?.stop();
      this.leave(q);
    };
  }
  entry(q: MetadataQuery) {
    const key = metadataKey(q);
    return (this.entries[key] ??= {
      value: null,
      loading: false,
      submitting: false,
      error: "",
      rename: "",
      pending: null,
      job: null,
    });
  }
  load(q: MetadataQuery) {
    this.#reads.get(metadataKey(q))?.abort();
    this.#retained.release(metadataKey(q));
    this.#queries.set(metadataKey(q), q);
    return this.#retained.refresh(metadataKey(q));
  }
  private async read(q: MetadataQuery) {
    const key = metadataKey(q),
      entry = this.entry(q);
    this.#reads.get(key)?.abort();
    const abort = new AbortController();
    this.#reads.set(key, abort);
    entry.loading = true;
    const params = new URLSearchParams({ session: q.session });
    if (q.repository) params.set("repository", q.repository);
    try {
      const value = await this.client.request<MetadataView>(
        `agents/metadata?${params}`,
        undefined,
        abort.signal,
      );
      if (abort.signal.aborted) return false;
      entry.value = value;
      entry.job = value.job ?? entry.job;
      entry.error = entry.job?.error ?? "";
      if (!entry.pending && entry.job?.state === "outcome_unknown")
        entry.pending = entry.job.original_command
          ? { ...entry.job.original_command, version: 1 }
          : null;
      return true;
    } catch (e) {
      if (!abort.signal.aborted)
        entry.error =
          e instanceof Error ? e.message : "Conversation details unavailable";
      return false;
    } finally {
      if (this.#reads.get(key) === abort) {
        entry.loading = false;
        this.#reads.delete(key);
      }
    }
  }
  async command(q: MetadataQuery, action: MetadataAction) {
    const entry = this.entry(q);
    if (
      !entry.value?.writable ||
      entry.submitting ||
      (entry.pending &&
        action.action !== "cancel" &&
        action.action !== "publish")
    )
      return;
    const command: Command = {
      version: 1,
      request_id: crypto.randomUUID().replaceAll("-", ""),
      query: q,
      expected: entry.value.revision ?? null,
      action,
    };
    entry.pending = command;
    await this.submit(q, command);
  }
  async retry(q: MetadataQuery) {
    const entry = this.entry(q);
    if (entry.submitting) return;
    if (entry.pending) await this.submit(q, entry.pending);
    else await this.followJob(q);
  }
  private async submit(q: MetadataQuery, command: Command) {
    const entry = this.entry(q);
    if (entry.submitting) return;
    entry.submitting = true;
    entry.error = "";
    try {
      entry.job = await this.client.request<Job>("agents/metadata", command);
      await this.followJob(q);
    } catch (e) {
      this.#retained.invalidate(metadataKey(q));
      entry.error =
        e instanceof Error
          ? e.message
          : "Metadata request outcome unknown. Check the retained request.";
      if (isAgentCommandRefusal(e)) entry.pending = null;
    } finally {
      entry.submitting = false;
    }
  }
  async followJob(q: MetadataQuery) {
    const key = metadataKey(q),
      entry = this.entry(q);
    this.#jobs.get(key)?.abort();
    const abort = new AbortController();
    const id = entry.pending?.request_id ?? entry.job?.request_id;
    if (!id) return;
    this.#jobs.set(key, abort);
    const deadline = setTimeout(
      () =>
        abort.abort(
          new DOMException(
            "Metadata generation is still running. Its request is retained.",
            "TimeoutError",
          ),
        ),
      150_000,
    );
    try {
      while (!abort.signal.aborted) {
        const observed = this.freshness?.journal.revision ?? 0;
        const job = await this.client.request<Job>(
          `agents/metadata/job?request=${encodeURIComponent(id)}`,
          undefined,
          abort.signal,
        );
        if (abort.signal.aborted) return;
        entry.job = job;
        if (!["prepared", "generating", "publishing"].includes(job.state)) {
          if (job.state === "completed") {
            if (entry.pending?.request_id === job.request_id) {
              if (
                entry.pending.action.action === "rename" &&
                entry.pending.action.title === entry.rename.trim()
              )
                entry.rename = "";
              entry.pending = null;
            }
            this.changed();
          } else {
            entry.error = job.error ?? job.state;
            if (
              job.state !== "outcome_unknown" &&
              entry.pending?.request_id === job.request_id
            )
              entry.pending = null;
          }
          await this.load(q);
          return;
        }
        if (!this.freshness) return;
        await this.freshness.journal.wait({
          after: observed,
          matches: (c) => c.domain === "agent_execution" || this.matches(q, c),
          healthy: () => true,
          listen: (listener) => this.freshness!.listen(listener),
          signal: abort.signal,
          delay: 0,
        });
      }
    } catch (e) {
      if (!abort.signal.aborted)
        entry.error =
          e instanceof Error ? e.message : "Metadata status unavailable";
    } finally {
      clearTimeout(deadline);
      if (this.#jobs.get(key) === abort) this.#jobs.delete(key);
    }
  }
  leave(q: MetadataQuery) {
    const key = metadataKey(q);
    this.#reads.get(key)?.abort();
    if (this.#reads.has(key)) this.#retained.release(key);
    this.#jobs.get(key)?.abort();
  }
  destroy() {
    this.#unlisten?.();
    this.#retained.clear();
    for (const abort of [...this.#reads.values(), ...this.#jobs.values()])
      abort.abort();
    this.#reads.clear();
    this.#jobs.clear();
  }
}
