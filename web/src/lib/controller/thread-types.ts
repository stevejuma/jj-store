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
export type ThreadSnapshot =
  Model<"CollabReadSnapshotV1"> | Model<"LocalCollabSnapshot">;
export type ThreadPage = Omit<Model<"CollabPage">, "snapshot"> & {
  snapshot: ThreadSnapshot;
};
export type ThreadDetail = Omit<Model<"ThreadDetailV1">, "snapshot"> & {
  snapshot: ThreadSnapshot;
};
export function threadSnapshotKey(snapshot: ThreadSnapshot): string | null {
  return "metadata_commit" in snapshot
    ? JSON.stringify([
        snapshot.profile,
        snapshot.repo_id,
        snapshot.metadata_commit,
      ])
    : snapshot.thread_root;
}
export type ReactionItems = Extract<
  Model<"CollabPageItems">,
  { type: "reactions" }
>;
export interface ThreadTransport {
  invalidate?(
    scope: "all" | "threads" | "reactions" | "emojis",
    target?: ReactionTarget,
  ): Promise<void>;
  cancelReads?(): Promise<void>;
  list(cursor?: string, fresh?: boolean): Promise<ThreadPage>;
  thread(id: string): Promise<ThreadDetail>;
  events(id: string, cursor?: string): Promise<ThreadPage>;
  emojis(cursor?: string): Promise<ThreadPage>;
  reactions(target: ReactionTarget, cursor?: string): Promise<ThreadPage>;
  asset(id: string): Promise<{ bytes: Uint8Array; type: string }>;
  command(
    kind: "threads" | "reactions",
    request: ThreadRequest | ReactionRequest,
  ): Promise<Model<"CollabCommandOutcome"> | Model<"LocalCollabOutcome">>;
  status(
    kind: "threads" | "reactions",
    id: string,
  ): Promise<Model<"CollabCommandStatus"> | Model<"LocalCollabOutcome">>;
}
export type ThreadsController = ReturnType<
  typeof import("./production-threads.svelte.js").createThreadsController
>;
export type ReactionsController = {
  setProfiles?(
    value: import("./directory.svelte.js").DirectoryManagement | undefined,
  ): void;
} & Pick<
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
