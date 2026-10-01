<script lang="ts">
  import { FileDiff } from "@lapismd/design-core/diff";
  import StoreMessageReactions from "../store-threads/StoreMessageReactions.svelte";
  import StoreReviewComposer from "./StoreReviewComposer.svelte";
  import CommunityMessage from "../community-message/CommunityMessage.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import {
    anchorPath,
    principalMember,
    reviewCommentRows,
    reviewMessage,
  } from "../../collaboration/presentation.js";
  import CommunityProjectsVirtualRows from "../community-projects/CommunityProjectsVirtualRows.svelte";
  import StoreMessageActions from "./StoreMessageActions.svelte";
  import { reviewDraft } from "./review-drafts.svelte.js";

  let {
    controller,
    inline = false,
    showComposer = true,
    showDiff = !inline,
  }: {
    controller: StoreController;
    inline?: boolean;
    showComposer?: boolean;
    showDiff?: boolean;
  } = $props();
  const view = $derived(controller.state);
  const detail = $derived(view.reviewThread);
  const commentRows = $derived(reviewCommentRows(detail?.comments ?? []));
  const members = $derived(
    detail
      ? [
          ...new Map(
            detail.comments.map((comment) => [
              comment.author,
              principalMember(comment.author),
            ]),
          ).values(),
        ]
      : [],
  );
  const path = $derived(detail ? anchorPath(detail.anchor.value) : undefined);
  const textSelection = $derived(
    detail?.anchor.value.target.target === "code_text"
      ? detail.anchor.value.target.value.text
      : undefined,
  );
  const patch = $derived(
    path
      ? view.reviewPatch?.path === path
        ? view.reviewPatch.patch
        : view.detail?.patches[path]
      : undefined,
  );
  const draft = $derived(
    reviewDraft(controller, detail?.thread.thread_id ?? ""),
  );
  const writable = $derived(
    view.repository?.permission === "write" ||
      view.repository?.permission === "admin",
  );
  const disabled = $derived(
    !writable ||
      view.status.reviewAction === "refreshing" ||
      view.status.reviewAction === "loading" ||
      !!view.reviewActionRequestId,
  );
</script>

{#if detail}<div
    class="store-review-conversation"
    data-inline={inline ? "true" : undefined}
  >
    <div
      class="store-collaboration__messages"
      role="list"
      aria-label="Review comments"
    >
      <CommunityProjectsVirtualRows
        rows={commentRows}
        getKey={(row) => row.comment.comment_id}
        estimateSize={() => 118}
      >
        {#snippet row(row)}
          {@const comment = row.comment}
          {@const author = principalMember(comment.author)}
          <div
            class="store-collaboration__message"
            data-thread-depth={row.depth}
            style={`--store-review-indent: ${row.indentRem}rem; --store-review-connector-indent: ${row.connectorIndentRem ?? 0}rem;`}
            role="listitem"
          >
            {#each row.guideIndentRems as guideIndent}
              <span
                class="store-collaboration__message-guide"
                data-ui-part="review-reply-guide"
                style={`--store-review-guide-indent: ${guideIndent}rem;`}
                aria-hidden="true"
              ></span>
            {/each}
            {#if row.connectorIndentRem !== undefined}<span
                class="store-collaboration__message-connector"
                data-ui-part="review-reply-connector"
                aria-hidden="true"
              ></span>{/if}
            {#if row.connectsToChildren}<span
                class="store-collaboration__message-descendant-guide"
                data-ui-part="review-descendant-guide"
                aria-hidden="true"
              ></span>{/if}
            <div class="store-collaboration__message-content">
              <CommunityMessage
                message={reviewMessage(comment)}
                {author}
                {members}
              >
                {#snippet supplement()}{#if controller.threads && detail && comment.content.state === "visible"}<StoreMessageReactions
                      threads={controller.threads}
                      target={{
                        type: "review_comment",
                        thread_id: detail.thread.thread_id,
                        comment_id: comment.comment_id,
                        revision: {
                          kind: "comment_revision",
                          id: comment.revision_id,
                        },
                      }}
                      {disabled}
                    />{/if}<StoreMessageActions
                    {disabled}
                    authorName={author.name}
                    items={[
                      {
                        label: "Copy message",
                        value:
                          comment.content.state === "visible"
                            ? comment.content.text
                            : null,
                      },
                      { label: "Copy comment ID", value: comment.comment_id },
                      { label: "Copy revision ID", value: comment.revision_id },
                      { label: "Copy author ID", value: comment.author },
                      {
                        label: "Copy thread ID",
                        value: detail.thread.thread_id,
                      },
                      {
                        label: "Copy parent comment ID",
                        value: comment.parent_comment_id,
                      },
                    ]}
                    onReply={writable && detail.thread.status === "open"
                      ? () => {
                          draft.reply = comment;
                          draft.edit = null;
                        }
                      : undefined}
                    onEdit={writable &&
                    comment.content.state === "visible" &&
                    comment.author === view.repository?.principal
                      ? () => {
                          draft.edit = comment;
                          draft.reply = null;
                          draft.body =
                            comment.content.state === "visible"
                              ? comment.content.text
                              : "";
                        }
                      : undefined}
                  />{/snippet}
              </CommunityMessage>
            </div>
          </div>
        {/snippet}
      </CommunityProjectsVirtualRows>
    </div>
    {#if showDiff && path}<div
        class="store-review-conversation__diff"
        role="group"
        aria-label={`Reviewed file ${path}`}
      >
        <header>
          <span>{path}</span>{#if textSelection}<span
              class="store-review-conversation__selection"
              >Bytes {textSelection.start_byte}–{textSelection.end_byte}</span
            >{/if}
        </header>
        {#if patch}<FileDiff {path} {patch} viewMode="unified" wrap />{:else}<p>
            The exact text diff is unavailable for this anchored file.
          </p>{/if}
      </div>{/if}
    {#if writable && ((showComposer && detail.thread.status === "open") || draft.reply || draft.edit)}
      <StoreReviewComposer {controller} {inline} {path} />
    {/if}
  </div>{/if}

<style>
  .store-review-conversation {
    display: flex;
    min-width: 0;
    min-height: 0;
    flex: 1 1 auto;
    flex-direction: column;
  }

  .store-review-conversation__diff {
    min-width: 0;
    margin: 0.5rem 0.75rem 0.75rem;
    overflow: hidden;
    border: 1px solid var(--border);
    border-radius: var(--radius);
  }

  .store-review-conversation__diff > header {
    display: flex;
    min-width: 0;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
    padding: 0.4rem 0.6rem;
    border-bottom: 1px solid var(--border);
    background: var(--jj-store-row-background);
    font-family: var(--font-mono, var(--community-font-mono));
    font-size: var(--community-text-secondary);
  }

  .store-review-conversation__selection {
    flex: 0 0 auto;
    color: var(--muted-foreground);
    font-family: var(--font-sans, var(--community-font-sans));
  }

  .store-review-conversation__diff > p {
    margin: 0;
    padding: 0.75rem;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
  }

  .store-review-conversation[data-inline="true"] {
    padding-block: 0.35rem;
  }
</style>
