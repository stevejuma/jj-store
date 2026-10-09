import { tick } from "svelte";
import type { ChatBodyComments } from "./comments.svelte.js";
import type {
  ChatComment,
  ChatPromptSubmission,
  ChatSubmissionResult,
} from "./model.js";
export async function send(state: ChatBodyComments) {
  if (!state.canSend) return false;
  const submission: ChatPromptSubmission = {
    id: crypto.randomUUID().replaceAll("-", ""),
    text: state.message,
    comments: state.pending.map(({ thread, comment }) => ({
      threadId: thread.id,
      commentId: comment.id,
      anchor: { ...thread.anchor },
      author: { ...comment.author },
      text: comment.text,
    })),
  };
  state.sending = true;
  state.error = "";
  for (const c of submission.comments)
    state.findComment(c.commentId)!.status = "sending";
  try {
    const outcome = await state.props().onSend!(submission);
    state.submissionResult = outcome ?? { publication: "published" };
    if (outcome?.publication === "unknown" || outcome?.recoveryRequired) {
      state.retainedSubmission = submission;
      if (outcome.publication === "published")
        state.markPublished(submission, outcome.batchId);
      state.error =
        outcome.error ??
        "Publication is awaiting confirmation. Check the retained request.";
      return false;
    }
    if (outcome?.publication === "refused")
      throw new Error(
        outcome.error ??
          "The submission was refused. Your drafts are retained.",
      );
    for (const c of submission.comments) {
      const comment = state.findComment(c.commentId)!;
      if (comment.status !== "replied") comment.status = "sent";
      comment.batchId = outcome?.batchId ?? submission.id;
    }
    if (state.message === submission.text)
      state.message = state.props().restoreMessage?.(submission.text) ?? "";
    state.error = outcome?.error ?? "";
    state.active = undefined;
    state.sending = false;
    await tick();
    state.focusMessage?.();
    return true;
  } catch (cause) {
    for (const c of submission.comments)
      state.findComment(c.commentId)!.status = "draft";
    state.error =
      cause instanceof Error
        ? cause.message
        : "The message could not be sent. Try again.";
    return false;
  } finally {
    state.sending = false;
  }
}
export function markPublished(
  state: ChatBodyComments,
  submission: ChatPromptSubmission,
  batchId?: string,
) {
  for (const entry of submission.comments) {
    const comment = state.findComment(entry.commentId);
    if (comment) {
      if (comment.status !== "replied") comment.status = "sent";
      comment.batchId = batchId ?? submission.id;
    }
  }
}
export function confirmPublication(
  state: ChatBodyComments,
  id: string,
  clearMessage = false,
  batchId?: string,
) {
  const submitted = state.retainedSubmission;
  if (!submitted || submitted.id !== id) return;
  state.markPublished(submitted, batchId);
  if (clearMessage && state.message === submitted.text)
    state.message = state.props().restoreMessage?.(submitted.text) ?? "";
  state.retainedSubmission = undefined;
  state.error = "";
}
export function recover(
  state: ChatBodyComments,
  id: string,
  result: ChatSubmissionResult,
) {
  if (state.retainedSubmission?.id !== id) return;
  state.submissionResult = result;
  if (result.publication === "refused") {
    state.refusePublication(id, result.error ?? "Publication was refused.");
    return;
  }
  if (result.publication === "published") {
    state.markPublished(state.retainedSubmission, result.batchId);
    if (!result.recoveryRequired) {
      state.confirmPublication(id, true, result.batchId);
      return;
    }
  }
  state.error =
    result.error ?? "Check the retained request before sending again.";
}
export function keepUnresolved(state: ChatBodyComments) {
  const submitted = state.retainedSubmission;
  if (!submitted) return;
  state.unresolvedHistory.push({
    submission: submitted,
    result: state.submissionResult,
  });
  if (state.message === submitted.text)
    state.message = state.props().restoreMessage?.(submitted.text) ?? "";
  state.retainedSubmission = undefined;
  state.active = undefined;
  state.error = "";
}
export function refusePublication(
  state: ChatBodyComments,
  id: string,
  error: string,
) {
  if (state.retainedSubmission?.id !== id) return;
  for (const entry of state.retainedSubmission.comments) {
    const comment = state.findComment(entry.commentId);
    if (comment?.status === "sending") comment.status = "draft";
  }
  state.retainedSubmission = undefined;
  state.error = error;
}

export function recordReply(
  state: ChatBodyComments,
  threadId: string,
  commentIds: string | readonly string[],
  reply: ChatComment,
  itemId: string,
) {
  const thread = state.threads.find((t) => t.id === threadId);
  const ids = typeof commentIds === "string" ? [commentIds] : commentIds;
  const comments = thread?.comments.filter(
    (c) => ids.includes(c.id) && c.status === "sent",
  );
  if (
    !thread ||
    !comments?.length ||
    thread.comments.some((c) => c.id === reply.id)
  )
    return;
  for (const comment of comments) {
    comment.status = "replied";
    comment.replyItemId = itemId;
  }
  thread.comments.push({ ...reply, replyTo: comments[0].id });
}
