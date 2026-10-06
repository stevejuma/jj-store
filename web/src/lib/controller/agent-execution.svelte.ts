import { activeAgentSession } from "./agent-activity-session.js";
import { showAgentLaunch } from "./agent-execution-launch-context.js";
import { AgentDiscussions } from "./agent-discussions.svelte.js";
import { consumeAgentEvents } from "./agent-execution-stream.js";
import { LocalClient } from "../api/local-client.js";
import { agentLaunchDraft } from "./agent-execution-types.js";
import { ApiError, decode, type Model } from "../api/client.js";
import type {
  AgentAction,
  AgentEvent,
  AgentLaunchContext,
  AgentOutcome,
  AgentSetup,
  AgentView,
  AgentStatus,
  AgentTarget,
} from "./agent-execution-types.js";
const newId = () => crypto.randomUUID().replaceAll("-", "");
/** Installation owner retains drafts and exact pending identities across navigation. */
export class AgentExecution {
  open = $state(false);
  onNavigate?: (view: AgentView, id?: string, relatedId?: string) => void;
  setupId = $state("");
  launchDraft = $state({ targetId: "", runner: "codex", commit: "" });
  launchOpen = $state(false);
  context = $state<AgentLaunchContext>({});
  value = $state<AgentStatus | null>(null);
  targets = $state<AgentTarget[]>([]);
  selected = $state("");
  events = $state<AgentEvent[]>([]);
  eventsLoading = $state(false);
  drafts = $state<Record<string, string>>({});
  setupDraftRevisions = $state<Record<string, number | bigint>>({});
  setupDrafts = $state<Record<string, AgentSetup>>({});
  busy = $state(false);
  error = $state("");
  errorSessionId = $state("");
  setupConflict = $state(false);
  detailsId = $state("");
  editRequested = $state<AgentSetup | undefined>();
  connectionError = $state("");
  activityError = $state("");
  pending = $state<{
    path: string;
    body: Record<string, unknown>;
    draft?: string;
  } | null>(null);
  outcome = $state<AgentOutcome | null>(null);
  #alive = true;
  #refreshing = false;
  #stream?: AbortController;
  #timer?: ReturnType<typeof setTimeout>;
  readonly client: LocalClient;
  readonly discussions: AgentDiscussions;
  constructor(
    readonly available: () => boolean,
    fetcher?: typeof fetch,
    principal?: (repository: string) => string,
  ) {
    this.client = new LocalClient("", fetcher);
    this.discussions = new AgentDiscussions(this, fetcher, principal);
  }
  get session() {
    return this.value?.sessions.find((s) => s.id === this.selected);
  }
  show(context: AgentLaunchContext = {}) {
    return showAgentLaunch(this, context);
  }
  directory = () => this.onNavigate?.("directory");
  startAgent(id: string) {
    this.context = {};
    this.launchDraft = { targetId: "", runner: "codex", commit: "" };
    this.setupId = id;
    this.onNavigate?.("new", id);
  }
  openSession(id: string, agent?: string) {
    this.onNavigate?.("session", id, agent);
  }
  viewSessions = (agent?: string) => this.onNavigate?.("sessions", agent);
  viewActivity(agent?: string, session?: string) {
    if (session) return this.openSession(session, agent);
    const active = activeAgentSession(this.value?.sessions ?? [], agent);
    if (active) this.openSession(active.id, agent);
    else this.viewSessions(agent);
  }
  pauseView() {
    this.open = false;
    clearTimeout(this.#timer);
    this.#stream?.abort();
  }
  async refresh() {
    if (!this.#alive || !this.available() || this.#refreshing) return;
    this.#refreshing = true;
    try {
      const [status, targets] = await Promise.all([
        this.client.request<AgentStatus>("agents"),
        this.client.request<AgentTarget[]>(
          `agents/targets${this.context.repository ? `?repository=${encodeURIComponent(this.context.repository)}` : ""}`,
        ),
      ]);
      if (!this.#alive) return;
      this.value = status;
      this.targets = targets;
      if (this.selected && this.open) void this.discussions.load(this.selected);
      if (this.pending) await this.checkOutcome();
      this.connectionError = "";
    } catch (cause) {
      if (this.#alive)
        this.connectionError =
          cause instanceof Error
            ? cause.message
            : "Agent execution unavailable";
    } finally {
      this.#refreshing = false;
      clearTimeout(this.#timer);
      if (this.#alive && this.open)
        this.#timer = setTimeout(() => void this.refresh(), 2000);
    }
  }
  async select(id: string) {
    this.#stream?.abort();
    this.drafts[id] ??= "";
    this.selected = id;
    void this.discussions.load(id);
    this.events = [];
    this.eventsLoading = true;
    const abort = (this.#stream = new AbortController());
    try {
      let after = 0;
      do {
        const page = await this.client.request<
          Model<"AgentExecutionEventPage">
        >(
          `agents/events?session=${id}&after=${after}`,
          undefined,
          abort.signal,
        );
        if (abort.signal.aborted || !this.#alive) return;
        this.events = [...this.events, ...page.events];
        after = Number(page.events.at(-1)?.sequence ?? after);
        if (page.next === null || page.next === undefined) break;
      } while (!abort.signal.aborted);
      void consumeAgentEvents(this.client, id, after, abort.signal, {
        alive: () => this.#alive,
        receive: (events) => {
          this.activityError = "";
          this.events = [...this.events, ...events];
        },
        failed: (message) => {
          this.activityError = message;
        },
      });
    } catch (cause) {
      if (!abort.signal.aborted) this.activityError = String(cause);
    } finally {
      if (this.#stream === abort) this.eventsLoading = false;
    }
  }
  private accept(outcome: AgentOutcome) {
    this.outcome = outcome;
    if (["dispatching", "outcome_unknown", "not_known"].includes(outcome.state))
      return;
    if (
      this.pending?.draft &&
      !["failed", "cancelled", "timed_out"].includes(outcome.state)
    )
      this.drafts[this.pending.draft] = "";
    if (this.pending && !outcome.error) this.error = "";
    this.pending = null;
    if (outcome.error) this.error = outcome.error;
  }
  async checkOutcome() {
    if (!this.pending) return;
    this.accept(
      await this.client.request<AgentOutcome>(
        `agents/outcome?request=${String(this.pending.body.request_id)}`,
      ),
    );
  }
  async retryRetainedRequest() {
    const retained = this.pending;
    if (!retained || this.busy) return;
    this.pending = null;
    await this.send(retained.path, retained.body, retained.draft);
  }
  retainAsUnresolved() {
    if (this.outcome?.state === "outcome_unknown") {
      this.pending = null;
      this.error =
        "The interrupted request remains in session history. Review the workspace before explicitly continuing.";
    }
  }
  private async send(
    path: string,
    body: Record<string, unknown>,
    draft?: string,
  ) {
    if (this.busy || this.pending || !this.available()) return;
    this.busy = true;
    this.error = "";
    this.errorSessionId =
      path === "agents/command" ? String(body.session_id) : "";
    this.setupConflict = false;
    this.pending = { path, body, draft };
    try {
      if (path === "agents/setups") {
        this.value = await this.client.request<AgentStatus>(path, body);
        this.pending = null;
        return this.value.revision;
      } else {
        const outcome = await this.client.request<AgentOutcome>(path, body);
        this.accept(outcome);
        if (path === "agents/launch" && outcome.session_id) {
          this.launchOpen = false;
          const key = `new:${typeof body.setup_id === "string" ? body.setup_id : "unassigned"}`;
          if (this.drafts[key]) {
            this.drafts[outcome.session_id] = this.drafts[key];
            delete this.drafts[key];
          }
          this.onNavigate?.("session", outcome.session_id);
          await this.select(outcome.session_id);
        }
      }
    } catch (cause) {
      if (
        cause instanceof ApiError &&
        [
          "invalid_agent_execution",
          "agent_selection_changed",
          "agent_execution_busy",
          "agent_execution_denied",
          "agent_request_identity_conflict",
          "agent_execution_limit",
        ].includes(cause.code)
      ) {
        this.pending = null;
        this.setupConflict =
          path === "agents/setups" && cause.code === "agent_selection_changed";
        this.error = cause.message;
      } else
        this.error = `${cause instanceof Error ? cause.message : "Request failed"}. The request identity is retained; check its outcome.`;
    } finally {
      this.busy = false;
      await this.refresh();
    }
  }
  async launch(
    target: AgentTarget,
    runner: string,
    setupId?: string,
    commit?: string,
  ) {
    await this.send("agents/launch", {
      version: 1,
      request_id: newId(),
      target_id: target.id,
      repository: target.repository,
      operation: target.operation,
      commit: commit || target.commit,
      runner,
      setup_id: setupId || null,
      context: this.context.text || null,
    });
  }
  async saveSetup(
    setup?: AgentSetup,
    remove?: string,
    revision?: number | bigint,
  ) {
    return this.send("agents/setups", {
      version: 1,
      request_id: newId(),
      expected_revision: revision ?? this.value?.revision ?? 0,
      setup: setup ?? null,
      remove: remove ?? null,
    });
  }
  async command(action: AgentAction, draft?: string) {
    await this.commandSession(this.selected, action, draft);
  }
  async commandSession(id: string, action: AgentAction, draft?: string) {
    if (!id || this.busy || this.pending || !this.available()) return;
    const request_id = newId();
    await this.send(
      "agents/command",
      { version: 1, request_id, session_id: id, action },
      draft,
    );
    return request_id;
  }
  destroy() {
    this.discussions.destroy();
    this.#alive = false;
    clearTimeout(this.#timer);
    this.#stream?.abort();
    this.client.close();
    this.drafts = {};
    this.setupDrafts = {};
    this.pending = null;
  }
}
