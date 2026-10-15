import { setContext } from "svelte";
import type { StoreController } from "../../controller/contracts.js";
import type { CommunityNavigationItemModel } from "../community-navigation/model.js";
import { settingsNavigationReason } from "./settings-navigation.js";
import {
  repositorySettingsGuardKey,
  sameRepositorySettingsOwner,
  type RepositorySettingsDraftGuard,
} from "./repository-settings-guards.js";

/** One owner-change policy for sidebar callbacks, commands and browser history. */
export function ownRepositorySettingsGuard(options: {
  controller(): StoreController;
  blocked(): boolean;
  stacksLocked(): boolean;
  showMain(): void;
}) {
  const drafts = $state<RepositorySettingsDraftGuard>({ folders: "" });
  setContext(repositorySettingsGuardKey, drafts);
  let notice = $state("");
  function reason(ownerChange: boolean) {
    const controller = options.controller(),
      sync = controller.configuration?.state.drafts.sync;
    return options.blocked() || options.stacksLocked()
      ? "Finish the current repository action before navigating."
      : ownerChange
        ? settingsNavigationReason(controller.state.settings) ||
          drafts.folders ||
          (sync?.dirty ||
          sync?.unknown ||
          controller.configuration?.state.saving
            ? "Save or resolve the Sync configuration before changing repositories."
            : "")
        : "";
  }
  $effect(() => {
    const controller = options.controller();
    controller.actions.setNavigationGuard?.(
      "settings-and-editing",
      (previous, next) => {
        notice = sameRepositorySettingsOwner(previous, next)
          ? ""
          : reason(
              previous.repository !== next.repository ||
                Boolean(next.projectsHub || next.agentsHub || next.threadsHub),
            );
        return notice;
      },
    );
    return () =>
      controller.actions.setNavigationGuard?.("settings-and-editing");
  });
  return {
    get notice() {
      return notice;
    },
    open(
      section?: import("../../controller/repository-settings-navigation.js").RepositorySettingsSection,
    ) {
      if (reason(false)) return;
      void options.controller().actions.openRepositorySettings(section);
      options.showMain();
    },
    allow(item: CommunityNavigationItemModel) {
      const current = options.controller().state.route;
      notice = reason(
        item.kind === "repository"
          ? item.id !== current.repository
          : ["projects", "project", "agents", "threads", "channel"].includes(
              item.kind,
            ),
      );
      return !notice;
    },
  };
}
