import { tick } from "svelte";
import type {
  ChatBodyItem,
  ChatBodyBlock,
  ChatBodyTool,
  ChatCommentThread,
  ChatComment,
} from "./model.js";
import type { ChatBodyComments } from "./comments.svelte.js";
import { rawToolBlock } from "./tool-blocks.js";
import { validAnchor } from "./anchors.js";
import { middleFoldKey } from "./middle-fold.js";
interface CommentSource {
  block: ChatBodyBlock;
  disclosures: string[];
  tool?: ChatBodyTool;
  raw?: boolean;
}
export function commentSources(
  items: readonly ChatBodyItem[],
): CommentSource[] {
  return items.flatMap((item) => [
    ...item.blocks.map((block) => ({
      block,
      disclosures: item.kind === "message" ? [] : [item.id],
    })),
    ...(item.tools ?? []).flatMap((tool) =>
      [...tool.blocks, rawToolBlock(tool)].map((block) => ({
        block,
        tool,
        raw: block.id === rawToolBlock(tool).id,
        disclosures: [item.id, tool.id, `file:${block.id}`],
      })),
    ),
  ]);
}
export async function revealCommentSource(
  state: ChatBodyComments,
  thread: ChatCommentThread,
  opener: HTMLElement,
) {
  const source = commentSources(state.props().items).find((s) =>
    validAnchor(s.block, thread.anchor),
  );
  if (!source) {
    await state.props().onNavigateSource?.(thread);
    return;
  }
  for (const id of source.disclosures) state.disclosures[id] = true;
  state.disclosures[middleFoldKey(source.block)] = true;
  if (source.tool) {
    state.rawTools[source.tool.id] = !!source.raw;
    if (source.tool.detailPlacement === "sidebar")
      await state.props().onInspectTool?.(source.tool, opener);
  }
  state.highlightedThread = thread.id;
  await tick();
  // Reader registration follows the component mount tick.
  await tick();
  const reader = state.readers.get(source.block.id);
  reader?.(thread.anchor, true);
  return !!reader;
}

export async function revealCommentMessage(
  state: ChatBodyComments,
  thread: ChatCommentThread,
  comment: ChatComment,
) {
  const pending = comment.status === "draft" || comment.status === "sending";
  if (!pending && !comment.batchId) return;
  state.disclosures[`batch:${pending ? "pending" : comment.batchId}`] = true;
  await tick();
  await tick();
  const selector = `[data-placement="${pending ? "queue" : "history"}"] [data-comment-id="${CSS.escape(comment.id)}"]`;
  const card = state.root?.querySelector<HTMLElement>(selector);
  if (!card) return;
  card.scrollIntoView({ block: "center" });
  if (pending) state.activate(thread, comment, "queue");
  else card.focus({ preventScroll: true });
}
