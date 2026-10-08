import { mount, unmount, tick } from "svelte";
import {
  Decoration,
  WidgetType,
  type DecorationSet,
  type EditorView,
} from "@codemirror/view";
import { estimateMarkdownBlockHeight } from "@lapismd/mira/codemirror";
import type { ChatBodyBlock } from "./model.js";
import type { ChatBodyComments } from "./comments.svelte.js";
import ChatCommentArea from "./ChatCommentArea.svelte";
import { validAnchor } from "./anchors.js";

const mounts = new WeakMap<HTMLElement, ReturnType<typeof mount>>();
const observers = new WeakMap<HTMLElement, ResizeObserver>();
class CommentWidget extends WidgetType {
  constructor(
    readonly state: ChatBodyComments,
    readonly block: ChatBodyBlock,
    readonly contexts: Map<unknown, unknown>,
    readonly threadId: string,
  ) {
    super();
  }
  eq(other: CommentWidget) {
    return (
      this.state === other.state &&
      this.block.id === other.block.id &&
      this.block.revision === other.block.revision &&
      this.threadId === other.threadId
    );
  }
  get estimatedHeight() {
    const thread = this.state.threads.find((t) => t.id === this.threadId);
    return (
      32 +
      (thread?.comments
        .filter((comment) => !comment.replyTo)
        .reduce(
          (height, c) => height + estimateMarkdownBlockHeight(c.text) + 32,
          0,
        ) ?? 48)
    );
  }
  toDOM(view: EditorView) {
    const target = document.createElement("div");
    target.contentEditable = "false";
    target.className = "chat-comment-widget";
    const instance = mount(ChatCommentArea, {
      target,
      context: this.contexts,
      props: { state: this.state, threadId: this.threadId },
    });
    mounts.set(target, instance);
    const observer = new ResizeObserver(() => view.requestMeasure());
    observer.observe(target);
    observers.set(target, observer);
    void tick().then(() => {
      if (target.isConnected) view.requestMeasure();
    });
    return target;
  }
  ignoreEvent() {
    return true;
  }
  destroy(dom: HTMLElement) {
    observers.get(dom)?.disconnect();
    observers.delete(dom);
    const instance = mounts.get(dom);
    if (instance) void unmount(instance);
    mounts.delete(dom);
  }
}

export function commentDecorations(
  block: ChatBodyBlock,
  state: ChatBodyComments,
  contexts: Map<unknown, unknown>,
): DecorationSet {
  const anchors = state.visibleThreads
    .filter((t) => validAnchor(block, t.anchor))
    .map((t) => ({ anchor: t.anchor, threadId: t.id }));
  const ranges = anchors.map(({ anchor, threadId }) => {
    const newline = block.text.indexOf("\n", anchor.to);
    const position = newline < 0 ? block.text.length : newline;
    return Decoration.widget({
      block: true,
      side: 1,
      widget: new CommentWidget(state, block, contexts, threadId),
    }).range(position);
  });
  for (const { anchor, threadId } of anchors) {
    if (anchor.to > anchor.from)
      ranges.push(
        Decoration.mark({
          class: `chat-comment-highlight${state.active?.threadId === threadId || state.highlightedThread === threadId ? " chat-comment-highlight-active" : ""}`,
        }).range(anchor.from, anchor.to),
      );
  }
  return Decoration.set(ranges, true);
}
