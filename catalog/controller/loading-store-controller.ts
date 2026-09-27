import type {
  StoreController,
  StoreControllerState,
} from "../../web/src/lib/controller/contracts.js";

/** Catalog-only pending state over the same production controller contract. */
export function createLoadingStoreController(
  controller: StoreController,
): StoreController {
  return {
    get state(): StoreControllerState {
      const state = controller.state;
      const files = state.route.page === "files";
      return {
        ...state,
        bookmarks: [],
        choices: [],
        targets: [],
        graph: { revisions: [], devices: [] },
        detail: undefined,
        snapshot: undefined,
        blob: undefined,
        status: {
          ...state.status,
          navigation: "loading",
          bookmarks: "loading",
          graph: files ? "idle" : "loading",
          files: files ? "loading" : "idle",
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
        status: { ...state.status, detail: "loading" },
      };
    },
    actions: controller.actions,
  };
}
