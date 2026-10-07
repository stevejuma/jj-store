/** Device-local opt-in audience; stores exact IDs, never credentials or prompts. */
const key = "jj-store:agent-audiences:v1";
type Audiences = Record<string, { enabled: boolean; ids: string[] }>;
export function readAgentAudience(scope: string, storage?: Storage) {
  try {
    const all = JSON.parse(storage?.getItem(key) ?? "{}") as Audiences;
    const value = all[scope];
    return {
      enabled: value?.enabled === true,
      ids: Array.isArray(value?.ids)
        ? value.ids
            .filter((id) => typeof id === "string" && id.length <= 64)
            .slice(0, 4)
        : [],
    };
  } catch {
    return { enabled: false, ids: [] as string[] };
  }
}
export function writeAgentAudience(
  scope: string,
  enabled: boolean,
  ids: readonly string[],
  storage?: Storage,
) {
  try {
    const all = JSON.parse(storage?.getItem(key) ?? "{}") as Audiences;
    delete all[scope];
    const recent = Object.fromEntries(Object.entries(all).slice(-127));
    recent[scope] = {
      enabled,
      ids: enabled ? [...new Set(ids)].slice(0, 4) : [],
    };
    storage?.setItem(key, JSON.stringify(recent));
  } catch {
    /* Storage denial leaves the current composer usable. */
  }
}
