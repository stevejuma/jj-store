import { mount, unmount } from "svelte";
import { Decoration, EditorView, WidgetType } from "@codemirror/view";
import CornerUpLeftIcon from "@lucide/svelte/icons/corner-up-left";

const icons = new WeakMap<HTMLElement, ReturnType<typeof mount>>();
class ReplyIndicator extends WidgetType {
  eq(other: WidgetType) {
    return other instanceof ReplyIndicator;
  }
  toDOM() {
    const node = document.createElement("span");
    node.className = "chat-reply-indicator";
    node.setAttribute("aria-hidden", "true");
    icons.set(
      node,
      mount(CornerUpLeftIcon, { target: node, props: { size: 16 } }),
    );
    return node;
  }
  ignoreEvent() {
    return false;
  }
  destroy(node: HTMLElement) {
    const icon = icons.get(node);
    if (icon) void unmount(icon);
    icons.delete(node);
  }
}

/** Display-only prefix: selection, copying and comment offsets keep the source text. */
export const replyIndicator = EditorView.decorations.of(
  Decoration.set([
    Decoration.widget({ widget: new ReplyIndicator(), side: -1 }).range(0),
  ]),
);
