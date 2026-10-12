import type { ThreadsController } from "../controller/thread-types.js";
import type { StoreController } from "../controller/contracts.js";
import type { ConversationSession } from "./session.svelte.js";
import type {
  ChatPromptSubmission,
  ChatSubmissionResult,
} from "../ui/store-agent-chat-body/model.js";
import { mentionedAgentIds } from "../community/agent-mentions.js";
export async function submitThreadConversation(
  context: {
    threads: ThreadsController;
    controller?: StoreController;
    conversation: ConversationSession;
    disabled: boolean;
    id: string;
    parent?: string;
  },
  submission: ChatPromptSubmission,
): Promise<ChatSubmissionResult> {
  const { threads, controller, conversation, disabled, id, parent } = context;
  if (disabled || !threads.detail)
    throw new Error(
      "This conversation is unavailable for sending. Your drafts are retained.",
    );
  const batch = conversation.prepare(
    submission,
    { id, revision: threads.detail.thread.reference },
    true,
  );
  if (parent) {
    const target = threads.projection.targets[parent];
    if (target?.type !== "human_message")
      throw new Error("Reply parent is unavailable.");
    batch.reply_to = target.revision;
  }
  if (controller?.directory?.available())
    batch.emojis = await controller.directory.snapshots(
      [batch.text ?? "", ...batch.comments.map((c) => c.body)].join("\n"),
    );
  const agents = mentionedAgentIds(submission.text);
  if (agents.length) {
    const manager = controller?.execution;
    if (!manager?.available())
      throw new Error("Agent execution is unavailable on this device.");
    manager.chat.showContext(controller!, false, parent);
    manager.chat.close();
    batch.message_id = null;
    batch.text = null;
    try {
      await manager.chat.send(
        submission.text,
        agents,
        submission.comments.length ? batch : undefined,
        submission.id,
      );
      await threads.refresh();
      return manager.promptQueue.submitted(
        submission,
        manager.chat.receipts[submission.id],
      );
    } catch (cause) {
      if (manager.chat.receipts[submission.id])
        return manager.promptQueue.submitted(
          submission,
          manager.chat.receipts[submission.id],
        );
      if (manager.chat.pending?.request_id === submission.id)
        return { publication: "unknown", error: manager.chat.error };
      throw cause;
    }
  }
  try {
    await threads.submitConversation({
      version: 1,
      request_id: submission.id,
      command: { type: "submit", submission: batch },
    });
    return { publication: "published" };
  } catch (cause) {
    if (threads.committedRequestId === submission.id)
      return { publication: "published" };
    if (threads.pending?.request.request_id === submission.id)
      return { publication: "unknown", error: threads.error };
    throw cause;
  }
}
