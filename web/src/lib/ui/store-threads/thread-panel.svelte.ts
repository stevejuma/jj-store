import { getContext, setContext } from "svelte";
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
