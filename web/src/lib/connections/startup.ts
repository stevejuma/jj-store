import type { StoreRouteBrowser } from "../controller/store-route-browser.js";
import { resumeGlobalSettingsHash } from "../controller/global-settings-route-codec.js";
import { readPreferences, preferenceKey } from "./preferences.js";
import { connectionHash, readConnectionHash } from "./transport.js";
export function connectionStartup(
  base: StoreRouteBrowser | undefined,
  storage: Storage | undefined,
) {
  let saved: string | null = null;
  try {
    saved = storage?.getItem(preferenceKey) ?? null;
  } catch {}
  const preferences = readPreferences(saved);
  const rootHash = base?.hash() ?? "";
  if (!rootHash || rootHash === "#/" || rootHash === "#") {
    const destination = preferences.routes[preferences.selected];
    if (destination)
      base?.replace(connectionHash(destination, preferences.selected));
    else if (preferences.selected === "local")
      base?.replace(resumeGlobalSettingsHash(rootHash, storage));
  }
  let parsed: { id: string; hash: string };
  try {
    parsed = readConnectionHash(base?.hash() ?? "");
  } catch {
    parsed = { id: "invalid", hash: "" };
  }
  return { preferences, parsed };
}
