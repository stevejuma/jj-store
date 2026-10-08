import { tick } from "svelte";
import type { StoreController } from "../controller/contracts.js";
import type { Model } from "../api/client.js";
import type { ConversationSession } from "./session.svelte.js";
import { sourceAnchor } from "./source-map.js";
import { commentSources } from "../ui/store-agent-chat-body/comment-navigation.js";
export async function openConversationSource(
  controller: StoreController,
  source: Model<"ConversationSourceV1">,
  itemId?: string,
) {
  const target = source.target,
    owner = controller.threads?.conversations;
  if (!owner) throw new Error("Conversation navigation is unavailable.");
  owner.destination = {
    source,
    itemId,
    repository: controller.state.repository.id,
  };
  if (
    target.type === "agent_message" ||
    target.type === "tool_output" ||
    target.type === "tool_invocation"
  )
    await controller.actions.selectAgentSession(target.session_id);
  else if (target.type === "human_message") {
    if (!controller.actions.openThreads)
      throw new Error("Thread navigation is unavailable.");
    await controller.actions.openThreads(
      target.thread_id,
      controller.state.repository.id,
    );
    for (
      let page = 0;
      page < 20 &&
      controller.threads?.hasOlder &&
      !controller.threads.projection.targets[target.message_id];
      page++
    )
      await controller.threads.older();
    const event = controller.threads?.events.find(
      (e) =>
        e.message?.message_id === target.message_id &&
        e.value.event.type === "message" &&
        e.value.event.revision.id === target.revision.id,
    );
    const parent = event?.message?.reply_to;
    if (parent) {
      const root = controller.threads?.events.find(
        (e) =>
          e.value.event.type === "message" &&
          e.value.event.revision.id === parent.id,
      )?.message;
      if (root)
        await controller.actions.openRoute?.({
          ...controller.state.route,
          replyId: root.message_id,
        });
    }
  } else
    throw new Error(
      "This review is available from its specialized review panel.",
    );
}
export async function revealConversationDestination(
  session: ConversationSession,
  source: Model<"ConversationSourceV1">,
  itemId?: string,
) {
  await tick();
  if (itemId && session.projection.items.some((i) => i.id === itemId)) {
    return !!(await session.comments.goToReply(itemId));
  }
  for (const { block } of commentSources(session.projection.items)) {
    const mapping = session.projection.sources.get(block.id),
      anchor = mapping && sourceAnchor(block, mapping, source);
    if (!anchor) continue;
    const opener = session.comments.root ?? document.body;
    return !!(await session.comments.goToSource(
      { id: "navigation", anchor, label: block.label, comments: [] },
      opener,
    ));
  }
  session.comments.error =
    "The exact destination is outside the loaded history or unavailable at this selection.";
  return false;
}

export function conversationNavigation(
  controller: StoreController | undefined,
  session: ConversationSession,
) {
  return {
    onNavigateSource: controller
      ? async (
          thread: import("../ui/store-agent-chat-body/model.js").ChatCommentThread,
        ) => {
          if (thread.source)
            await openConversationSource(controller, thread.source);
        }
      : undefined,
    onNavigateReply: controller
      ? async (id: string) => {
          if (session.projection.items.some((i) => i.id === id)) return;
          const answer = session.projection.answers?.get(id);
          if (!answer) throw new Error("The exact answer is unavailable.");
          await openConversationSource(controller, answer.answer, id);
        }
      : undefined,
  };
}

export async function consumeConversationDestination(
  controller: StoreController | undefined,
  session: ConversationSession,
) {
  const owner = controller?.threads?.conversations,
    target = owner?.destination,
    next = session.projection;
  if (
    !controller ||
    !owner ||
    !target ||
    target.repository !== controller.state.repository.id ||
    !next.items.length
  )
    return;
  const source = target.source.target;
  const relevant = [...next.sources.values()].some((s) =>
    source.type === "human_message"
      ? s.target.type === "human_message" &&
        s.target.thread_id === source.thread_id
      : "session_id" in source &&
        "session_id" in s.target &&
        s.target.session_id === source.session_id,
  );
  if (!relevant) return;
  try {
    if (
      (await revealConversationDestination(
        session,
        target.source,
        target.itemId,
      )) &&
      owner.destination === target
    )
      owner.destination = undefined;
  } catch (cause) {
    session.comments.error = String(cause);
  }
}
