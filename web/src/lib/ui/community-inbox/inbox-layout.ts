export const agentInboxLayoutKey = "jj-store/layout/v1/agent-inbox/sessions";

export function loadInboxWidth(storage?: Storage): number | undefined {
  try {
    const value = JSON.parse(storage?.getItem(agentInboxLayoutKey) ?? "null");
    return value?.version === 1 &&
      value.panel === "agent-sessions" &&
      typeof value.width === "number" &&
      Number.isFinite(value.width)
      ? Math.min(480, Math.max(224, value.width))
      : undefined;
  } catch {
    return undefined;
  }
}
export function saveInboxWidth(
  storage: Storage | undefined,
  width: number | undefined,
) {
  try {
    storage?.setItem(
      agentInboxLayoutKey,
      JSON.stringify({
        version: 1,
        panel: "agent-sessions",
        width: width ?? null,
      }),
    );
  } catch {
    /* Layout remains usable when device storage is unavailable. */
  }
}
