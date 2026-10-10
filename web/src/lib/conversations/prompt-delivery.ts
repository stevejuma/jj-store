import { agentMentionMatches } from "../community/agent-mentions.js";
import {
  object,
  text,
  type AgentEvent,
} from "../controller/agent-execution-types.js";

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
export function promptDelivery(events: readonly AgentEvent[], request: string) {
  return promptDeliveries(events).get(request);
}
export function promptDeliveries(events: readonly AgentEvent[]) {
  return new Map(
    [...promptDeliveryObservations(events)].map(([request, value]) => [
      request,
      value.state,
    ]),
  );
}
export function promptDeliveryObservations(events: readonly AgentEvent[]) {
  const states = new Map<string, { state: string; sequence: number }>();
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
      states.set(request, { state, sequence: Number(event.sequence) });
  }
  return states;
}
