<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import MessageSquareIcon from "@lucide/svelte/icons/message-square";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import { CommunityChatComposer } from "@lapismd/lapis-community/components/community-chat-composer";
  import { CommunityMessage } from "@lapismd/lapis-community/components/community-message";
  import type { StoreController } from "../../controller/contracts.js";
  import {
    anchorLabel,
    principalMember,
    reviewMessage,
  } from "../../collaboration/presentation.js";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import CommunityProjectsColumnAlert from "../community-projects/CommunityProjectsColumnAlert.svelte";
  import CommunityProjectsVirtualRows from "../community-projects/CommunityProjectsVirtualRows.svelte";
  import StoreIdentityCopy from "./StoreIdentityCopy.svelte";
  import StoreReviewAnchorIdentity from "./StoreReviewAnchorIdentity.svelte";

  let { controller }: { controller: StoreController } = $props();
  const view = $derived(controller.state);
  const detail = $derived(view.reviewThread);
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
  let draft = $state("");

  async function submit(content: string) {
    await controller.actions.replyToReview(content);
    if (controller.state.status.reviewAction === "ready") draft = "";
  }
</script>

{#if detail}
  <div class="store-review-discussion">
    <header class="store-collaboration__detail-header">
      <div class="store-collaboration__detail-title-row">
        <span class="store-collaboration__detail-title"
          >{anchorLabel(detail.anchor.value)}</span
        >
        <StoreIdentityCopy
          value={detail.thread.thread_id}
          label="review thread ID"
        />
      </div>
      <div class="store-collaboration__detail-meta">
        <CommunityChip
          tone={detail.thread.status === "open" ? "accent" : "success"}
        >
          {detail.thread.status}
        </CommunityChip>
        <StoreIdentityCopy
          value={detail.snapshot.review_root}
          label="review root"
        />
        <StoreReviewAnchorIdentity anchor={detail.anchor.value} />
      </div>
      <div class="store-collaboration__actions">
        <Button
          variant="outline"
          size="sm"
          disabled={view.status.reviewAction === "refreshing"}
          onclick={() =>
            controller.actions.setReviewResolved(
              detail.thread.status === "open",
            )}
        >
          {detail.thread.status === "open" ? "Resolve" : "Reopen"}
        </Button>
      </div>
    </header>
    {#if view.reviewActionMessage}
      <CommunityProjectsColumnAlert
        title={view.status.reviewAction === "unavailable"
          ? "Review outcome unknown"
          : "Review updated"}
        description={view.reviewActionMessage}
      >
        {#if view.reviewActionRequestId}
          <Button
            variant="outline"
            size="sm"
            onclick={() => controller.actions.checkReviewAction()}
          >
            <RefreshCwIcon data-icon="inline-start" aria-hidden="true" />Check
            status
          </Button>
        {/if}
      </CommunityProjectsColumnAlert>
    {/if}
    <div
      class="store-collaboration__messages"
      role="list"
      aria-label="Review comments"
    >
      <CommunityProjectsVirtualRows
        rows={detail.comments}
        getKey={(comment) => comment.comment_id}
        estimateSize={() => 118}
      >
        {#snippet row(comment)}
          {@const author = principalMember(comment.author)}
          <div
            class="store-collaboration__message"
            data-reply={comment.parent_comment_id ? "" : undefined}
            role="listitem"
          >
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
              highlighted={comment.parent_comment_id !== null}
            />
          </div>
        {/snippet}
      </CommunityProjectsVirtualRows>
    </div>
    {#if detail.thread.status === "open"}
      <div class="store-collaboration__composer">
        <CommunityChatComposer
          mode="form"
          showSubmitButton
          bind:value={draft}
          {members}
          disabled={view.status.reviewAction === "refreshing"}
          busy={view.status.reviewAction === "refreshing"}
          ariaLabel="Reply to review thread"
          submitLabel="Reply"
          placeholder="Reply with Markdown"
          onSubmit={(submission) => submit(submission.content)}
        />
      </div>
    {/if}
  </div>
{:else}
  <Empty.Root class="store-collaboration__empty">
    <Empty.Header>
      <Empty.Media variant="icon"
        ><MessageSquareIcon aria-hidden="true" /></Empty.Media
      >
      <Empty.Title>Select a review thread</Empty.Title>
      <Empty.Description
        >Choose a thread to inspect its exact anchor and discussion.</Empty.Description
      >
    </Empty.Header>
  </Empty.Root>
{/if}
