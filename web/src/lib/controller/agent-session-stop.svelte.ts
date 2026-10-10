import type { LocalClient } from "../api/local-client.js";
import type { AgentAction, AgentOutcome } from "./agent-execution-types.js";
import {
  isAgentCommandRefusal,
  agentCommandError,
} from "./agent-execution-errors.js";

type StopRequest = {
  body: Record<string, unknown>;
  busy: boolean;
  error: string;
  outcome?: AgentOutcome;
};

/** Cancellation has its own receipt lane, so a pending Send cannot block Stop. */
export class AgentSessionStop {
  requests = $state<Record<string, StopRequest>>({});
  errors = $state<Record<string, string>>({});
  constructor(
    private client: LocalClient,
    private refresh: () => Promise<unknown>,
  ) {}
  private accept(id: string, outcome: AgentOutcome) {
    const request = this.requests[id];
    if (!request || outcome.request_id !== request.body.request_id) return;
    request.outcome = outcome;
    if (
      ![
        "dispatching",
        "queued",
        "started",
        "outcome_unknown",
        "not_known",
      ].includes(outcome.state)
    ) {
      if (outcome.error) this.errors[id] = outcome.error;
      else delete this.errors[id];
      delete this.requests[id];
    } else if (["outcome_unknown", "not_known"].includes(outcome.state))
      request.error =
        "Stop outcome unknown. Check the retained request before stopping again.";
  }
  async check(id: string) {
    const request = this.requests[id];
    if (!request || request.busy) return;
    request.busy = true;
    try {
      this.accept(
        id,
        await this.client.request<AgentOutcome>(
          `agents/outcome?request=${String(request.body.request_id)}`,
        ),
      );
    } catch {
      request.error =
        "Could not confirm Stop. Its request identity is retained.";
    } finally {
      request.busy = false;
    }
  }
  checkOutcomes() {
    return Promise.all(Object.keys(this.requests).map((id) => this.check(id)));
  }
  async stop(id: string, action: AgentAction) {
    if (this.requests[id]) return this.check(id);
    let request: StopRequest = {
      body: {
        version: 1,
        request_id: crypto.randomUUID().replaceAll("-", ""),
        session_id: id,
        action,
      },
      busy: true,
      error: "",
    };
    this.requests[id] = request;
    request = this.requests[id];
    delete this.errors[id];
    try {
      this.accept(
        id,
        await this.client.request<AgentOutcome>("agents/command", request.body),
      );
    } catch (cause) {
      request.error = isAgentCommandRefusal(cause)
        ? agentCommandError(cause)
        : "Stop outcome unknown. Its request identity is retained.";
      if (isAgentCommandRefusal(cause)) {
        this.errors[id] = request.error;
        delete this.requests[id];
      }
    } finally {
      request.busy = false;
      await this.refresh();
    }
  }
}
