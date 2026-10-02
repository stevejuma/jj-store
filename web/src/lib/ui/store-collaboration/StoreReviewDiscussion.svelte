<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import MessageSquareIcon from "@lucide/svelte/icons/message-square";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import type { StoreController } from "../../controller/contracts.js";
  import {
    anchorAgentMember,
    anchorLabel,
  } from "../../collaboration/presentation.js";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import CommunityProjectsColumnAlert from "../community-projects/CommunityProjectsColumnAlert.svelte";
  import StoreIdentityCopy from "./StoreIdentityCopy.svelte";
  import StoreReviewAnchorIdentity from "./StoreReviewAnchorIdentity.svelte";
  import StoreReviewConversation from "./StoreReviewConversation.svelte";

  let { controller }: { controller: StoreController } = $props();
  const view = $derived(controller.state);
  const detail = $derived(view.reviewThread);
  const agent = $derived(
    detail
      ? anchorAgentMember(detail.anchor.value, view.agentSessions)
      : undefined,
  );
</script>

{#if detail}
  <div class="store-review-discussion">
    <header class="store-collaboration__detail-header">
      <div class="store-collaboration__detail-title-row">
        <span class="store-collaboration__detail-identity">
          {#if agent}<CommunityMemberAvatar
              member={agent}
              size="small"
              decorative
            />{/if}
          <span class="store-collaboration__detail-title"
            >{anchorLabel(detail.anchor.value)}</span
          >
        </span>
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
        {#if detail.snapshot.review_root}<StoreIdentityCopy
            value={detail.snapshot.review_root}
            label="review root"
          />{/if}
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
    <StoreReviewConversation {controller} />
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
