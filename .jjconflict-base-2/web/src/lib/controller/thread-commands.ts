import { ApiError, type Model } from "../api/client.js";
import { collabRequestId } from "./collab-actions.svelte.js";
import type { ThreadRequest } from "./thread-types.js";
import type { channelMessages } from "../collaboration/thread-presentation.js";
export function createThreadCommands(options: {
  detail(): Model<"ThreadDetailV1"> | undefined;
  projection(): ReturnType<typeof channelMessages>;
  events(): Model<"ThreadEventItemV1">[];
  execute(kind: "threads", request: ThreadRequest): Promise<void>;
}) {
  async function send(body: string, parentId?: string, editId?: string) {
    const detail = options.detail();
    if (!detail) throw new ApiError("select_thread_first");
    const existing = editId ? options.projection().targets[editId] : undefined;
    const parent = parentId
      ? options.projection().targets[parentId]
      : undefined;
    const old = editId
      ? options.events().find((e) => e.message?.message_id === editId)?.message
      : undefined;
    if (parentId && parent?.type !== "human_message")
      throw new ApiError("reply_parent_unavailable");
    await options.execute("threads", {
      version: 1,
      request_id: collabRequestId(),
      command: {
        type: "message",
        thread_id: detail.thread.value.thread_id,
        expected_thread: detail.thread.reference,
        message_id: editId ?? collabRequestId(),
        expected_revision:
          existing?.type === "human_message" ? existing.revision : null,
        reply_to:
          old?.reply_to ??
          (parent?.type === "human_message" ? parent.revision : null),
        body,
      },
    });
  }
  async function rename(title: string) {
    const detail = options.detail();
    if (!detail) throw new ApiError("select_thread_first");
    await options.execute("threads", {
      version: 1,
      request_id: collabRequestId(),
      command: {
        type: "rename",
        thread_id: detail.thread.value.thread_id,
        expected: detail.thread.reference,
        title: title.trim(),
      },
    });
  }
  async function lifecycle(action: Model<"ThreadActionV1">) {
    const detail = options.detail();
    if (!detail) throw new ApiError("select_thread_first");
    await options.execute("threads", {
      version: 1,
      request_id: collabRequestId(),
      command: {
        type: "lifecycle",
        thread_id: detail.thread.value.thread_id,
        expected: detail.thread.reference,
        action,
      },
    });
  }
  return { send, rename, lifecycle };
}
