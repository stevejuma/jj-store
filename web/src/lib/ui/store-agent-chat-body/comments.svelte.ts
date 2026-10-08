import { tick } from "svelte";
import type {
  ChatBodyBlock,
  ChatCommentAnchor,
  ChatBodyProps,
  ChatCommentThread,
  ChatComment,
  ChatPromptSubmission,
} from "./model.js";
import { anchorKey, commentAnchor, validAnchor } from "./anchors.js";
import {
  commentSources,
  revealCommentSource,
  revealCommentMessage,
} from "./comment-navigation.js";

const localAuthor = {
  id: "local-user",
  name: "You",
  handle: "",
  initials: "YO",
};
export class ChatBodyComments {
  active = $state<{
    anchor: ChatCommentAnchor;
    label: string;
    threadId: string;
    commentId: string;
    location: "inline" | "queue";
  }>();
  threads = $state<ChatCommentThread[]>([]);
  message = $state("");
  sending = $state(false);
  error = $state("");
  disclosures = $state<Record<string, boolean>>({});
  rawTools = $state<Record<string, boolean>>({});
  bookmarkedTurns = $state<Record<string, boolean>>({});
  highlightedThread = $state<string>();
  root: HTMLElement | null = null;
  focusMessage?: () => void;
  readers = new Map<
    string,
    (anchor: ChatCommentAnchor, scroll?: boolean) => void
  >();
  constructor(
    readonly props: () => ChatBodyProps,
    initial: ChatCommentThread[] = [],
  ) {
    this.threads = structuredClone($state.snapshot(initial));
  }
  get author() {
    return this.props().author ?? localAuthor;
  }
  get pending() {
    return this.threads.flatMap((thread) =>
      thread.comments
        .filter(
          (c) =>
            (c.status === "draft" || c.status === "sending") && !!c.text.trim(),
        )
        .map((comment) => ({ thread, comment })),
    );
  }
  get visibleThreads() {
    return this.threads.filter(
      (t) =>
        t.comments.some((c) => c.text.trim()) || this.active?.threadId === t.id,
    );
  }
  get canSend() {
    return (
      !!this.props().onSend &&
      !this.sending &&
      (!!this.message.trim() || this.pending.length > 0)
    );
  }
  get key() {
    return this.active ? anchorKey(this.active.anchor) : "";
  }
  get draft() {
    return this.findComment(this.active?.commentId)?.text ?? "";
  }
  set draft(text: string) {
    if (this.active) this.edit(this.active.commentId, text);
  }
  findComment(id?: string) {
    return this.threads.flatMap((t) => t.comments).find((c) => c.id === id);
  }
  isAvailable(thread: ChatCommentThread) {
    return commentSources(this.props().items).some((s) =>
      validAnchor(s.block, thread.anchor),
    );
  }
  forBlocks(blocks: readonly ChatBodyBlock[]) {
    return this.visibleThreads.filter((t) =>
      blocks.some((b) => validAnchor(b, t.anchor)),
    );
  }
  open(
    block: ChatBodyBlock,
    selection: { from: number; to: number },
    initial = "",
    threadId?: string,
  ) {
    if (this.sending) return;
    const anchor = commentAnchor(block, selection);
    let thread = this.threads.find((t) =>
      threadId ? t.id === threadId : anchorKey(t.anchor) === anchorKey(anchor),
    );
    if (!thread) {
      thread = {
        id: crypto.randomUUID(),
        anchor,
        label: block.label,
        sourceKind: block.kind,
        comments: [],
      };
      this.threads.push(thread);
      thread = this.threads.at(-1)!;
    }
    this.begin(thread, "inline", initial);
  }
  begin(
    thread: ChatCommentThread,
    location: "inline" | "queue",
    initial = "",
    newEntry = false,
  ) {
    if (this.sending) return;
    let comment = newEntry
      ? undefined
      : thread.comments
          .filter((c) => c.author.id === this.author.id && c.status === "draft")
          .at(-1);
    if (!comment) {
      thread.comments.push({
        id: crypto.randomUUID(),
        author: this.author,
        text: "",
        status: "draft",
        createdAt: Date.now(),
      });
      comment = thread.comments.at(-1)!;
    }
    if (initial) comment.text += initial;
    this.active = {
      anchor: thread.anchor,
      label: thread.label,
      threadId: thread.id,
      commentId: comment.id,
      location,
    };
  }
  activate(
    thread: ChatCommentThread,
    comment: ChatComment,
    location: "inline" | "queue",
  ) {
    if (
      this.active?.commentId === comment.id &&
      this.active.location === location
    )
      return;
    this.active = {
      anchor: thread.anchor,
      label: thread.label,
      threadId: thread.id,
      commentId: comment.id,
      location,
    };
  }
  edit(id: string, text: string) {
    const comment = this.findComment(id);
    if (comment?.status === "draft") comment.text = text;
  }
  remove(id: string) {
    const comment = this.findComment(id);
    if (comment?.status !== "draft") return;
    if (this.active?.commentId === id) this.close();
    this.threads = this.threads
      .map((t) => ({ ...t, comments: t.comments.filter((c) => c.id !== id) }))
      .filter((t) => t.comments.length);
  }
  close(discard = false) {
    const active = this.active;
    this.active = undefined;
    if (active && (discard || !this.findComment(active.commentId)?.text.trim()))
      this.remove(active.commentId);
    if (active)
      void tick().then(() => {
        if (!this.active)
          this.readers.get(active.anchor.blockId)?.(active.anchor);
      });
  }
  async send() {
    if (!this.canSend) return false;
    const submission: ChatPromptSubmission = {
      id: crypto.randomUUID(),
      text: this.message,
      comments: this.pending.map(({ thread, comment }) => ({
        threadId: thread.id,
        commentId: comment.id,
        anchor: { ...thread.anchor },
        author: { ...comment.author },
        text: comment.text,
      })),
    };
    this.sending = true;
    this.error = "";
    for (const c of submission.comments)
      this.findComment(c.commentId)!.status = "sending";
    try {
      await this.props().onSend!(submission);
      for (const c of submission.comments) {
        const comment = this.findComment(c.commentId)!;
        comment.status = "sent";
        comment.batchId = submission.id;
      }
      if (this.message === submission.text) this.message = "";
      this.active = undefined;
      this.sending = false;
      await tick();
      this.focusMessage?.();
      return true;
    } catch (cause) {
      for (const c of submission.comments)
        this.findComment(c.commentId)!.status = "draft";
      this.error =
        cause instanceof Error
          ? cause.message
          : "The message could not be sent. Try again.";
      return false;
    } finally {
      this.sending = false;
    }
  }
  recordReply(
    threadId: string,
    commentId: string,
    reply: ChatComment,
    itemId: string,
  ) {
    const thread = this.threads.find((t) => t.id === threadId);
    const comment = thread?.comments.find((c) => c.id === commentId);
    if (
      !thread ||
      !comment ||
      comment.status !== "sent" ||
      thread.comments.some((c) => c.id === reply.id)
    )
      return;
    comment.status = "replied";
    comment.replyItemId = itemId;
    thread.comments.push({ ...reply, replyTo: commentId });
  }
  async goToSource(thread: ChatCommentThread, opener: HTMLElement) {
    await revealCommentSource(this, thread, opener);
  }
  async goToMessage(thread: ChatCommentThread, comment: ChatComment) {
    await revealCommentMessage(this, thread, comment);
  }
  goToReply(itemId: string) {
    const item = Array.from(
      this.root?.querySelectorAll<HTMLElement>("[data-chat-item]") ?? [],
    ).find((node) => node.dataset.chatItem === itemId);
    item?.scrollIntoView({ block: "center" });
    item?.focus({ preventScroll: true });
  }
}
