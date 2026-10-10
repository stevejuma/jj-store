import type { AgentExecution } from "../controller/agent-execution.svelte.js";
import type { AgentSession } from "../controller/agent-execution-types.js";
import { object } from "../controller/agent-execution-types.js";
import type { Model } from "../api/client.js";
import type { ChatPromptSubmission } from "../ui/store-agent-chat-body/model.js";
import { activityRows } from "../ui/store-agent-execution/presentation.js";
import {
  authoredPrompt,
  promptDeliveryObservations,
} from "./prompt-delivery.js";
import { conversationSubmissionResult } from "./submission-result.js";

type Recipient = { request: string; session: string; state: string };
type Captured = { submission: ChatPromptSubmission; recipients: Recipient[] };

/** Captured submissions belong to the application, independently of its renderers. */
export class QueuedAgentMessages {
  captured = $state<Record<string, Captured>>({});
  disclosures = $state<Record<string, boolean>>({});
  cancelling = $state<Record<string, boolean>>({});
  errors = $state<Record<string, string>>({});
  confirmed = $state<Record<string, { state: string; sequence: number }>>({});
  private recipients = $state<Record<string, Recipient>>({});
  private deliveries = $derived.by(() => {
    const states = new Map<string, { state: string; sequence: number }>();
    const sessions = new Set([
      ...Object.keys(this.manager.chat.events),
      this.manager.selected,
    ]);
    for (const session of sessions)
      for (const [request, state] of promptDeliveryObservations(
        this.events(session),
      ))
        states.set(`${session}:${request}`, state);
    return states;
  });
  constructor(private manager: AgentExecution) {}
  submitted(
    submission: ChatPromptSubmission,
    receipt: Model<"AgentConversationOutcome">,
  ) {
    this.captured[submission.id] = {
      submission: structuredClone($state.snapshot(submission)),
      recipients: receipt.agents.flatMap((a) =>
        a.session_id
          ? [
              {
                request: a.request_id,
                session: a.session_id,
                state: a.state,
              },
            ]
          : [],
      ),
    };
    for (const recipient of this.captured[submission.id].recipients)
      this.recipients[`${recipient.session}:${recipient.request}`] = recipient;
    const result = conversationSubmissionResult(receipt);
    // Deferred workspace reviews can be accepted before their publication begins.
    if (
      submission.comments.length &&
      !receipt.comment_publication &&
      !result.recoveryRequired
    )
      return { ...result, publication: "queued" as const };
    return result;
  }
  events(session: string) {
    const events = [
      ...(this.manager.chat.events[session] ?? []),
      ...(this.manager.selected === session ? this.manager.events : []),
    ];
    return [
      ...new Map(events.map((e) => [Number(e.sequence), e])).values(),
    ].sort((a, b) => Number(a.sequence) - Number(b.sequence));
  }
  state(recipient: Recipient) {
    const key = `${recipient.session}:${recipient.request}`;
    const confirmed = this.confirmed[key];
    const observed = this.deliveries.get(key);
    if (
      confirmed &&
      (!observed ||
        observed.sequence <= confirmed.sequence ||
        ["cancelled", "completed", "failed"].includes(confirmed.state))
    )
      return confirmed.state;
    if (["cancelled", "completed", "failed"].includes(recipient.state))
      return recipient.state;
    if (observed) return observed.state;
    const outcome = this.manager.outcome;
    if (
      outcome?.request_id === recipient.request &&
      outcome.session_id === recipient.session
    )
      return outcome.state;
    for (const receipt of Object.values(this.manager.chat.receipts)) {
      const exact = receipt.agents.find(
        (a) =>
          a.request_id === recipient.request &&
          a.session_id === recipient.session,
      );
      if (exact) return exact.state;
    }
    return recipient.state;
  }
  isQueued(session: string, request: string) {
    const captured = this.recipients[`${session}:${request}`];
    return captured
      ? this.state(captured) === "queued"
      : this.state({ session, request, state: "" }) === "queued";
  }
  hidePrompt(session: string, request: string) {
    if (this.isPending(session, request)) return true;
    const recipient = this.recipients[`${session}:${request}`];
    if (
      this.state(recipient ?? { session, request, state: "" }) !== "cancelled"
    )
      return false;
    const exact = this.events(session).filter(
      (e) => object(e.payload).requestId === request,
    );
    return (
      !exact.some((e) => object(e.payload).type === "started") &&
      (recipient?.state === "queued" ||
        exact.some(
          (e) => object(object(e.payload).sentContext).delivery === "queued",
        ))
    );
  }
  isPending(session: string, request: string) {
    const recipient = this.recipients[`${session}:${request}`];
    if (!recipient) return this.isQueued(session, request);
    const state = this.state(recipient);
    if (state === "queued") return true;
    if (
      !["error", "failed", "unknown", "outcome_unknown", "not_known"].includes(
        state,
      )
    )
      return false;
    const events = this.events(session);
    const exact = events.filter((e) => object(e.payload).requestId === request);
    const started = exact.some((e) => object(e.payload).type === "started");
    const queued =
      recipient.state === "queued" ||
      exact.some(
        (e) =>
          object(e.payload).type === "queued" ||
          object(object(e.payload).sentContext).delivery === "queued",
      );
    return (
      state === "queued" ||
      (!started &&
        queued &&
        ["error", "failed", "unknown", "outcome_unknown", "not_known"].includes(
          state,
        ))
    );
  }
  entries(sessions: readonly AgentSession[]) {
    const ids = new Set(sessions.map((s) => s.id));
    const entries = Object.values(this.captured).flatMap((entry) => {
      const recipients = entry.recipients.filter(
        (r) => ids.has(r.session) && this.isPending(r.session, r.request),
      );
      return recipients.length ? [{ ...entry, recipients }] : [];
    });
    const known = new Set(
      entries.flatMap((e) =>
        e.recipients.map((r) => `${r.session}:${r.request}`),
      ),
    );
    for (const session of sessions) {
      for (const row of activityRows(this.events(session.id))) {
        if (
          row.kind !== "user" ||
          known.has(`${session.id}:${row.request}`) ||
          !this.isQueued(session.id, row.request)
        )
          continue;
        entries.push({
          submission: {
            id: `${session.id}:${row.request}`,
            text: row.text,
            comments: [],
          },
          recipients: [
            { session: session.id, request: row.request, state: "queued" },
          ],
        });
      }
    }
    return entries.map((e) => ({
      ...e,
      text: authoredPrompt(e.submission.text),
      recipients: e.recipients.map((r) => ({
        ...r,
        state: this.state(r),
        label: sessions.find((s) => s.id === r.session)?.title ?? r.session,
      })),
    }));
  }
  async check(session: string, request: string) {
    const outcome = await this.manager.client.request<
      Model<"AgentExecutionOutcome">
    >(`agents/outcome?request=${request}`);
    if (outcome.request_id !== request || outcome.session_id !== session)
      throw new Error("The queued request outcome is unavailable.");
    this.confirmed[`${session}:${request}`] = {
      state: outcome.state,
      sequence: this.deliveries.get(`${session}:${request}`)?.sequence ?? 0,
    };
    return outcome;
  }
  async review(session: string, request: string) {
    const key = `${session}:${request}`;
    try {
      await this.check(session, request);
      this.errors[key] = "";
    } catch (cause) {
      this.errors[key] =
        cause instanceof Error ? cause.message : "Request status unavailable";
    }
  }
  async cancel(session: string, request: string) {
    const key = `${session}:${request}`;
    if (this.cancelling[key] || !this.isQueued(session, request)) return;
    this.cancelling[key] = true;
    this.errors[key] = "";
    try {
      const current = await this.check(session, request);
      if (current.state !== "queued")
        throw new Error(
          "The message is no longer queued. Its current outcome is retained.",
        );
      const id = await this.manager.commandSession(session, {
        action: "cancel",
        target: request,
      });
      if (!id || this.manager.pending || this.manager.error)
        throw new Error(
          this.manager.error ||
            "Cancellation is awaiting confirmation. Check the retained request.",
        );
      const outcome = await this.check(session, request);
      for (const entry of Object.values(this.captured)) {
        const exact = entry.recipients.find(
          (r) => r.session === session && r.request === request,
        );
        if (exact) exact.state = outcome.state;
      }
      for (const receipt of Object.values(this.manager.chat.receipts)) {
        const exact = receipt.agents.find(
          (a) => a.request_id === request && a.session_id === session,
        );
        if (exact) Object.assign(exact, outcome);
      }
      await this.manager.chat.refresh();
      if (this.manager.selected === session) this.manager.invalidateHistory();
      if (outcome.state !== "cancelled")
        throw new Error(
          "The message is no longer queued. Its current outcome is retained.",
        );
    } catch (cause) {
      this.errors[key] =
        cause instanceof Error ? cause.message : "Cancellation unavailable";
    } finally {
      this.cancelling[key] = false;
    }
  }
}
