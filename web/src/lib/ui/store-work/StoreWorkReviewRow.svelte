<script lang="ts">
  // CommunityProjectsPullRequestList.svelte reviewRow (Apache-2.0).
  import {
    CommunityMemberHoverCard,
    CommunityRelativeTime,
  } from "@lapismd/lapis-community/components";
  import CircleCheckIcon from "@lucide/svelte/icons/circle-check";
  import MessageSquareIcon from "@lucide/svelte/icons/message-square";
  import GitBranchIcon from "@lucide/svelte/icons/git-branch";
  import { anchorLabel } from "../../collaboration/presentation.js";
  import type { Model } from "../../api/client.js";
  import { workMember } from "./work-presentation.js";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  let {
    thread,
    onOpen,
    agent,
  }: {
    thread: Model<"ReviewThreadSummary">;
    onOpen?: (id: string) => void | Promise<void>;
    agent?: CommunityMemberModel;
  } = $props();
  const title = $derived(
    thread.presentation?.excerpt || anchorLabel(thread.anchor),
  );
  const target = $derived(thread.anchor.target);
  const artifact = $derived(
    "value" in target && "commit_id" in target.value
      ? `${anchorLabel(thread.anchor)} ${target.value.commit_id.slice(0, 7)}`
      : anchorLabel(thread.anchor),
  );
</script>

<article
  class="community-projects__review-row store-projects__native-review-row"
>
  <span class="community-projects__review-status-icon" title={thread.status}>
    {#if thread.status === "open"}<MessageSquareIcon
        aria-hidden="true"
      />{:else}<CircleCheckIcon aria-hidden="true" />{/if}
    <span class="sr-only">{thread.status}</span>
  </span>
  <button
    type="button"
    class="community-projects__review-row-title"
    {title}
    disabled={!onOpen}
    onclick={() => onOpen?.(thread.thread_id)}
    aria-label={`Open ${anchorLabel(thread.anchor)} ${thread.status} review with ${thread.comment_count} comments`}
    >{title}</button
  >
  <button
    type="button"
    class="community-projects__review-row-id"
    title={thread.thread_id}
    disabled={!onOpen}
    onclick={() => onOpen?.(thread.thread_id)}
    aria-label={`Open review ${thread.thread_id}`}
    >#{thread.thread_id.slice(0, 8)}</button
  >
  <span class="community-projects__review-row-branch" title={artifact}
    ><span aria-hidden="true">›</span>{#if agent}<CommunityMemberAvatar
        member={agent}
        size="small"
        decorative
      />{:else}<GitBranchIcon aria-hidden="true" />{/if}<span>{artifact}</span
    ></span
  >
  <span
    class="community-projects__review-row-comments"
    aria-label={`${thread.comment_count} comments`}
    ><MessageSquareIcon aria-hidden="true" />{thread.comment_count}</span
  >
  {#if thread.presentation}<CommunityMemberHoverCard
      member={workMember(thread.presentation.created_by)}
      avatarSize="xsmall"
    />
    <CommunityRelativeTime
      timestamp={Number(thread.presentation.updated_at_millis) / 1000}
    />
  {:else}<span
      class="store-projects__review-unavailable"
      aria-label="Review author unavailable">—</span
    ><span
      class="store-projects__review-unavailable"
      aria-label="Review activity date unavailable">—</span
    >{/if}
</article>
