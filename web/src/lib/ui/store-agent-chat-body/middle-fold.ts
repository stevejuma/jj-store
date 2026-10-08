import { Text } from "@codemirror/state";
import { Decoration, WidgetType, type EditorView } from "@codemirror/view";
import type { ChatBodyBlock } from "./model.js";
import type { ChatBodyComments } from "./comments.svelte.js";
import { validAnchor } from "./anchors.js";
type FoldState = Pick<ChatBodyComments, "disclosures" | "threads" | "active">;

export const middleFoldKey = (block: ChatBodyBlock) =>
  `middle:${block.id}:${block.revision}`;
export function middleRange(block: ChatBodyBlock) {
  if (block.kind !== "code" && block.kind !== "patch") return;
  const doc = Text.of(block.text.split("\n"));
  if (doc.lines <= 32) return;
  return {
    from: doc.line(13).from,
    to: doc.line(doc.lines - 7).from,
    lines: doc.lines - 20,
  };
}
class MiddleFoldWidget extends WidgetType {
  constructor(
    readonly block: ChatBodyBlock,
    readonly comments: FoldState,
    readonly range: NonNullable<ReturnType<typeof middleRange>>,
  ) {
    super();
  }
  eq(other: MiddleFoldWidget) {
    return (
      this.block.id === other.block.id &&
      this.block.revision === other.block.revision &&
      this.comments === other.comments
    );
  }
  get estimatedHeight() {
    return 28;
  }
  toDOM(view: EditorView) {
    const row = document.createElement("div");
    row.className = "chat-middle-fold";
    row.contentEditable = "false";
    const button = document.createElement("button");
    button.type = "button";
    button.dataset.chatFoldControl = "";
    button.textContent = `Show ${this.range.lines} more lines ›`;
    button.setAttribute(
      "aria-label",
      `Show ${this.range.lines} more lines in ${this.block.label}`,
    );
    button.addEventListener("click", () => {
      this.comments.disclosures[middleFoldKey(this.block)] = true;
      view.dispatch({ selection: { anchor: this.range.from } });
      view.focus();
    });
    row.append(button, document.createElement("span"));
    return row;
  }
  ignoreEvent() {
    return true;
  }
}
export function middleDecorations(block: ChatBodyBlock, comments: FoldState) {
  const range = middleRange(block);
  if (!range || comments.disclosures[middleFoldKey(block)])
    return Decoration.none;
  const anchors = [
    ...comments.threads.map((thread) => thread.anchor),
    ...(comments.active ? [comments.active.anchor] : []),
  ];
  if (
    anchors.some(
      (anchor) =>
        validAnchor(block, anchor) &&
        anchor.from < range.to &&
        anchor.to >= range.from,
    )
  )
    return Decoration.none;
  return Decoration.set([
    Decoration.replace({
      block: true,
      widget: new MiddleFoldWidget(block, comments, range),
    }).range(range.from, range.to),
  ]);
}
