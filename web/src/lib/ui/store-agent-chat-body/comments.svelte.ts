import type {
  ChatBodyBlock,
  ChatBodyProps,
  ChatCommentAnchor,
} from "./model.js";
import { anchorKey, commentAnchor } from "./anchors.js";

export class ChatBodyComments {
  active = $state<{
    anchor: ChatCommentAnchor;
    label: string;
    threadId?: string;
  }>();
  drafts = $state<Record<string, string>>({});
  disclosures = $state<Record<string, boolean>>({});
  rawTools = $state<Record<string, boolean>>({});
  bookmarkedTurns = $state<Record<string, boolean>>({});
  readers = new Map<string, (anchor: ChatCommentAnchor) => void>();
  constructor(readonly props: () => ChatBodyProps) {}
  get threads() {
    return this.props().threads ?? [];
  }
  get key() {
    return this.active ? anchorKey(this.active.anchor) : "";
  }
  get draft() {
    return this.drafts[this.key] ?? "";
  }
  set draft(value: string) {
    this.drafts = { ...this.drafts, [this.key]: value };
  }
  open(
    block: ChatBodyBlock,
    selection: { from: number; to: number },
    initial = "",
    threadId?: string,
  ) {
    const anchor = commentAnchor(block, selection);
    const existing = this.threads.find(
      (t) => anchorKey(t.anchor) === anchorKey(anchor),
    );
    this.active = {
      anchor,
      label: block.label,
      threadId: threadId ?? existing?.id,
    };
    if (initial) this.draft += initial;
  }
  reveal(anchor: ChatCommentAnchor, label: string, threadId: string) {
    this.active = { anchor, label, threadId };
  }
  close(discard = false) {
    const anchor = this.active?.anchor;
    if (discard) this.draft = "";
    this.active = undefined;
    if (anchor)
      requestAnimationFrame(() => {
        if (!this.active) this.readers.get(anchor.blockId)?.(anchor);
      });
  }
  submit() {
    if (!this.active || !this.draft.trim()) return;
    const { anchor, threadId } = this.active;
    if (threadId) this.props().onReply(threadId, this.draft.trim());
    else this.props().onAddComment(anchor, this.draft.trim());
    this.close(true);
  }
}
