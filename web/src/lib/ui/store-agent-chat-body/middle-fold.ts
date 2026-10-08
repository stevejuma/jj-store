import { Text } from "@codemirror/state";
import { Decoration, WidgetType, type EditorView } from "@codemirror/view";
import { mount, unmount } from "svelte";
import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
import type { ChatBodyBlock } from "./model.js";
import type { ChatBodyComments } from "./comments.svelte.js";
import { validAnchor } from "./anchors.js";
type FoldState = Pick<ChatBodyComments, "disclosures" | "threads" | "active">;
const icons = new WeakMap<HTMLElement, ReturnType<typeof mount>>();

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
    readonly expanded: boolean,
  ) {
    super();
  }
  eq(other: MiddleFoldWidget) {
    return (
      this.block.id === other.block.id &&
      this.block.revision === other.block.revision &&
      this.comments === other.comments &&
      this.expanded === other.expanded
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
    button.textContent = this.expanded
      ? "Show less"
      : `Show ${this.range.lines} more lines`;
    icons.set(
      row,
      mount(this.expanded ? ChevronDownIcon : ChevronRightIcon, {
        target: button,
        props: { size: 16, "aria-hidden": true },
      }),
    );
    button.setAttribute("aria-expanded", String(this.expanded));
    button.setAttribute(
      "aria-label",
      this.expanded
        ? `Show less in ${this.block.label}`
        : `Show ${this.range.lines} more lines in ${this.block.label}`,
    );
    button.addEventListener("click", () => {
      this.comments.disclosures[middleFoldKey(this.block)] = !this.expanded;
      view.dispatch({
        selection: {
          anchor: this.expanded ? this.range.from - 1 : this.range.from,
        },
      });
      view.focus();
    });
    row.append(button, document.createElement("span"));
    return row;
  }
  ignoreEvent() {
    return true;
  }
  destroy(dom: HTMLElement) {
    const icon = icons.get(dom);
    if (icon) void unmount(icon);
    icons.delete(dom);
  }
}
export function middleDecorations(block: ChatBodyBlock, comments: FoldState) {
  const range = middleRange(block);
  if (!range) return Decoration.none;
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
  const expanded = !!comments.disclosures[middleFoldKey(block)];
  const widget = new MiddleFoldWidget(block, comments, range, expanded);
  return Decoration.set([
    expanded
      ? Decoration.widget({ block: true, side: -1, widget }).range(range.from)
      : Decoration.replace({ block: true, widget }).range(range.from, range.to),
  ]);
}
