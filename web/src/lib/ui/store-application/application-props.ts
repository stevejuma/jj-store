import type { AppShellDisplayMode } from "@lapismd/design-core/shell";
import type { Snippet } from "svelte";
import type { StoreController } from "../../controller/contracts.js";
import type { StorePage } from "../community-navigation/model.js";

import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";

export interface StoreApplicationProps {
  /** Optional local presentation adapter; absent until backend integration is approved. */
  stackWorkspace?: StackWorkspaceController;
  controller: StoreController;
  initiallyCollapsed?: boolean;
  displayMode?: AppShellDisplayMode;
  layoutStorage?: Storage;
  desktopChrome?: boolean;
  children?: Snippet<[StorePage, string]>;
}
