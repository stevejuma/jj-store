import {
  globalSettingsDestination,
  type GlobalSettingsDestination,
} from "./global-settings-navigation.js";
import { projectsRouteFromHash } from "./projects-route-codec.js";
import type { StoreRouteHashResult } from "./store-route-codec.js";
export function globalSettingsHash(
  destination: GlobalSettingsDestination,
): string | null {
  const valid = globalSettingsDestination(destination.section, destination.tab);
  return valid ? `#/settings?${new URLSearchParams(valid)}` : null;
}
export function readGlobalSettingsHash(
  hash: string,
): StoreRouteHashResult | undefined {
  if (!hash.startsWith("#/settings")) return;
  const [path, ...queryParts] = hash.split("?");
  const invalid = {
    kind: "invalid" as const,
    message: "Invalid global settings route.",
  };
  if (path !== "#/settings" || queryParts.length > 1) return invalid;
  const query = new URLSearchParams(queryParts[0] ?? "");
  if (
    [...query.keys()].some(
      (key) =>
        !["section", "tab"].includes(key) || query.getAll(key).length !== 1,
    )
  )
    return invalid;
  const destination = globalSettingsDestination(
    query.get("section") ?? "service",
    query.get("tab"),
  );
  if (!destination) return invalid;
  return {
    kind: "route",
    route: {
      ...projectsRouteFromHash("#/projects?tab=projects")!,
      globalSettings: destination,
    },
  };
}
const key = "jj-store/settings-destination/v1";
type DeviceStorage = Pick<Storage, "getItem" | "setItem" | "removeItem">;
/** Explicit URLs win. Only a root startup resumes the last Settings destination. */
export function resumeGlobalSettingsHash(
  hash: string,
  storage?: DeviceStorage,
): string {
  if (hash && hash !== "#" && hash !== "#/") return hash;
  try {
    const saved = JSON.parse(storage?.getItem(key) ?? "null");
    if (
      !saved ||
      typeof saved.section !== "string" ||
      typeof saved.tab !== "string"
    )
      return hash;
    const destination = globalSettingsDestination(saved.section, saved.tab);
    return destination ? globalSettingsHash(destination)! : hash;
  } catch {
    return hash;
  }
}
export function rememberGlobalSettingsHash(
  hash: string,
  storage?: DeviceStorage,
) {
  try {
    const parsed = readGlobalSettingsHash(hash);
    if (parsed?.kind === "route")
      storage?.setItem(key, JSON.stringify(parsed.route.globalSettings));
    else if (parsed?.kind !== "invalid") storage?.removeItem(key);
  } catch {
    /* Unavailable device storage never blocks navigation. */
  }
}
