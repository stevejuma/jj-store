import type { LocalClient } from "../api/local-client.js";
import { Client, type Model } from "../api/client.js";
import { HttpThreadsDataSource } from "./http-threads-data-source.js";
import { LocalInboxThreads } from "./local-inbox-threads.js";
import { createStoreQueryClient } from "./store-query-client.js";
import { createThreadsController } from "./production-threads.svelte.js";
import { initialStoreRoute } from "./store-route-history.svelte.js";
import { watchAgentDiscussion } from "./agent-discussion-refresh.js";
import type { ApplicationFreshness } from "./application-freshness.js";
import type { ReactionTarget } from "./thread-types.js";
type Feedback = Model<"AgentExecutionFeedback">;

/** Repository-owned feedback with installation-owned drafts and exact receipt recovery. */
export class AgentDiscussions {
  values = $state<Record<string, Feedback>>({});
  errors = $state<Record<string, string>>({});
  drafts = $state<Record<string, string>>({});
  recovered = $state<Record<string, number>>({});
  #loading = new Set<string>();
  #alive = true;
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
  #profiles = $state<Record<string, string>>({});
  constructor(
    private client: LocalClient,
    private sessions: () => readonly {
      id: string;
      repository: string;
      title: string;
    }[],
    private fetcher?: typeof fetch,
    private principal: (repository: string) => string = () => "",
    private freshness?: ApplicationFreshness,
  ) {}
  reader(repository: string) {
    // Every rendered message tracks profile adoption, including readers created
    // while the asynchronous local selection is being prepared.
    const profile = this.#profiles[repository] ?? "native";
    let reader = this.#readers.get(repository);
    if (!reader) {
      reader = this.createReader(repository, profile);
      this.#readers.set(repository, reader);
    }
    return reader.threads;
  }
  private createReader(repository: string, profile = "native") {
    const client = new Client(repository, null, () => {}, this.fetcher);
    const queries = createStoreQueryClient();
    let active = true;
    let route = initialStoreRoute();
    const local =
      profile === "local_jj_git"
        ? new LocalInboxThreads(repository, this.fetcher)
        : undefined;
    const source =
      local ??
      new HttpThreadsDataSource(
        client,
        queries,
        { repository, profile: "native", session: crypto.randomUUID() },
        () => active,
      );
    const threads = createThreadsController({
      source: () => source,
      repository: () => repository,
      principal: () => local?.actor ?? this.principal(repository),
      route: () => route,
      select: (id) => {
        route = { ...route, threadId: id };
      },
    });
    return {
      threads,
      profile,
      principal: () => local?.actor ?? this.principal(repository),
      prepare: () => local?.prepare() ?? Promise.resolve(),
      async invalidate() {
        await source.invalidate("reactions");
        threads.invalidateReactions();
      },
      close() {
        local?.close();
        active = false;
        threads.clear();
        client.close();
        void queries.cancelQueries();
        queries.clear();
      },
    };
  }
  async load(id: string) {
    if (!this.#alive) return true;
    if (this.#loading.has(id)) return false;
    this.#loading.add(id);
    try {
      const value = await this.client.request<Feedback>(
        `agents/feedback?session=${encodeURIComponent(id)}`,
      );
      if (!this.#alive) return;
      if (!Array.isArray(value.messages))
        throw new Error("Invalid recorded message feedback");
      const session = this.sessions().find((s) => s.id === id);
      if (session) {
        const profile = value.profile ?? "native";
        if (!["native", "local_jj_git"].includes(profile))
          throw new Error("Unknown feedback repository profile");
        const prior = this.#readers.get(session.repository);
        if (prior?.profile !== profile) {
          if (prior?.threads.pending)
            throw new Error(
              "Check the original reply status before changing repository profile",
            );
          const reader = this.createReader(session.repository, profile);
          try {
            await reader.prepare();
          } catch (cause) {
            reader.close();
            throw cause;
          }
          if (!this.#alive) {
            reader.close();
            return;
          }
          const current = this.#readers.get(session.repository);
          if (current?.threads.pending) {
            reader.close();
            throw new Error(
              "Check the original reply status before changing repository profile",
            );
          }
          current?.close();
          this.#readers.set(session.repository, reader);
          this.#profiles[session.repository] = profile;
        }
        this.reader(session.repository);
        await this.#readers.get(session.repository)?.invalidate();
      }
      if (!this.#alive) return;
      this.values = { ...this.values, [id]: value };
      this.errors = { ...this.errors, [id]: "" };
      return true;
    } catch (cause) {
      if (!this.#alive) return;
      this.errors = {
        ...this.errors,
        [id]: cause instanceof Error ? cause.message : String(cause),
      };
      return false;
    } finally {
      this.#loading.delete(id);
    }
  }
  watch(id: string, repository: string) {
    return watchAgentDiscussion(
      () => this.load(id),
      repository,
      this.freshness,
    );
  }
  actor(repository: string) {
    return (
      this.#readers.get(repository)?.principal() ?? this.principal(repository)
    );
  }
  contexts = $state<
    Record<
      string,
      { reply?: string; edit?: string; revision?: string; thread?: string }
    >
  >({});
  message(id: string, row: string) {
    return this.values[id]?.messages.find((m) => m.row_id === row);
  }
  async save(id: string, row: string, body: string) {
    const session = this.sessions().find((s) => s.id === id);
    const message = this.message(id, row);
    if (this.values[id]?.write_unavailable)
      throw new Error(this.values[id].write_unavailable!);
    if (!session || !message)
      throw new Error(
        "Waiting for this message to be published. Your reply draft is retained.",
      );
    const threads = this.reader(session.repository);
    const key = `${id}:${row}`,
      draft = this.drafts[key];
    try {
      const context = this.contexts[key];
      const thread = context?.thread ?? message.threads[0];
      if (thread) {
        if (!message.threads.includes(thread))
          throw new Error("The original discussion is unavailable.");
        await threads.open(thread);
        const target = context?.edit ?? context?.reply;
        if (threads.detail?.thread.value.thread_id !== thread)
          throw new Error(
            "The discussion could not be loaded. Retry your reply.",
          );
        if (
          target &&
          threads.projection.targets[target]?.type === "human_message"
        ) {
          const current = threads.projection.targets[target];
          if (
            current.type !== "human_message" ||
            current.revision.id !== context?.revision
          )
            throw new Error(
              "The reply changed. Reopen its reply or edit action to continue.",
            );
        } else if (target)
          throw new Error("The original reply is unavailable.");
        await threads.send(body, context?.reply, context?.edit);
      } else
        await threads.create(
          `Replies to ${session.title}`,
          body,
          message.target,
        );
      if (this.drafts[key] === draft) {
        this.drafts[key] = "";
        delete this.contexts[key];
      }
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
      delete this.contexts[key];
      delete this.#awaiting[key];
    }
    await this.load(id);
  }
  authorName(repository: string, actor: string) {
    const local = this.#readers.get(repository);
    return actor ===
      (local?.profile === "local_jj_git"
        ? local.principal()
        : this.principal(repository))
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
    this.#alive = false;
    for (const reader of this.#readers.values()) reader.close();
    this.#readers.clear();
  }
}
