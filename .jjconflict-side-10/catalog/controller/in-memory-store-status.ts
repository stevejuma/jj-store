import type { StoreControllerStatus } from "../../web/src/lib/controller/contracts.js";

export function inMemoryStoreStatus(input: {
  authenticated: boolean;
  hasBookmarks: boolean;
  hasFiles: boolean;
  hasDetail: boolean;
  hasAgentSession: boolean;
  hasAgents: boolean;
  hasReviews: boolean;
  reviewAction: StoreControllerStatus["reviewAction"];
  search: StoreControllerStatus["search"];
}): StoreControllerStatus {
  return {
    connection: input.authenticated ? "connected" : "disconnected",
    navigation: "ready",
    repositories: "ready",
    bookmarks: input.hasBookmarks ? "ready" : "empty",
    graph: "ready",
    files: input.hasFiles ? "ready" : "empty",
    detail: input.hasDetail ? "ready" : "idle",
    search: input.search,
    agents: input.hasAgents ? "ready" : "empty",
    transcript: input.hasAgentSession ? "ready" : "idle",
    reviews: input.hasReviews ? "ready" : "empty",
    reviewAction: input.reviewAction,
    live: input.authenticated ? "live" : "disconnected",
    refreshAvailable: false,
    error: "",
  };
}
