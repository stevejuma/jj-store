import { disposeAgentResources } from "./agent-resource-disposal.js";
import {
  agentControlBody,
  agentSetupBody,
  agentLaunchBody,
} from "./agent-execution-status.js";
import {
  isAgentCommandRefusal,
  agentCommandError,
  retainUnknownAgentOutcome,
} from "./agent-execution-errors.js";
import { openAgentActivity } from "./agent-activity-session.js";
import * as launch from "./agent-execution-launch-context.js";
import { AgentDiscussions } from "./agent-discussions.svelte.js";
import { AgentChat } from "./agent-chat.svelte.js";
import { AgentModelCatalog } from "./agent-model-catalog.svelte.js";
import { AgentExecutionRefresh } from "./agent-execution-refresh.js";
import { AgentCompute } from "./agent-compute.svelte.js";
import { AgentSettings } from "./agent-settings.svelte.js";
import { AgentRuntimes } from "./agent-runtimes.svelte.js";
import {
  consumeAgentEvents,
  readAgentEventHistory,
} from "./agent-execution-stream.js";
import { LocalClient } from "../api/local-client.js";
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
  targetError = $state("");
  activityError = $state("");
  pending = $state<{
    path: string;
    body: Record<string, unknown>;
    draft?: string;
  } | null>(null);
  outcome = $state<AgentOutcome | null>(null);
  #alive = true;
  #stream?: AbortController;
  readonly updates: AgentExecutionRefresh;
  readonly client: LocalClient;
  readonly discussions: AgentDiscussions;
  readonly chat: AgentChat;
  readonly catalogs: AgentModelCatalog;
  readonly tooling: AgentRuntimes;
  readonly settings: AgentSettings;
  readonly compute: AgentCompute;
  constructor(
    readonly available: () => boolean,
    fetcher?: typeof fetch,
    principal?: (repository: string) => string,
    freshness?: import("./application-freshness.js").ApplicationFreshness,
  ) {
    this.client = new LocalClient("", fetcher);
    this.chat = new AgentChat(this, freshness);
    this.settings = new AgentSettings(this.client);
    this.compute = new AgentCompute(this.client);
    this.catalogs = new AgentModelCatalog(this.client);
    this.tooling = new AgentRuntimes(available, fetcher);
    this.updates = new AgentExecutionRefresh(this, freshness);
    this.discussions = new AgentDiscussions(
      this.client,
      () => this.value?.sessions ?? [],
      fetcher,
      principal,
      freshness,
    );
  }
  get session() {
    return this.value?.sessions.find((s) => s.id === this.selected);
  }
  show(context: AgentLaunchContext = {}) {
    return launch.showAgentLaunch(this, context);
  }
  directory = () => this.onNavigate?.("directory");
  startAgent = (id: string) =>
    this.send("agents/control", agentControlBody(id, true));
  stopAgent(id: string) {
    return this.send("agents/control", agentControlBody(id, false));
  }
  newChat = (id?: string) => this.chat.showGeneral(id);
  openSession(id: string, agent?: string) {
    this.onNavigate?.("session", id, agent);
  }
  viewSessions = (agent?: string) => this.onNavigate?.("sessions", agent);
  viewActivity(agent?: string, session?: string) {
    openAgentActivity(this, agent, session);
  }
  pauseView() {
    this.open = false;
    this.#stream?.abort();
  }
  refresh() {
    return this.updates.refresh();
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
      const after = await readAgentEventHistory(
        this.client,
        id,
        abort.signal,
        (events) => {
          if (this.#alive) this.events = [...this.events, ...events];
        },
      );
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
    retainUnknownAgentOutcome(this);
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
        this.catalogs.invalidate();
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
      if (isAgentCommandRefusal(cause)) {
        this.pending = null;
        this.setupConflict =
          path === "agents/setups" && cause.code === "agent_selection_changed";
        this.error = agentCommandError(cause);
      } else
        this.error = `${cause instanceof Error ? cause.message : "Request failed"}. The request identity is retained; check its outcome.`;
    } finally {
      this.busy = false;
      await this.updates.status();
    }
  }
  async launch(
    target: AgentTarget,
    runner: string,
    setupId?: string,
    commit?: string,
  ) {
    await this.send(
      "agents/launch",
      agentLaunchBody(target, runner, setupId, commit, this.context.text),
    );
  }
  async saveSetup(
    setup?: AgentSetup,
    remove?: string,
    revision?: number | bigint,
  ) {
    return this.send(
      "agents/setups",
      agentSetupBody(setup, remove, revision ?? this.value?.revision ?? 0),
    );
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
    this.#alive = false;
    this.updates.destroy();
    disposeAgentResources(this);
    this.#stream?.abort();
  }
}
