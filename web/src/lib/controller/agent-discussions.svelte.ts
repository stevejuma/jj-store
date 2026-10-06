import { Client, type Model } from "../api/client.js";
import { HttpThreadsDataSource } from "./http-threads-data-source.js";
import { createStoreQueryClient } from "./store-query-client.js";
import { createThreadsController } from "./production-threads.svelte.js";
import { initialStoreRoute } from "./store-route-history.svelte.js";
import type { AgentExecution } from "./agent-execution.svelte.js";
import type { ReactionTarget } from "./thread-types.js";
type Feedback = Model<"AgentExecutionFeedback">;

/** Repository-owned feedback with installation-owned drafts and exact receipt recovery. */
export class AgentDiscussions {
  values = $state<Record<string, Feedback>>({});
  errors = $state<Record<string, string>>({});
  drafts = $state<Record<string, string>>({});
  recovered = $state<Record<string, number>>({});
  #loading = new Set<string>();
  #awaiting = $state<
    Record<
      string,
      | {
          repository: string;
          request: string;
          thread: string;
          creating: boolean;
          draft: string;
        }
      | undefined
    >
  >({});
  #readers = new Map<string, ReturnType<typeof this.createReader>>();
  constructor(
    private manager: AgentExecution,
    private fetcher?: typeof fetch,
    private principal: (repository: string) => string = () => "",
  ) {}
  reader(repository: string) {
    let reader = this.#readers.get(repository);
    if (!reader) {
      reader = this.createReader(repository);
      this.#readers.set(repository, reader);
    }
    return reader.threads;
  }
  private createReader(repository: string) {
    const client = new Client(repository, null, () => {}, this.fetcher);
    const queries = createStoreQueryClient();
    let active = true;
    let route = initialStoreRoute();
    const source = new HttpThreadsDataSource(
      client,
      queries,
      { repository, profile: "native", session: crypto.randomUUID() },
      () => active,
    );
    const threads = createThreadsController({
      source: () => source,
      repository: () => repository,
      principal: () => this.principal(repository),
      route: () => route,
      select: (id) => {
        route = { ...route, threadId: id };
      },
    });
    return {
      threads,
      async invalidate() {
        await source.invalidate("reactions");
        threads.invalidateReactions();
      },
      close() {
        active = false;
        threads.clear();
        client.close();
        void queries.cancelQueries();
        queries.clear();
      },
    };
  }
  async load(id: string) {
    if (this.#loading.has(id)) return;
    this.#loading.add(id);
    try {
      const value = await this.manager.client.request<Feedback>(
        `agents/feedback?session=${encodeURIComponent(id)}`,
      );
      if (!Array.isArray(value.messages))
        throw new Error("Invalid recorded message feedback");
      this.values = { ...this.values, [id]: value };
      this.errors = { ...this.errors, [id]: "" };
      const session = this.manager.value?.sessions.find((s) => s.id === id);
      if (session) {
        this.reader(session.repository);
        await this.#readers.get(session.repository)?.invalidate();
      }
    } catch (cause) {
      this.errors = {
        ...this.errors,
        [id]: cause instanceof Error ? cause.message : String(cause),
      };
    } finally {
      this.#loading.delete(id);
    }
  }
  message(id: string, row: string) {
    return this.values[id]?.messages.find((m) => m.row_id === row);
  }
  async save(id: string, row: string, body: string) {
    const session = this.manager.value?.sessions.find((s) => s.id === id);
    const message = this.message(id, row);
    if (this.values[id]?.write_unavailable)
      throw new Error(this.values[id].write_unavailable!);
    if (!session || !message || this.errors[id])
      throw new Error(
        "Waiting for this message to be published. Your reply draft is retained.",
      );
    const threads = this.reader(session.repository);
    const key = `${id}:${row}`,
      draft = this.drafts[key];
    try {
      if (message.threads[0]) {
        await threads.open(message.threads[0]);
        await threads.send(body);
      } else
        await threads.create(
          `Replies to ${session.title}`,
          body,
          message.target,
        );
      if (this.drafts[key] === draft) this.drafts[key] = "";
      await this.load(id);
    } catch (cause) {
      const pending = threads.pending;
      if (pending?.kind === "threads" && "thread_id" in pending.request.command)
        this.#awaiting[`${id}:${row}`] = {
          repository: session.repository,
          request: pending.request.request_id,
          thread: pending.request.command.thread_id,
          creating: pending.request.command.type === "create_agent_discussion",
          draft,
        };
      throw cause;
    }
  }
  awaiting(id: string, row: string) {
    return !!this.#awaiting[`${id}:${row}`];
  }
  async recover(id: string, row: string, retry = false) {
    const key = `${id}:${row}`,
      awaiting = this.#awaiting[key];
    if (!awaiting) return;
    const threads = this.reader(awaiting.repository);
    if (retry) await threads.retryPending();
    else await threads.checkPending();
    if (
      !threads.pending &&
      (awaiting.creating
        ? threads.postedThreadId === awaiting.thread
        : threads.committedRequestId === awaiting.request)
    ) {
      if (this.drafts[key] === awaiting.draft) this.drafts[key] = "";
      this.recovered[key] = (this.recovered[key] ?? 0) + 1;
      delete this.#awaiting[key];
    }
    await this.load(id);
  }
  authorName(repository: string, actor: string) {
    return actor === this.principal(repository)
      ? "You"
      : `Contributor ${actor.slice(0, 8)}`;
  }
  toggle(
    repository: string,
    target: ReactionTarget,
    emoji: string,
    custom?: { shortcode: string; url: string },
    fromBar = false,
  ) {
    const threads = this.reader(repository);
    if (threads.busy || threads.pending) return Promise.resolve();
    return threads.toggle(target, emoji, custom, fromBar).catch(() => {});
  }
  destroy() {
    for (const reader of this.#readers.values()) reader.close();
    this.#readers.clear();
  }
}
