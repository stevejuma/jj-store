import {
  AppShellController,
  createLocalStorageAppShellLayoutPersistence,
} from "@lapismd/design-core/shell";
import {
  loadRepositoryShellLayout,
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
  return shell;
}
