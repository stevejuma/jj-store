import {
  object,
  text,
  type AgentEvent,
} from "../../controller/agent-execution-types.js";
export type ActivityRow = {
  id: string;
  kind: "user" | "assistant" | "tool" | "plan" | "permission" | "record";
  text: string;
  value: Record<string, unknown>;
  request: string;
  sequence: number;
};
export function activityRows(events: readonly AgentEvent[]): ActivityRow[] {
  const rows = new Map<string, ActivityRow>();
  for (const event of events) {
    const value = object(event.payload),
      request = text(value.requestId),
      update = object(value.update),
      type = text(value.type);
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
      });
    if (type === "prompt") row(`${request}/user`, "user", text(value.text));
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
