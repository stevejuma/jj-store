import {
  AppShellController,
  createLocalStorageAppShellLayoutPersistence,
} from "@lapismd/design-core/shell";
import {
  loadRepositoryShellLayout,
  repositoryDetailsPanelId,
  repositoryShellStorageKey,
} from "../layout-storage.js";
export function createStoreShell(
  storage: Storage | undefined,
  initiallyCollapsed: boolean,
) {
  const initialLayout = loadRepositoryShellLayout(storage);
  const shell = new AppShellController({
    leftCollapsed: initialLayout.left?.collapsed ?? initiallyCollapsed,
    leftClosed: initialLayout.left?.closed ?? false,
    ...(initialLayout.left?.width === undefined
      ? {}
      : { leftWidth: initialLayout.left.width }),
    rightCollapsed: initialLayout.right?.collapsed ?? false,
    rightClosed: initialLayout.right?.closed ?? true,
    rightWidth: initialLayout.right?.width ?? 320,
    sidebarMinWidth: 48,
    sidebarMaxWidth: 440,
    ...(storage
      ? {
          persistence: createLocalStorageAppShellLayoutPersistence(
            repositoryShellStorageKey,
            storage,
          ),
        }
      : {}),
  });
  // Match Community's standard pane plus independently persisted outer rail.
  // Seed the rail from the formerly shared value only for existing layouts.
  const rail = initialLayout.repositoryDetails ?? initialLayout.right;
  shell.createSidebar(repositoryDetailsPanelId, "right", {
    collapsed: rail?.collapsed ?? false,
    closed: rail?.closed ?? true,
    width: rail?.width ?? 280,
    minWidth: 220,
    maxWidth: 440,
  });
  return shell;
}
