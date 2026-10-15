import type { StoreRoute } from "../../controller/store-route-types.js";

/** Presentation drafts supplement controller-owned pending-command guards. */
export const repositorySettingsGuardKey = Symbol("repository-settings-drafts");
export interface RepositorySettingsDraftGuard {
  folders: string;
}
/** Only category/subtab changes may reuse a pending action's exact route owner. */
export function sameRepositorySettingsOwner(
  previous: StoreRoute,
  next: StoreRoute,
): boolean {
  if (
    !previous.repository ||
    previous.page !== "settings" ||
    next.page !== "settings" ||
    previous.globalSettings ||
    next.globalSettings ||
    previous.projectsHub ||
    next.projectsHub ||
    previous.agentsHub ||
    next.agentsHub ||
    previous.threadsHub ||
    next.threadsHub
  )
    return false;
  const keys = new Set([
    ...Object.keys(previous),
    ...Object.keys(next),
  ] as (keyof StoreRoute)[]);
  return [...keys].every(
    (key) =>
      key === "repositorySettingsSection" ||
      key === "repositorySettingsTab" ||
      Object.is(previous[key], next[key]),
  );
}
