import { isAgentCommandRefusal } from "./agent-execution-errors.js";
import type { paths } from "../api/generated.js";
import type { Model } from "../api/client.js";
import type { LocalClient } from "../api/local-client.js";
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
  readonly #polls = new Map<string, AbortController>();
  constructor(
    readonly client: LocalClient,
    readonly changed: () => void,
  ) {}
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
  async load(q: MetadataQuery) {
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
      if (abort.signal.aborted) return;
      entry.value = value;
      entry.job = value.job ?? entry.job;
      entry.error = entry.job?.error ?? "";
      if (!entry.pending && entry.job?.state === "outcome_unknown")
        entry.pending = entry.job.original_command
          ? { ...entry.job.original_command, version: 1 }
          : null;
    } catch (e) {
      if (!abort.signal.aborted)
        entry.error =
          e instanceof Error ? e.message : "Conversation details unavailable";
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
    else await this.poll(q);
  }
  private async submit(q: MetadataQuery, command: Command) {
    const entry = this.entry(q);
    if (entry.submitting) return;
    entry.submitting = true;
    entry.error = "";
    try {
      entry.job = await this.client.request<Job>("agents/metadata", command);
      await this.poll(q);
    } catch (e) {
      entry.error =
        e instanceof Error
          ? e.message
          : "Metadata request outcome unknown. Check the retained request.";
      if (isAgentCommandRefusal(e)) entry.pending = null;
    } finally {
      entry.submitting = false;
    }
  }
  async poll(q: MetadataQuery) {
    const key = metadataKey(q),
      entry = this.entry(q);
    this.#polls.get(key)?.abort();
    const abort = new AbortController();
    this.#polls.set(key, abort);
    const id = entry.pending?.request_id ?? entry.job?.request_id;
    if (!id) return;
    const until = Date.now() + 150_000;
    try {
      while (!abort.signal.aborted && Date.now() < until) {
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
        await new Promise<void>((resolve) => {
          const finish = () => {
            clearTimeout(timer);
            abort.signal.removeEventListener("abort", finish);
            resolve();
          };
          const timer = setTimeout(finish, 750);
          abort.signal.addEventListener("abort", finish, { once: true });
        });
      }
    } catch (e) {
      if (!abort.signal.aborted)
        entry.error =
          e instanceof Error ? e.message : "Metadata status unavailable";
    } finally {
      if (this.#polls.get(key) === abort) this.#polls.delete(key);
    }
  }
  leave(q: MetadataQuery) {
    const key = metadataKey(q);
    this.#reads.get(key)?.abort();
    this.#polls.get(key)?.abort();
  }
  destroy() {
    for (const abort of [...this.#reads.values(), ...this.#polls.values()])
      abort.abort();
    this.#reads.clear();
    this.#polls.clear();
  }
}
