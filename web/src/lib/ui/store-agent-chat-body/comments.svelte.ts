import * as submission from "./comment-submission.js";
import { tick } from "svelte";
import type {
  ChatBodyBlock,
  ChatCommentAnchor,
  ChatBodyProps,
  ChatCommentThread,
  ChatComment,
  ChatPromptSubmission,
  ChatSubmissionResult,
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
  retainedSubmission = $state<ChatPromptSubmission>();
  submissionResult = $state<ChatSubmissionResult>();
  unresolvedHistory = $state<
    { submission: ChatPromptSubmission; result?: ChatSubmissionResult }[]
  >([]);
  disclosures = $state<Record<string, boolean>>({});
  rawTools = $state<Record<string, boolean>>({});
  bookmarkedTurns = $state<Record<string, boolean>>({});
  highlightedThread = $state<string>();
  root: HTMLElement | null = null;
  focusMessage?: () => void;
  selections = new Map<string, { anchor: number; head: number }>();
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
            (c.status === "draft" || c.status === "sending") &&
            !!c.text.trim() &&
            !this.unresolvedHistory.some((h) =>
              h.submission.comments.some((e) => e.commentId === c.id),
            ),
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
      !this.retainedSubmission &&
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
    return [
      ...commentSources(this.props().items).map((s) => s.block),
      ...(this.props().externalBlocks ?? []),
    ].some((block) => validAnchor(block, thread.anchor));
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
    if (
      this.sending ||
      this.props().canComment === false ||
      block.commentable === false
    )
      return;
    const anchor = commentAnchor(block, selection);
    let thread = this.threads.find((t) =>
      threadId ? t.id === threadId : anchorKey(t.anchor) === anchorKey(anchor),
    );
    if (!thread) {
      thread = {
        id: crypto.randomUUID().replaceAll("-", ""),
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
    if (this.sending || this.props().canComment === false) return;
    let comment = newEntry
      ? undefined
      : thread.comments
          .filter((c) => c.author.id === this.author.id && c.status === "draft")
          .at(-1);
    if (!comment) {
      thread.comments.push({
        id: crypto.randomUUID().replaceAll("-", ""),
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
  editSent(thread: ChatCommentThread, original: ChatComment) {
    if (
      this.sending ||
      original.historical ||
      original.author.id !== this.author.id ||
      !original.revision
    )
      return;
    const pending = thread.comments.find(
      (c) =>
        c.status === "draft" &&
        c.messageId === (original.messageId ?? original.id),
    );
    if (pending) {
      this.activate(thread, pending, "inline");
      return;
    }
    const draft: ChatComment = {
      id: crypto.randomUUID().replaceAll("-", ""),
      messageId: original.messageId ?? original.id,
      revision: original.revision,
      author: original.author,
      text: original.text,
      status: "draft",
    };
    thread.comments.push(draft);
    this.activate(thread, thread.comments.at(-1)!, "inline");
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
  send() {
    return submission.send(this);
  }
  markPublished(value: ChatPromptSubmission, batchId?: string) {
    submission.markPublished(this, value, batchId);
  }
  confirmPublication(id: string, clearMessage = false, batchId?: string) {
    submission.confirmPublication(this, id, clearMessage, batchId);
  }
  recover(id: string, result: ChatSubmissionResult) {
    submission.recover(this, id, result);
  }
  keepUnresolved() {
    submission.keepUnresolved(this);
  }
  refusePublication(id: string, error: string) {
    submission.refusePublication(this, id, error);
  }
  recordReply(
    threadId: string,
    commentIds: string | readonly string[],
    reply: ChatComment,
    itemId: string,
  ) {
    submission.recordReply(this, threadId, commentIds, reply, itemId);
  }
  async goToSource(thread: ChatCommentThread, opener: HTMLElement) {
    try {
      return await revealCommentSource(this, thread, opener);
    } catch (cause) {
      this.error =
        cause instanceof Error ? cause.message : "The source is unavailable.";
    }
  }
  async goToMessage(thread: ChatCommentThread, comment: ChatComment) {
    try {
      await revealCommentMessage(this, thread, comment);
    } catch (cause) {
      this.error =
        cause instanceof Error
          ? cause.message
          : "The submitted message is unavailable.";
    }
  }
  async goToReply(itemId: string) {
    try {
      await this.props().onNavigateReply?.(itemId);
    } catch (cause) {
      this.error =
        cause instanceof Error ? cause.message : "The reply is unavailable.";
      return;
    }
    await tick();
    const item = Array.from(
      this.root?.querySelectorAll<HTMLElement>("[data-chat-item]") ?? [],
    ).find((node) => node.dataset.chatItem === itemId);
    item?.scrollIntoView({ block: "center" });
    item?.focus({ preventScroll: true });
    return !!item;
  }
}
