import type { StoreController } from "../controller/contracts.js";
import type { ThreadsController } from "../controller/thread-types.js";
import type { ConversationSession } from "./session.svelte.js";
import type {
  ChatPromptSubmission,
  ChatSubmissionResult,
} from "../ui/store-agent-chat-body/model.js";
import { mentionedAgentIds } from "../community/agent-mentions.js";
import { conversationSubmissionResult } from "./submission-result.js";
export async function submitRecordedConversation(
  context: {
    controller: StoreController;
    threads: ThreadsController;
    conversation: ConversationSession;
    sessionId: string;
    parentId: string;
    identity: (s: string) => string;
  },
  submission: ChatPromptSubmission,
): Promise<ChatSubmissionResult> {
  const { controller, threads, conversation, sessionId, parentId, identity } =
    context;
  const parent = await threads.ensureConversation(parentId, identity("create"));
  const batch = conversation.prepare(
    submission,
    { id: parentId, revision: parent.thread.reference },
    true,
  );
  const agents = mentionedAgentIds(submission.text);
  try {
    if (agents.length) {
      const manager = controller.execution;
      if (!manager?.available())
        throw new Error(
          "Agent execution is unavailable. Your draft is retained.",
        );
      manager.chat.show(
        {
          scope: `recorded:${controller.state.repository.id}:${sessionId}`,
          home: controller.state.repository.id,
          thread: parentId,
          agent_target: null,
          context: `Recorded session ${sessionId}`,
        },
        [],
        undefined,
        false,
      );
      batch.text = null;
      batch.message_id = null;
      await manager.chat.send(
        submission.text,
        agents,
        submission.comments.length ? batch : undefined,
        submission.id,
      );
      await threads.loadConversation(parentId);
      return conversationSubmissionResult(manager.chat.receipts[submission.id]);
    }
    await threads.submitConversation({
      version: 1,
      request_id: submission.id,
      command: { type: "submit", submission: batch },
    });
    await threads.loadConversation(parentId);
    await controller.actions.refreshAgentConversation?.();
    return { publication: "published" };
  } catch (cause) {
    if (controller.execution?.chat.receipts[submission.id])
      return conversationSubmissionResult(
        controller.execution.chat.receipts[submission.id],
      );
    if (threads.committedRequestId === submission.id)
      return { publication: "published" };
    if (
      threads.pending?.request.request_id === submission.id ||
      controller.execution?.chat.pending?.request_id === submission.id
    )
      return {
        publication: "unknown",
        error: threads.error || controller.execution?.chat.error,
      };
    throw cause;
  }
}
