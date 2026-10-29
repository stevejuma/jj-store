import { agentMentionMatches } from "./agent-mentions.js";

/** Sanitized host event only. Sequence values retain the host's full width. */
export interface AgentEvent<S = number | bigint> {
  sequence: S;
  payload: unknown;
}
function object(value: unknown): Record<string, unknown> {
  return value && typeof value === "object" && !Array.isArray(value)
    ? (value as Record<string, unknown>)
    : {};
}
function text(value: unknown): string {
  return typeof value === "string" ? value : "";
}

/** Audience tokens select recipients; they are not authored prompt content. */
export function authoredPrompt(value: string) {
  return agentMentionMatches(value)
    .reduceRight(
      (result, match) => result.slice(0, match.start) + result.slice(match.end),
      value,
    )
    .trim();
}

/** Controller queue receipts precede the ACP started acknowledgement. */
export function promptDelivery<S>(
  events: readonly AgentEvent<S>[],
  request: string,
) {
  return promptDeliveries(events).get(request);
}
export function promptDeliveries<S>(events: readonly AgentEvent<S>[]) {
  return new Map(
    [...promptDeliveryObservations(events)].map(([request, value]) => [
      request,
      value.state,
    ]),
  );
}
export function promptDeliveryObservations<S>(
  events: readonly AgentEvent<S>[],
) {
  const states = new Map<string, { state: string; sequence: S }>();
  for (const event of events) {
    const payload = object(event.payload);
    const request = text(payload.requestId);
    if (!request) continue;
    let state = states.get(request)?.state;
    const kind = text(payload.type);
    if (["queued", "started", "record", "error"].includes(kind)) state = kind;
    if (kind === "prompt" || kind === "context_delivery") {
      const delivery = text(object(payload.sentContext).delivery);
      if (
        delivery === "queued" &&
        !["started", "record", "error", "failed"].includes(state ?? "")
      )
        state = "queued";
      if (
        delivery === "sent" &&
        !["record", "error", "failed"].includes(state ?? "")
      )
        state = "started";
      if (delivery === "failed" || delivery === "unknown") state = delivery;
    }
    if (
      state &&
      [
        "queued",
        "started",
        "record",
        "error",
        "prompt",
        "context_delivery",
      ].includes(kind)
    )
      states.set(request, { state, sequence: event.sequence });
  }
  return states;
}
