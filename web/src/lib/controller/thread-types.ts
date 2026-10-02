export type { Model } from "../api/client.js";
import type { Exact, Model } from "../api/client.js";
import type { operations } from "../api/generated.js";
export type ThreadRequest = Exact<
  NonNullable<
    operations["collab_thread_apply"]["requestBody"]
  >["content"]["application/json"]
>;
export type ReactionRequest = Exact<
  NonNullable<
    operations["collab_reaction_apply"]["requestBody"]
  >["content"]["application/json"]
>;
export type ReactionTarget = Model<"ReactionTargetV1">;
export type ReactionItems = Extract<
  Model<"CollabPageItems">,
  { type: "reactions" }
>;
export interface ThreadTransport {
  list(cursor?: string): Promise<Model<"CollabPage">>;
  thread(id: string): Promise<Model<"ThreadDetailV1">>;
  events(id: string, cursor?: string): Promise<Model<"CollabPage">>;
  emojis(cursor?: string): Promise<Model<"CollabPage">>;
  reactions(
    target: ReactionTarget,
    cursor?: string,
  ): Promise<Model<"CollabPage">>;
  asset(id: string): Promise<{ bytes: Uint8Array; type: string }>;
  command(
    kind: "threads" | "reactions",
    request: ThreadRequest | ReactionRequest,
  ): Promise<Model<"CollabCommandOutcome">>;
  status(
    kind: "threads" | "reactions",
    id: string,
  ): Promise<Model<"CollabCommandStatus">>;
}
export type ThreadsController = ReturnType<
  typeof import("./production-threads.svelte.js").createThreadsController
>;
export type ReactionsController = Pick<
  ThreadsController,
  | "busy"
  | "pending"
  | "emojis"
  | "reactionError"
  | "getReaction"
  | "reactionModels"
  | "loadEmojis"
  | "loadReactions"
  | "toggle"
  | "checkPending"
  | "retryPending"
  | "reactionRevision"
>;
