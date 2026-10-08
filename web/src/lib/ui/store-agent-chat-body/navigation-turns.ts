import type {
  ChatBodyItem,
  ChatBodyTurn,
  ChatComment,
  ChatCommentThread,
} from "./model.js";
import { patchLines } from "./patch-lines.js";
export interface ChatNavigationTurn extends ChatBodyTurn {
  firstItemId: string;
  toolCount: number;
  changedFiles: number;
  additions: number;
  deletions: number;
  comments: { threadId: string; comment: ChatComment }[];
  commentsOnly?: boolean;
  pending?: boolean;
}
/** A turn is a prompt and its response/tool flow, never an individual tool row. */
export function navigationTurns(
  items: readonly ChatBodyItem[],
  threads: readonly ChatCommentThread[] = [],
): ChatNavigationTurn[] {
  const groups: { turn: ChatBodyTurn; items: ChatBodyItem[] }[] = [];
  for (const item of items) {
    const previous = groups.at(-1);
    const turn =
      item.turn ??
      (item.role !== "user" && previous
        ? previous.turn
        : {
            id: item.id,
            author: item.author?.name ?? "You",
            prompt:
              item.role === "user"
                ? item.blocks.map((block) => block.text).join("\n\n")
                : item.label,
            summary:
              item.role === "assistant"
                ? (item.blocks[0]?.text ?? item.label)
                : "",
          });
    if (previous?.turn.id === turn.id) previous.items.push(item);
    else groups.push({ turn, items: [item] });
  }
  const authored = threads.flatMap((thread) =>
    thread.comments
      .filter((comment) => !comment.replyTo && !!comment.text.trim())
      .map((comment) => ({ threadId: thread.id, comment })),
  );
  const turns: ChatNavigationTurn[] = groups.map(({ turn, items: content }) => {
    const tools = content.flatMap((item) => item.tools ?? []);
    const patches = tools.flatMap((tool) =>
      tool.blocks.filter((block) => block.kind === "patch"),
    );
    const lines = patches.flatMap((block) => patchLines(block.text));
    const batches = new Set(
      content.map((item) => item.commentBatchId).filter(Boolean),
    );
    const comments = authored.filter(
      ({ comment }) => comment.batchId && batches.has(comment.batchId),
    );
    const commentsOnly =
      comments.length > 0 &&
      !content[0].blocks.some((block) => !!block.text.trim());
    return {
      ...turn,
      prompt: commentsOnly ? commentPrompt(comments) : turn.prompt,
      comments,
      commentsOnly,
      firstItemId: content[0].id,
      toolCount: tools.length,
      changedFiles: new Set(patches.map((block) => block.path).filter(Boolean))
        .size,
      additions: lines.filter((line) => line.kind === "addition").length,
      deletions: lines.filter((line) => line.kind === "deletion").length,
    };
  });
  const pending = authored.filter(
    ({ comment }) => comment.status === "draft" || comment.status === "sending",
  );
  if (pending.length)
    turns.push({
      id: "pending-comments",
      firstItemId: "pending-comments",
      pending: true,
      author: pending[0].comment.author.name,
      prompt: commentPrompt(pending),
      summary: "",
      comments: pending,
      commentsOnly: true,
      agentName: turns.at(-1)?.agentName,
      modelLabel: turns.at(-1)?.modelLabel,
      createdAt: pending[0].comment.createdAt,
      toolCount: 0,
      changedFiles: 0,
      additions: 0,
      deletions: 0,
    });
  return turns;
}

function commentPrompt(comments: ChatNavigationTurn["comments"]) {
  return `${comments.length === 1 ? "Comment" : "Comments"}: ${comments.map(({ comment }) => comment.text).join("; ")}`;
}
