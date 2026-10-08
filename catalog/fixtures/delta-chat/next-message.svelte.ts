import type {
  ChatBodyItem,
  ChatPromptSubmission,
} from "../../../web/src/lib/ui/store-agent-chat-body/model.js";
import type { ChatBodyComments } from "../../../web/src/lib/ui/store-agent-chat-body/comments.svelte.js";
export const demoSender = {
  id: "demo-user",
  name: "You",
  handle: "",
  initials: "YO",
};
export const demoAgent = {
  id: "demo-agent",
  name: "Demo agent",
  handle: "",
  initials: "AI",
};
export class DemoNextMessage {
  messages = $state<ChatBodyItem[]>([]);
  failNext = $state(false);
  submissions = $state<ChatPromptSubmission[]>([]);
  async accept(submission: ChatPromptSubmission) {
    await Promise.resolve();
    if (this.failNext) {
      this.failNext = false;
      throw new Error(
        "Simulated send failed. Your message and comments are retained.",
      );
    }
    this.submissions.push(submission);
    const { id, text, comments } = submission;
    this.messages.push({
      id,
      kind: "message",
      role: "user",
      label: "Local message from You",
      author: demoSender,
      commentBatchId: comments.length ? id : undefined,
      turn: {
        id: `turn:${id}`,
        author: "You",
        prompt: text || `${comments.length} comments`,
        summary: "Local demo message; awaiting simulated replies.",
      },
      blocks: text.trim()
        ? [
            {
              id: `${id}/text`,
              revision: id,
              kind: "markdown",
              label: "Local message",
              text,
            },
          ]
        : [],
    });
  }
  completeReplies(state: ChatBodyComments) {
    for (const submission of this.submissions) {
      for (const comment of submission.comments) {
        if (state.findComment(comment.commentId)?.status !== "sent") continue;
        const id = `demo-reply:${comment.commentId}`;
        const text = `Simulated reply: I received your comment about “${comment.anchor.quote.slice(0, 100)}”. No agent was contacted.`;
        state.recordReply(
          comment.threadId,
          comment.commentId,
          {
            id,
            author: demoAgent,
            text,
            status: "sent",
          },
          id,
        );
        const userIndex = this.messages.findIndex(
          (item) => item.id === submission.id,
        );
        const nextTurn = this.messages.findIndex(
          (item, index) => index > userIndex && item.role === "user",
        );
        this.messages.splice(
          nextTurn < 0 ? this.messages.length : nextTurn,
          0,
          {
            id,
            kind: "message",
            role: "assistant",
            author: demoAgent,
            label: "Simulated agent reply",
            turn: {
              id: `turn:${submission.id}`,
              author: "You",
              prompt:
                submission.text || `${submission.comments.length} comments`,
              summary: "Simulated reply; no agent was contacted.",
            },
            replyContext: {
              threadId: comment.threadId,
              commentIds: [comment.commentId],
            },
            blocks: [
              {
                id: `${id}/text`,
                revision: id,
                kind: "markdown",
                label: "Simulated reply",
                text,
              },
            ],
          },
        );
      }
    }
  }
}
