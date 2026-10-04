import { getContext, setContext, untrack } from "svelte";
const key = Symbol("store-thread-panel");
export type ThreadPanelKind = "settings" | "members" | "replies";
export function createThreadPanel(
  openShell: () => void,
  closeShell: () => void,
) {
  let kind = $state<ThreadPanelKind>("settings");
  const panel = {
    get kind() {
      return kind;
    },
    open(next: ThreadPanelKind) {
      kind = next;
      openShell();
    },
    close: closeShell,
  };
  setContext(key, panel);
  return panel;
}
export function useThreadPanel() {
  return getContext<ReturnType<typeof createThreadPanel>>(key);
}

import type { StoreController } from "../../controller/contracts.js";
import type { AppearanceStore } from "../../appearance/store.svelte.js";
export function createApplicationThreadPanel(
  getController: () => StoreController,
  shell: { right: { open(): void; close(): void } },
  appearance: AppearanceStore,
) {
  const controller = untrack(getController);
  const panel = createThreadPanel(
    () => {
      if (panel.kind !== "replies" || appearance.live.threads !== "focus")
        shell.right.open();
    },
    () => shell.right.close(),
  );
  $effect(() => {
    const route = controller.state.route;
    if (route.page === "threads" && route.replyId) {
      const focus = appearance.live.threads === "focus";
      untrack(() => {
        panel.open("replies");
        if (focus) shell.right.close();
      });
    }
  });
  return {
    ...panel,
    get kind() {
      return panel.kind;
    },
    get focused() {
      return (
        controller.state.route.page === "threads" &&
        panel.kind === "replies" &&
        Boolean(controller.state.route.replyId) &&
        appearance.live.threads === "focus"
      );
    },
  };
}
