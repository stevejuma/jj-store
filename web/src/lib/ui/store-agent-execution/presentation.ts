import {
  object,
  text,
  type AgentEvent,
} from "../../controller/agent-execution-types.js";
import { agentMessageTimestamp } from "../../collaboration/presentation.js";
import {
  communityMessageDay,
  communityMessageDayLabel,
} from "../community-conversation/conversation-day.js";
export type ActivityRow = {
  id: string;
  kind:
    | "user"
    | "assistant"
    | "tool"
    | "plan"
    | "permission"
    | "record"
    | "session";
  text: string;
  value: Record<string, unknown>;
  request: string;
  sequence: number;
  observedAtMillis?: number;
};
function openingReplay(value: Record<string, unknown>) {
  const session = text(value.controllerSessionId);
  return (
    value.type === "activity" &&
    !!session &&
    (value.requestId === `open-${session}` ||
      text(value.requestId).startsWith(`open-${session}-`))
  );
}
/** Old UI versions exposed a reply composer on this synthetic replay row. */
export function openingReplayRowIds(events: readonly AgentEvent[]) {
  return [
    ...new Set(
      events.flatMap((event) => {
        const value = object(event.payload);
        return openingReplay(value) &&
          object(value.update).sessionUpdate === "agent_message_chunk"
          ? [`${text(value.requestId)}/assistant`]
          : [];
      }),
    ),
  ];
}
export function activityRows(events: readonly AgentEvent[]): ActivityRow[] {
  const rows = new Map<string, ActivityRow>();
  for (const event of events) {
    const value = object(event.payload),
      request = text(value.requestId),
      update = object(value.update),
      type = text(value.type);
    // session/load replays history under the opening request. Those chunks
    // have no new canonical message identity and must not become chat rows.
    if (openingReplay(value)) continue;
    const row = (
      id: string,
      kind: ActivityRow["kind"],
      content: string,
      data = value,
    ) =>
      rows.set(id, {
        id,
        kind,
        text: content,
        value: data,
        request,
        sequence: Number(event.sequence),
        observedAtMillis:
          rows.get(id)?.observedAtMillis ??
          (agentMessageTimestamp(event.observed_at_millis).createdAt &&
          Number(event.observed_at_millis) >= 0
            ? Number(event.observed_at_millis)
            : undefined),
      });
    if (type === "opened" && !rows.has("session/started"))
      row("session/started", "session", "Session started");
    if (type === "prompt" || type === "context_delivery")
      row(
        `${request}/user`,
        "user",
        text(value.userText) || text(value.authoredText) || text(value.text),
      );
    if (type === "activity" && update.sessionUpdate === "agent_message_chunk") {
      const id = `${request}/assistant`,
        previous = rows.get(id);
      row(
        id,
        "assistant",
        (previous?.text ?? "") + text(object(update.content).text),
      );
    }
    if (type === "activity" && value.toolSnapshot) {
      const tool = object(value.toolSnapshot);
      row(
        `${request}/tool/${text(tool.toolCallId)}`,
        "tool",
        text(tool.title),
        tool,
      );
    }
    if (type === "activity" && update.sessionUpdate === "plan")
      row(`${request}/plan`, "plan", "Plan", update);
    if (type === "permission")
      row(
        `permission/${text(value.permissionId)}`,
        "permission",
        text(object(object(value.request).toolCall).title),
      );
    if (type === "permission_resolved")
      rows.delete(`permission/${text(value.permissionId)}`);
    if (type === "record")
      row(`${request}/record`, "record", text(object(value.record).status));
  }
  return [...rows.values()];
}
/** Use the thread chat's day presentation, with honest groups for legacy data. */
export function activityDayDividers(rows: readonly ActivityRow[]) {
  const dividers = new Map<string, string>();
  let previous: string | undefined;
  for (const row of rows) {
    const message = {
      id: row.id,
      authorId: "",
      content: row.text,
      ...agentMessageTimestamp(row.observedAtMillis),
    };
    const day = communityMessageDay(message);
    if (day !== previous)
      dividers.set(
        row.id,
        day === "unknown"
          ? "Time not recorded"
          : communityMessageDayLabel(message),
      );
    previous = day;
  }
  return dividers;
}
export function choices(value: unknown, key: string, id = "id") {
  const array = object(value)[key];
  return Array.isArray(array)
    ? array
        .map((v) => object(v))
        .map((v) => ({ id: text(v[id]), name: text(v.name) || text(v[id]) }))
        .filter((v) => v.id)
    : [];
}
/** Prefer the negotiated ACP configuration category, including grouped options. */
export function runtimeChoices(session: unknown, category: string) {
  const facts = object(session);
  const configs = Array.isArray(facts.configOptions) ? facts.configOptions : [];
  const matching = configs
    .map(object)
    .filter((option) => option.category === category);
  const flatten = (entries: unknown[]): { id: string; name: string }[] =>
    entries.flatMap((entry) => {
      const value = object(entry);
      return Array.isArray(value.options)
        ? flatten(value.options)
        : text(value.value)
          ? [
              {
                id: text(value.value),
                name: text(value.name) || text(value.value),
              },
            ]
          : [];
    });
  if (matching.length)
    return matching.flatMap((option) =>
      Array.isArray(option.options) ? flatten(option.options) : [],
    );
  if (category === "mode") return choices(facts.modes, "availableModes");
  if (category === "model")
    return choices(facts.models, "availableModels", "modelId");
  return [];
}
export function queuedRequests(events: readonly AgentEvent[]) {
  const states = new Map<string, string>();
  for (const event of events) {
    const payload = object(event.payload),
      type = text(payload.type);
    if (["queued", "started", "record", "error"].includes(type))
      states.set(text(payload.requestId), type);
  }
  return new Set(
    [...states]
      .filter(([, type]) => type === "queued")
      .map(([request]) => request),
  );
}
