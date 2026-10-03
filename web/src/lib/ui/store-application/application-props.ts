import type { AppShellDisplayMode } from "@lapismd/design-core/shell";
import type { Snippet } from "svelte";
import type { StoreController } from "../../controller/contracts.js";
import type { StorePage } from "../community-navigation/model.js";

export interface StoreApplicationProps {
  controller: StoreController;
  initiallyCollapsed?: boolean;
  displayMode?: AppShellDisplayMode;
  layoutStorage?: Storage;
  desktopChrome?: boolean;
  children?: Snippet<[StorePage, string]>;
}
