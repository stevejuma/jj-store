import { acknowledged, refreshChatReceipt } from "./agent-chat-receipts.js";
import type { ApplicationFreshness } from "./application-freshness.js";
import { watchAgentChanges } from "./agent-freshness.js";
import type { Model } from "../api/client.js";
import type * as Chat from "./agent-chat-types.js";
export type {
  ChatOrigin,
  ChatSend,
  ChatAttachment,
} from "./agent-chat-types.js";
import type { AgentExecution } from "./agent-execution.svelte.js";
import type { AgentEvent, AgentOutcome } from "./agent-execution-types.js";
import { isAgentCommandRefusal } from "./agent-execution-errors.js";
import {
  agentMentionToken,
  agentMentionMatches,
} from "../community/agent-mentions.js";
import type { StoreController } from "./contracts.js";
import {
  attach,
  removeAttachment,
  selectRepository,
  selectWorkspace,
  validateWorkspaces,
} from "./agent-chat-selections.js";
import { chatContext, sameChatOrigin } from "./agent-chat-context.js";
const newId = () => crypto.randomUUID().replaceAll("-", "");
/** App-lifetime scoped drafts and exact requests, independent of the selected inbox session. */
export class AgentChat {
  open = $state(false);
  selected = $state("");
  drafts = $state<Record<string, Chat.ChatDraft>>({});
  events = $state<Record<string, AgentEvent[]>>({});
  busy = $state(false);
  error = $state("");
  streamError = $state("");
  pending = $state<Chat.ChatSend | null>(null);
  outcomes = $state<AgentOutcome[]>([]);
  receipts = $state<Record<string, Model<"AgentConversationOutcome">>>({});
  publications = $state<
    Record<string, Model<"AgentConversationOutcome">["comment_publication"]>
  >({});
  #updates: ReturnType<typeof watchAgentChanges>;
  #alive = true;
  #refreshing = false;
  #watching = new Map<string, number>();
  constructor(
    readonly manager: AgentExecution,
    freshness?: ApplicationFreshness,
  ) {
    this.#updates = watchAgentChanges(freshness, {
      active: () =>
        this.#alive &&
        manager.available() &&
        (this.open || this.#watching.size > 0),
      // Status reads adopt session membership before invalidating this reader.
      matches: () => false,
      refresh: () => this.refresh(),
    });
  }
  invalidate() {
    if (this.open || this.#watching.size) this.#updates.request();
  }
  get draft() {
    return this.drafts[this.selected];
  }
  get sessions() {
    return (
      this.manager.value?.sessions.filter(
        (s) => s.conversation_id === this.selected,
      ) ?? []
    );
  }
  // "personal" is the stable device-local alias of the lazily created personal home.
  get audienceKey() {
    const o = this.draft?.origin;
    return o
      ? JSON.stringify([
          "agent-chat",
          o.home ?? "personal",
          o.scope,
          this.selected,
        ])
      : undefined;
  }
  show(
    origin: Chat.ChatOrigin,
    attachments: Chat.ChatAttachment[] = [],
    agentId?: string,
    fresh = false,
  ) {
    const saved =
      !fresh &&
      this.manager.value?.conversations
        ?.filter((c) => sameChatOrigin(c.origin, origin))
        .at(-1);
    const retained =
      !fresh &&
      Object.values(this.drafts)
        .filter((d) => sameChatOrigin(d.origin, origin))
        .at(-1);
    const id = retained ? retained.id : saved ? saved.id : newId();
    this.drafts[id] ??= {
      id,
      origin,
      text: agentId ? `${agentMentionToken(agentId)} ` : "",
      attachments,
    };
    if (this.pending?.conversation_id !== id) this.drafts[id]!.origin = origin;
    this.selected = id;
    this.open = true;
    this.error = "";
    void this.refresh();
  }
  showGeneral(agentId?: string, fresh = true) {
    this.show(
      {
        scope: "personal",
        home: null,
        thread: null,
        agent_target: null,
        context: "General conversation",
      },
      [],
      agentId,
      fresh,
    );
  }
  showContext(controller: StoreController, fresh = false, parent?: string) {
    const context = chatContext(controller, parent);
    this.show(context.origin, context.attachments, undefined, fresh);
    if (this.draft && !this.pending) {
      this.draft.threadCodebase = context.codebase;
      this.draft.allowedRepositories = context.repositories;
      if (context.repositories)
        this.draft.attachments = this.draft.attachments.filter((a) =>
          context.repositories!.includes(a.repository),
        );
    }
  }
  newConversation() {
    if (this.draft && !this.busy && !this.pending)
      this.show(
        this.draft.origin,
        [...this.draft.attachments],
        undefined,
        true,
      );
  }
  close() {
    this.open = false;
  }
  watch(scope: string, home: string | null) {
    const key = JSON.stringify([scope, home]);
    this.#watching.set(key, (this.#watching.get(key) ?? 0) + 1);
    void this.refresh();
    return () => {
      const count = this.#watching.get(key) ?? 1;
      if (count > 1) this.#watching.set(key, count - 1);
      else this.#watching.delete(key);
    };
  }
  attach = (repository: string) => attach(this, repository);
  removeAttachment = (repository: string) => removeAttachment(this, repository);
  selectRepository = (previous: string, repository: string) =>
    selectRepository(this, previous, repository);
  selectWorkspace = (repository: string, targetId: string) =>
    selectWorkspace(this, repository, targetId);
  async send(
    content: string,
    agentIds: readonly string[],
    comments?: NonNullable<Chat.ChatSend["comments"]>,
    requestId?: string,
  ) {
    if (this.busy || this.pending || !this.draft)
      throw new Error("Resolve the pending request before sending again.");
    if (!agentIds.length) throw new Error("Use @ to choose an agent.");
    if (
      agentIds.some(
        (id) => !this.manager.value?.setups.some((a) => a.id === id),
      )
    )
      throw new Error("A tagged agent is no longer available.");
    const text = agentMentionMatches(content)
      .reduceRight(
        (value, match) => value.slice(0, match.start) + value.slice(match.end),
        content,
      )
      .trim();
    if (!text && !comments)
      throw new Error("Write a message after the agent tags.");
    validateWorkspaces(this);
    const body: Chat.ChatSend = {
      version: 1,
      request_id: requestId ?? newId(),
      comments,
      conversation_id: this.selected,
      origin: this.draft.origin,
      thread_codebase: this.draft.threadCodebase,
      attachments: [...this.draft.attachments],
      agent_ids: [...agentIds],
      text,
      max_duration_ms: 3600000,
    };
    this.pending = body;
    await this.dispatch(body);
  }
  async retry() {
    if (this.pending && !this.busy) await this.dispatch(this.pending);
  }
  async dispatch(body: Chat.ChatSend) {
    this.busy = true;
    this.error = "";
    try {
      const outcome = await this.manager.client.request<
        Model<"AgentConversationOutcome">
      >("agents/chat", body);
      this.receipts[body.request_id] = outcome;
      this.outcomes = outcome.agents;
      this.publications[body.request_id] = outcome.comment_publication;
      if (
        outcome.agents.some((a) =>
          ["outcome_unknown", "not_known"].includes(a.state),
        )
      ) {
        this.error =
          "The outcome is unknown. Review the saved work before explicitly continuing.";
        throw new Error(this.error);
      }
      const refused = outcome.agents.find(
        (a) =>
          a.error || ["failed", "cancelled", "timed_out"].includes(a.state),
      );
      if (refused)
        throw new Error(refused.error ?? "The prompt was not accepted.");
      // Admission precedes the harness acknowledgement. Keep the exact request
      // available to the shared composer recovery until every recipient confirms.
      if (outcome.agents.every((a) => acknowledged(a.state)))
        this.pending = null;
      await this.manager.updates.status();
      await this.refresh();
      return outcome;
    } catch (cause) {
      if (isAgentCommandRefusal(cause)) this.pending = null;
      this.error =
        cause instanceof Error ? cause.message : "Chat is unavailable";
      throw cause;
    } finally {
      this.busy = false;
    }
  }
  retainUnknown() {
    this.pending = null;
    this.error =
      "The interrupted request is retained in history. Send a new message only after reviewing the work.";
  }
  async refresh() {
    if ((!this.open && !this.#watching.size) || !this.#alive) return true;
    if (this.#refreshing) return false;
    this.#refreshing = true;
    try {
      const watched = new Set(
        (this.manager.value?.conversations ?? [])
          .filter((c) =>
            this.#watching.has(JSON.stringify([c.origin.scope, c.origin.home])),
          )
          .map((c) => c.id),
      );
      if (this.open) watched.add(this.selected);
      for (const session of this.manager.value?.sessions.filter(
        (s) => s.conversation_id && watched.has(s.conversation_id),
      ) ?? []) {
        let after = Number(this.events[session.id]?.at(-1)?.sequence ?? 0);
        void this.manager.discussions.ensure(session.id);
        if (this.events[session.id] && after >= Number(session.last_sequence))
          continue;
        do {
          const page = await this.manager.client.request<
            Model<"AgentExecutionEventPage">
          >(`agents/events?session=${session.id}&after=${after}`);
          if (!this.#alive) return true;
          this.events[session.id] = [
            ...(this.events[session.id] ?? []),
            ...page.events,
          ];
          after = Number(page.events.at(-1)?.sequence ?? after);
          if (page.next === null || page.next === undefined) break;
        } while (this.#alive);
      }
      if (this.pending) await refreshChatReceipt(this);
      this.streamError = "";
      return true;
    } catch {
      this.streamError = "Chat updates are unavailable. Reconnecting…";
      this.invalidate();
      return false;
    } finally {
      this.#refreshing = false;
    }
  }
  destroy() {
    this.#alive = false;
    this.#updates.stop();
    this.#watching.clear();
    this.close();
    this.drafts = {};
    this.events = {};
  }
}
