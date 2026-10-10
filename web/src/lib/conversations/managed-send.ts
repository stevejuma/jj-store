import type { AgentExecution } from "../controller/agent-execution.svelte.js";
import type { ConversationSession } from "./session.svelte.js";
import type {
  ChatPromptSubmission,
  ChatSubmissionResult,
} from "../ui/store-agent-chat-body/model.js";
import { mentionedAgentIds } from "../community/agent-mentions.js";
export async function submitManagedConversation(
  manager: AgentExecution,
  conversation: ConversationSession,
  submission: ChatPromptSubmission,
): Promise<ChatSubmissionResult> {
  const chat = manager.chat;
  const explicit = mentionedAgentIds(submission.text);
  const addressed = explicit.length
    ? explicit
    : [
        ...new Set(
          chat.sessions.flatMap((s) => (s.setup_id ? [s.setup_id] : [])),
        ),
      ];
  if (!addressed.length)
    throw new Error(
      "Choose an agent with @ before sending. Your draft is retained.",
    );
  let comments;
  if (submission.comments.length) {
    const saved = manager.value?.conversations.find(
      (c) => c.id === chat.selected,
    );
    if (!saved?.discussion_thread)
      throw new Error(
        "The conversation must be published before adding comments.",
      );
    const threads = manager.discussions.reader(saved.home);
    await threads.open(saved.discussion_thread);
    if (threads.detail?.thread.value.thread_id !== saved.discussion_thread)
      throw new Error(
        "The conversation is unavailable. Your draft is retained.",
      );
    comments = conversation.prepare(submission, {
      id: saved.discussion_thread,
      revision: threads.detail.thread.reference,
    });
  }
  try {
    await chat.send(submission.text, addressed, comments, submission.id);
    return manager.promptQueue.submitted(
      submission,
      chat.receipts[submission.id],
    );
  } catch (cause) {
    const state = chat.publications[submission.id]?.state;
    if (chat.receipts[submission.id])
      return manager.promptQueue.submitted(
        submission,
        chat.receipts[submission.id],
      );
    if (chat.pending?.request_id === submission.id)
      return { publication: "unknown", error: chat.error };
    throw cause;
  }
}
