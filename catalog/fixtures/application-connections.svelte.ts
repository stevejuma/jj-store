import type { ServerConnectionsView } from "../../web/src/lib/connections/presentation.js";
import type { StoreController } from "../../web/src/lib/controller/contracts.js";
import { createMemoryStoryStorage } from "./memory-storage.js";
/** No transports or device storage: the same connection presentation contract as App. */
export function applicationConnections(
  controller: StoreController,
  storage = createMemoryStoryStorage(),
): ServerConnectionsView {
  const state = $state({
    dialog: false,
    error: "",
    editing: null as ServerConnectionsView["editing"],
  });
  return {
    controller,
    workspaces: [{ id: "local", controller, layoutStorage: storage }],
    selected: "local",
    supported: false,
    records: [],
    busy: false,
    get dialog() {
      return state.dialog;
    },
    set dialog(value) {
      state.dialog = value;
    },
    get error() {
      return state.error;
    },
    get editing() {
      return state.editing;
    },
    select() {},
    manage() {
      state.dialog = true;
    },
    add() {
      state.dialog = true;
    },
    async connect() {
      state.error = "This catalog connection uses in-memory fixture data.";
    },
    async rename() {},
    async remove() {},
    async signOut() {},
    guard() {
      return () => false;
    },
    shareUrl(hash, href) {
      return new URL(hash, href).href;
    },
  };
}
