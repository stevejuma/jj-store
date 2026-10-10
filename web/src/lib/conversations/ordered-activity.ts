import {
  object,
  text,
  type AgentEvent,
} from "../controller/agent-execution-types.js";
import {
  openingReplay,
  type ActivityRow,
} from "../ui/store-agent-execution/presentation.js";
import { agentMessageTimestamp } from "../collaboration/presentation.js";

function observedTime(event: AgentEvent): number | undefined {
  const millis = event.observed_at_millis;
  return agentMessageTimestamp(millis).createdAt && Number(millis) >= 0
    ? Number(millis)
    : undefined;
}
export interface OrderedActivityRow extends ActivityRow {
  sourceRowId?: string;
  sourceFrom?: number;
  sourceTo?: number;
  /** No guessed tool/prose ordering for captures that only retained a final answer. */
  orderingKnown?: boolean;
}
/** Replace only assistant rows, preserving the existing nontext event projection. */
export function orderedAssistantRows(
  events: readonly AgentEvent[],
  base: readonly ActivityRow[],
): OrderedActivityRow[] {
  const result: OrderedActivityRow[] = base
    .filter((row) => row.kind !== "assistant")
    .map((row) => ({ ...row }));
  const totals = new Map<string, string>();
  const active = new Map<string, OrderedActivityRow>();
  const counts = new Map<string, number>();
  for (const event of events) {
    const value = object(event.payload),
      update = object(value.update),
      request = text(value.requestId);
    if (openingReplay(value)) continue;
    if (
      value.type === "activity" &&
      update.sessionUpdate === "agent_message_chunk"
    ) {
      const chunk = text(object(update.content).text);
      if (!chunk) continue;
      const prior = totals.get(request) ?? "";
      let row = active.get(request);
      if (!row) {
        const count = counts.get(request) ?? 0;
        row = {
          id: count
            ? `${request}/assistant/segment/${event.sequence}`
            : `${request}/assistant`,
          sourceRowId: `${request}/assistant`,
          kind: "assistant",
          text: "",
          value,
          request,
          sequence: Number(event.sequence),
          observedAtMillis: observedTime(event),
          sourceFrom: prior.length,
          sourceTo: prior.length,
          orderingKnown: true,
        };
        counts.set(request, count + 1);
        active.set(request, row);
        result.push(row);
      }
      row.observedAtMillis ??= observedTime(event);
      row.text += chunk;
      row.sourceTo = prior.length + chunk.length;
      totals.set(request, prior + chunk);
    } else if (
      (value.type === "activity" &&
        (value.toolSnapshot || update.sessionUpdate === "plan")) ||
      value.type === "permission"
    ) {
      active.delete(request);
    } else if (value.type === "record") {
      const captured = text(value.assistantText);
      if (captured && !totals.has(request)) {
        result.push({
          id: `${request}/assistant`,
          sourceRowId: `${request}/assistant`,
          kind: "assistant",
          text: captured,
          value,
          request,
          sequence: Number(event.sequence),
          observedAtMillis: observedTime(event),
          sourceFrom: 0,
          sourceTo: captured.length,
          orderingKnown: false,
        });
      } else if (captured && captured !== totals.get(request)) {
        // Retain the displayed stream but withhold source ranges on disagreement.
        for (const row of result.filter(
          (r) => r.request === request && r.kind === "assistant",
        )) {
          row.sourceFrom = undefined;
          row.sourceTo = undefined;
        }
      }
      active.delete(request);
    }
  }
  // Nontext rows retain their first observed position even as tool snapshots update.
  const first = new Map<string, number>();
  for (const event of events) {
    const value = object(event.payload),
      request = text(value.requestId);
    const id =
      value.type === "activity" && value.toolSnapshot
        ? `${request}/tool/${text(object(value.toolSnapshot).toolCallId)}`
        : value.type === "prompt"
          ? `${request}/user`
          : undefined;
    if (id && !first.has(id)) first.set(id, Number(event.sequence));
  }
  for (const row of result) row.sequence = first.get(row.id) ?? row.sequence;
  return result.sort((a, b) => a.sequence - b.sequence);
}
