import { LocalClient } from "../api/local-client.js";
import { ApiError, decode, type Model } from "../api/client.js";
import type {
  AgentAction,
  AgentEvent,
  AgentLaunchContext,
  AgentOutcome,
  AgentSetup,
  AgentStatus,
  AgentTarget,
} from "./agent-execution-types.js";
const newId = () => crypto.randomUUID().replaceAll("-", "");
/** Installation owner retains drafts and exact pending identities across navigation. */
export class AgentExecution {
  open = $state(false);
  launchOpen = $state(false);
  context = $state<AgentLaunchContext>({});
  value = $state<AgentStatus | null>(null);
  targets = $state<AgentTarget[]>([]);
  selected = $state("");
  events = $state<AgentEvent[]>([]);
  drafts = $state<Record<string, string>>({});
  setupDrafts = $state<Record<string, AgentSetup>>({});
  busy = $state(false);
  error = $state("");
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
  constructor(
    readonly available: () => boolean,
    fetcher?: typeof fetch,
  ) {
    this.client = new LocalClient("", fetcher);
  }
  get session() {
    return this.value?.sessions.find((s) => s.id === this.selected);
  }
  async show(context: AgentLaunchContext = {}) {
    if (!this.available()) return;
    this.context = context;
    this.open = true;
    this.launchOpen = true;
    await this.refresh();
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
    this.selected = id;
    this.events = [];
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
      void this.stream(id, after, abort.signal);
    } catch (cause) {
      if (!abort.signal.aborted) this.error = String(cause);
    }
  }
  private async stream(id: string, after: number, signal: AbortSignal) {
    while (this.#alive && !signal.aborted) {
      try {
        const response = await this.client.fetch(
          `agents/stream?session=${id}&after=${after}`,
          { signal },
        );
        if (
          !response.ok ||
          !response.headers.get("content-type")?.startsWith("text/event-stream")
        )
          throw new Error("Agent activity unavailable");
        const reader = response.body!.getReader(),
          decoder = new TextDecoder();
        let buffer = "";
        try {
          while (!signal.aborted) {
            const next = await reader.read();
            if (next.done) break;
            buffer += decoder.decode(next.value, { stream: true });
            if (buffer.length > 16 * 1024 * 1024)
              throw new Error("Agent activity limit");
            let end: number;
            while ((end = buffer.indexOf("\n\n")) >= 0) {
              const frame = buffer.slice(0, end);
              buffer = buffer.slice(end + 2);
              const data = frame
                .split("\n")
                .filter((line) => line.startsWith("data:"))
                .map((line) => line.slice(5).trimStart())
                .join("\n");
              if (!data) continue;
              const page = decode(data) as Model<"AgentExecutionEventPage">;
              this.activityError = "";
              const added = page.events.filter(
                (e) => Number(e.sequence) > after,
              );
              if (added.length) {
                after = Number(added.at(-1)!.sequence);
                this.events = [...this.events, ...added];
              }
            }
          }
        } finally {
          await reader.cancel().catch(() => {});
          reader.releaseLock();
        }
      } catch (cause) {
        if (!signal.aborted)
          this.activityError =
            cause instanceof Error
              ? cause.message
              : "Agent activity reconnecting";
      }
      if (!signal.aborted)
        await new Promise<void>((resolve) => {
          const timeout = setTimeout(resolve, 1000);
          signal.addEventListener(
            "abort",
            () => {
              clearTimeout(timeout);
              resolve();
            },
            { once: true },
          );
        });
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
    this.pending = { path, body, draft };
    try {
      if (path === "agents/setups") {
        this.value = await this.client.request<AgentStatus>(path, body);
        this.pending = null;
      } else {
        const outcome = await this.client.request<AgentOutcome>(path, body);
        this.accept(outcome);
        if (path === "agents/launch" && outcome.session_id) {
          this.launchOpen = false;
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
  async saveSetup(setup?: AgentSetup, remove?: string) {
    await this.send("agents/setups", {
      version: 1,
      request_id: newId(),
      expected_revision: this.value?.revision ?? 0,
      setup: setup ?? null,
      remove: remove ?? null,
    });
  }
  async command(action: AgentAction, draft?: string) {
    if (!this.selected) return;
    await this.send(
      "agents/command",
      { version: 1, request_id: newId(), session_id: this.selected, action },
      draft,
    );
  }
  destroy() {
    this.#alive = false;
    clearTimeout(this.#timer);
    this.#stream?.abort();
    this.client.close();
    this.drafts = {};
    this.setupDrafts = {};
    this.pending = null;
  }
}
