import type {
  StoreController,
  StoreControllerState,
} from "../../web/src/lib/controller/contracts.js";

/** Catalog-only pending state over the same production controller contract. */
export function createLoadingStoreController(
  controller: StoreController,
  connecting = false,
): StoreController {
  return {
    get state(): StoreControllerState {
      const state = controller.state;
      const files = state.route.page === "files";
      return {
        ...state,
        bookmarks: [],
        workingCopies: [],
        choices: [],
        targets: [],
        graph: { revisions: [], devices: [] },
        detail: undefined,
        snapshot: undefined,
        blob: undefined,
        agentSessions: [],
        agentSession: undefined,
        reviewThreads: [],
        reviewThread: undefined,
        status: {
          ...state.status,
          connection: connecting ? "connecting" : state.status.connection,
          navigation: connecting ? "idle" : "loading",
          bookmarks: connecting ? "idle" : "loading",
          graph: connecting || files ? "idle" : "loading",
          files: !connecting && files ? "loading" : "idle",
          agents: connecting ? "idle" : "loading",
          reviews: connecting ? "idle" : "loading",
          reviewAction: "idle",
          detail: "idle",
        },
      };
    },
    actions: controller.actions,
  };
}

export function createDetailLoadingStoreController(
  controller: StoreController,
): StoreController {
  return {
    get state(): StoreControllerState {
      const state = controller.state;
      return {
        ...state,
        detail: undefined,
        blob: undefined,
        reviewThread: undefined,
        status: { ...state.status, detail: "loading", reviewAction: "loading" },
      };
    },
    actions: controller.actions,
  };
}
