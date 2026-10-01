<script lang="ts">
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import {
    principalMember,
    reviewMessage,
  } from "../../collaboration/presentation.js";
  import { reviewDraft } from "./review-drafts.svelte.js";
  import "../editor-focus.css";
  let {
    controller,
    inline,
    path,
  }: {
    controller: StoreController;
    inline: boolean;
    path: string | undefined;
  } = $props();
  const view = $derived(controller.state),
    detail = $derived(view.reviewThread);
  const draft = $derived(
    reviewDraft(controller, detail?.thread.thread_id ?? ""),
  );
  const disabled = $derived(
    (view.repository?.permission !== "write" &&
      view.repository?.permission !== "admin") ||
      view.status.reviewAction === "refreshing" ||
      view.status.reviewAction === "loading" ||
      !!view.reviewActionRequestId,
  );
  const members = $derived([
    ...new Map(
      (detail?.comments ?? []).map((c) => [
        c.author,
        principalMember(c.author),
      ]),
    ).values(),
  ]);
  $effect(() => {
    if (
      draft.pendingRequestId &&
      view.committedReviewRequestId === draft.pendingRequestId
    ) {
      draft.body = "";
      draft.reply = null;
      draft.edit = null;
      draft.pendingRequestId = null;
    }
  });
  const context = $derived(
    draft.edit
      ? {
          kind: "edit" as const,
          id: draft.edit.comment_id,
          label:
            draft.edit.content.state === "visible"
              ? draft.edit.content.text
              : "",
        }
      : draft.reply
        ? {
            kind: "reply" as const,
            target: {
              eventId: draft.reply.comment_id,
              roomId: detail?.thread.thread_id ?? "",
              authorId: draft.reply.author,
              authorName: principalMember(draft.reply.author).name,
              kind: "channel" as const,
              content: reviewMessage(draft.reply).content,
            },
          }
        : undefined,
  );

  function cancelContext() {
    if (draft.edit) draft.body = "";
    draft.edit = null;
    draft.reply = null;
  }
  async function submit(content: string) {
    if (disabled || !detail) return;
    const currentDraft = draft;
    const threadId = detail.thread.thread_id;
    if (currentDraft.edit) {
      const selected = detail.comments.find(
        (comment) => comment.comment_id === currentDraft.edit!.comment_id,
      );
      if (
        !selected ||
        selected.content.state !== "visible" ||
        selected.author !== view.repository?.principal
      )
        throw new Error("The selected comment can no longer be edited.");
      await controller.actions.editReviewComment(
        currentDraft.edit.comment_id,
        currentDraft.edit.revision_id,
        content,
      );
    } else
      await controller.actions.replyToReview(
        content,
        currentDraft.reply?.comment_id,
      );
    if (
      controller.state.status.reviewAction === "ready" &&
      !controller.state.reviewActionRequestId &&
      controller.state.reviewThread?.thread.thread_id === threadId
    ) {
      currentDraft.body = "";
      currentDraft.reply = null;
      currentDraft.edit = null;
    } else {
      currentDraft.pendingRequestId = controller.state.reviewActionRequestId;
      throw new Error(
        controller.state.reviewActionMessage ||
          "Review update has not been confirmed.",
      );
    }
  }
</script>

<div class="store-collaboration__composer">
  <CommunityChatComposer
    mode="form"
    showSubmitButton
    bind:value={draft.body}
    {members}
    {disabled}
    {context}
    onCancelContext={cancelContext}
    busy={view.status.reviewAction === "refreshing"}
    ariaLabel={inline
      ? `Add a comment to ${path ?? "the selected diff"}`
      : "Reply to review thread"}
    submitLabel={draft.edit ? "Save comment" : inline ? "Comment" : "Reply"}
    placeholder={inline ? "Add a review comment" : "Reply with Markdown"}
    onSubmit={(submission) => submit(submission.content)}
  />
</div>
