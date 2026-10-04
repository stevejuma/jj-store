import type {
  StoreController,
  StoreControllerState,
  StoreLoadStatus,
} from "../../web/src/lib/controller/contracts.js";

export type CatalogCollaborationState =
  "ready" | "loading" | "empty" | "failed";

export function createCollaborationStateStoreController(
  controller: StoreController,
  mode: CatalogCollaborationState,
): StoreController {
  if (mode === "ready") return controller;
  return {
    get state(): StoreControllerState {
      const state = controller.state;
      const area = state.route.page === "reviews" ? "reviews" : "agents";
      const status = mode as StoreLoadStatus;
      return {
        ...state,
        agentSessions: [],
        agentSession: undefined,
        agentTranscript: [],
        reviewThreads: [],
        reviewThread: undefined,
        status: {
          ...state.status,
          [area]: status,
          transcript: "idle",
          reviewAction: "idle",
          error:
            mode === "failed"
              ? "The collaboration snapshot could not be loaded."
              : "",
        },
      };
    },
    actions: controller.actions,
  };
}
