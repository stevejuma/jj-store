export interface InboxPreferences {
  pinned: string[];
  archived: string[];
}
export const inboxPreferencesKey = (kind: "thread" | "session") =>
  `jj-store/inbox/v1/${kind}`;
const limit = 1024;
export function readInboxPreferences(
  storage: Storage | undefined,
  kind: "thread" | "session",
): InboxPreferences {
  try {
    return parseInboxPreferences(
      storage?.getItem(inboxPreferencesKey(kind)) ?? null,
    );
  } catch {
    return { pinned: [], archived: [] };
  }
}
export function parseInboxPreferences(
  serialized: string | null,
): InboxPreferences {
  try {
    if (!serialized || serialized.length > 1_100_000)
      return { pinned: [], archived: [] };
    const value = JSON.parse(serialized);
    const valid = (entries: unknown): entries is string[] =>
      Array.isArray(entries) &&
      entries.length <= limit &&
      entries.every(
        (id) => typeof id === "string" && id.length > 0 && id.length <= 512,
      );
    if (value?.version !== 1 || !valid(value.pinned) || !valid(value.archived))
      return { pinned: [], archived: [] };
    return {
      pinned: [...new Set<string>(value.pinned)],
      archived: [...new Set<string>(value.archived)],
    };
  } catch {
    return { pinned: [], archived: [] };
  }
}
export function toggleInboxPreference(
  current: InboxPreferences,
  field: keyof InboxPreferences,
  id: string,
): InboxPreferences {
  if (!id || id.length > 512) return current;
  const values = current[field];
  return {
    ...current,
    [field]: values.includes(id)
      ? values.filter((value) => value !== id)
      : [...values.slice(-(limit - 1)), id],
  };
}
export function saveInboxPreferences(
  storage: Storage | undefined,
  kind: "thread" | "session",
  value: InboxPreferences,
) {
  try {
    storage?.setItem(
      inboxPreferencesKey(kind),
      JSON.stringify({ version: 1, ...value }),
    );
  } catch {
    /* The same session remains interactive when storage is unavailable. */
  }
}
