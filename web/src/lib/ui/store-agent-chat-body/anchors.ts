import type { ChatBodyBlock, ChatCommentAnchor } from "./model.js";

export function anchorKey(anchor: ChatCommentAnchor) {
  return JSON.stringify([
    anchor.blockId,
    anchor.revision,
    anchor.from,
    anchor.to,
  ]);
}
export function commentAnchor(
  block: ChatBodyBlock,
  selection: { from: number; to: number },
): ChatCommentAnchor {
  let from = Math.max(0, Math.min(selection.from, block.text.length));
  let to = Math.max(from, Math.min(selection.to, block.text.length));
  if (
    block.wholeSection ||
    block.kind === "custom" ||
    block.kind === "context"
  ) {
    from = 0;
    to = block.text.length;
  } else if (from === to) {
    if (block.kind === "markdown") {
      const before = block.text.slice(0, from);
      const previous = [...before.matchAll(/\n\s*\n/g)].at(-1);
      from = previous ? previous.index + previous[0].length : 0;
      const next = /\n\s*\n/.exec(block.text.slice(to));
      to = next ? to + next.index : block.text.length;
    } else {
      from = from === 0 ? 0 : block.text.lastIndexOf("\n", from - 1) + 1;
      const end = block.text.indexOf("\n", to);
      to = end < 0 ? block.text.length : end;
    }
  }
  return {
    blockId: block.id,
    revision: block.revision,
    from,
    to,
    quote: block.text.slice(from, to),
  };
}
export function validAnchor(block: ChatBodyBlock, anchor: ChatCommentAnchor) {
  return (
    anchor.blockId === block.id &&
    anchor.revision === block.revision &&
    anchor.from >= 0 &&
    anchor.to <= block.text.length &&
    anchor.from <= anchor.to &&
    block.text.slice(anchor.from, anchor.to) === anchor.quote
  );
}
