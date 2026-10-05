import type { StoreController } from "../../controller/contracts.js";

export function settingsNavigationReason(
  settings: StoreController["state"]["settings"],
): string {
  if (settings.searchSettings.dirty)
    return "Save or cancel your Search changes before changing repositories.";
  if (settings.workspacePath)
    return "Link or cancel your folder draft before changing repositories.";
  if (settings.storageSettings.pending)
    return "Resolve the pending Storage request before changing repositories.";
  if (
    settings.workspaceLinking ||
    settings.workspacePicking ||
    settings.searchSettings.changing ||
    settings.storageSettings.changing ||
    settings.syncSettings.changing ||
    settings.indexPreparation === "loading" ||
    ["capturing", "waiting"].includes(settings.syncSettings.wait.state)
  )
    return "Wait for the current Settings action to finish before changing repositories.";
  return "";
}
