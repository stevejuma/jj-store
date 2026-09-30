<script lang="ts">
  import { FileDiff } from "@lapismd/design-core/diff";
  import { CommunityChatComposer } from "@lapismd/lapis-community/components/community-chat-composer";
  import "../editor-focus.css";
  import { CommunityMessage } from "@lapismd/lapis-community/components/community-message";
  import type { StoreController } from "../../controller/contracts.js";
  import {
    anchorPath,
    principalMember,
    reviewCommentRows,
    reviewMessage,
  } from "../../collaboration/presentation.js";
  import CommunityProjectsVirtualRows from "../community-projects/CommunityProjectsVirtualRows.svelte";
  import StoreIdentityCopy from "./StoreIdentityCopy.svelte";

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
  let draft = $state("");

  async function submit(content: string) {
    await controller.actions.replyToReview(content);
    if (controller.state.status.reviewAction === "ready") draft = "";
  }
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
              <div class="store-collaboration__message-identities">
                <StoreIdentityCopy
                  value={comment.author}
                  label="review author ID"
                />
                <StoreIdentityCopy
                  value={comment.comment_id}
                  label="review comment ID"
                />
              </div>
              <CommunityMessage
                message={reviewMessage(comment)}
                {author}
                {members}
                density="compact"
              />
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
    {#if showComposer && detail.thread.status === "open"}
      <div class="store-collaboration__composer">
        <CommunityChatComposer
          mode="form"
          showSubmitButton
          bind:value={draft}
          {members}
          disabled={view.status.reviewAction === "refreshing"}
          busy={view.status.reviewAction === "refreshing"}
          ariaLabel={inline
            ? `Add a comment to ${path ?? "the selected diff"}`
            : "Reply to review thread"}
          submitLabel={inline ? "Comment" : "Reply"}
          placeholder={inline ? "Add a review comment" : "Reply with Markdown"}
          onSubmit={(submission) => submit(submission.content)}
        />
      </div>
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
