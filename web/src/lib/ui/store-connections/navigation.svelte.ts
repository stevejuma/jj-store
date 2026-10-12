import type { StoreController } from "../../controller/contracts.js";
import { serverConnections } from "../../connections/context.js";
import { provideProfiles } from "../../profiles/context.svelte.js";
import { provideTerminalControls } from "../../terminal/controls.svelte.js";
import type { StoreSettingsSection } from "../store-settings/model.js";
import type { createStoreShell } from "../store-application/create-store-shell.js";
export function connectionSettingsNavigation(
  controller: () => StoreController,
  blocked: () => boolean,
  shell: ReturnType<typeof createStoreShell>,
) {
  const showMain = () => {
    if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
  };
  return {
    openSettings(section?: StoreSettingsSection) {
      if (blocked()) return;
      controller().actions.openGlobalSettings(
        section ??
          (controller().state.accessMode === "authenticated"
            ? "profile"
            : undefined),
      );
      showMain();
    },
    selectSettings(section: StoreSettingsSection) {
      controller().actions.selectGlobalSettings(section);
      showMain();
    },
    closeSettings() {
      void controller().actions.leaveGlobalSettings();
      showMain();
    },
  };
}
export function ownConnectionNavigation(
  controller: () => StoreController,
  blocked: () => boolean,
  settings: () => void,
) {
  provideProfiles(() => controller().directory);
  provideTerminalControls();
  const connections = serverConnections();
  $effect(() => connections?.guard(controller(), () => !blocked()));
  return profileMenuActions(controller, connections, settings);
}
export function profileMenuActions(
  controller: () => StoreController,
  connections: ReturnType<typeof serverConnections>,
  settings: () => void,
) {
  return (action: string) => {
    if (action === "settings") settings();
    else if (action === "sign-in") controller().actions.signIn();
    else if (connections && connections.selected !== "local")
      void connections.signOut(connections.selected);
    else controller().actions.signOut();
  };
}
