export function inboxSections(kind: "session" | "thread", scopeLabel?: string) {
  return [
    { value: "all", label: "All" },
    ...(scopeLabel ? [{ value: "scope", label: scopeLabel }] : []),
    ...(kind === "thread"
      ? [{ value: "active", label: "Active" }]
      : [{ value: "connected", label: "Connected" }]),
    { value: "archived", label: "Archived" },
    ...(kind === "thread" ? [] : [{ value: "agents", label: "Agents" }]),
  ];
}

export function inboxPanelIds(kind: "session" | "thread") {
  return kind === "thread"
    ? {
        plural: "Threads",
        listLabel: "Thread conversations",
        panelId: "thread-conversations",
        settingsId: "thread-conversation-settings",
      }
    : {
        plural: "Sessions",
        listLabel: "Session chats",
        panelId: "agent-sessions",
        settingsId: "agent-session-settings",
      };
}
