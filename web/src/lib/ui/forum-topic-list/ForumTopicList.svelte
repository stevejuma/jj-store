<script lang="ts">
  import "./forum-topic-list.css";
  import CommunityTopicComposer from "../community-topic-composer/CommunityTopicComposer.svelte";
  import type { CommunityTopicDraft } from "../../community/contracts.js";
  import type { ComposerStatus } from "@lapismd/design-core/ai/chat";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import MessagesSquareIcon from "@lucide/svelte/icons/messages-square";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import CommunityMemberHoverCard from "../community-member-hover-card/CommunityMemberHoverCard.svelte";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import VirtualForumTopicRows from "./VirtualForumTopicRows.svelte";
  import {
    memberById,
    type CommunityMemberModel,
    type CommunityTopicModel,
  } from "../community-workspace/model.js";

  let {
    title = "Threads",
    topics,
    members,
    onOpenTopic,
    onCreateTopic,
    disabled = false,
    composerStatus,
    viewport,
    filtered = false,
  }: {
    title?: string;
    topics: CommunityTopicModel[];
    members: CommunityMemberModel[];
    onOpenTopic?: (id: string) => void;
    onCreateTopic?: (draft: CommunityTopicDraft) => Promise<void>;
    disabled?: boolean;
    composerStatus?: ComposerStatus;
    viewport?: HTMLElement | null;
    filtered?: boolean;
  } = $props();
</script>

<section
  class="forum-topic-list"
  class:forum-topic-list--empty={!topics.length}
  aria-label={`${title} listing`}
>
  {#if onCreateTopic}<CommunityTopicComposer
      forumName={title}
      {members}
      {disabled}
      {composerStatus}
      onPublish={onCreateTopic}
    />{/if}
  {#if !topics.length}<Empty.Root class="forum-topic-list__empty"
      ><Empty.Header
        ><Empty.Media variant="icon"
          ><MessagesSquareIcon aria-hidden="true" /></Empty.Media
        ><Empty.Title
          >{filtered ? "No matching threads" : "No threads yet"}</Empty.Title
        ><Empty.Description
          >{filtered
            ? "Try a different search or clear the filter."
            : "Create a thread to start a conversation."}</Empty.Description
        ></Empty.Header
      ></Empty.Root
    >
  {:else}
    <div class="forum-topic-list__columns" aria-hidden="true">
      <span>Topic</span><span>Replies</span><span>Activity</span>
    </div>
    <div class="forum-topic-list__topics">
      {#snippet row(topic: CommunityTopicModel)}
        {@const author = memberById(members, topic.authorId)}
        <div class="forum-topic-list__topic-row" data-topic-id={topic.id}>
          <Button
            type="button"
            variant="ghost"
            class="forum-topic-list__topic"
            aria-label={`Open thread ${topic.title}`}
            onclick={() => onOpenTopic?.(topic.id)}
          />
          <span
            class="forum-topic-list__author-avatar"
            data-ui-part="forum-topic-author"
            ><CommunityMemberHoverCard
              member={author}
              trigger="avatar"
              avatarSize="medium"
            /></span
          >
          <span class="forum-topic-list__topic-main"
            ><span class="forum-topic-list__title-row"
              ><strong>{topic.title}</strong></span
            ><span class="forum-topic-list__excerpt">{topic.excerpt}</span>
            {#if topic.tags?.length}<span class="forum-topic-list__meta"
                >{#each topic.tags as tag}<CommunityChip>{tag}</CommunityChip
                  >{/each}</span
              >{/if}
          </span>
          <span
            class="forum-topic-list__participants"
            data-ui-part="forum-topic-participants"
            aria-hidden="true"
            >{#each topic.participantIds.slice(0, 3) as id}<span
                class="forum-topic-list__avatar"
                ><CommunityMemberAvatar
                  member={memberById(members, id)}
                  size="xsmall"
                  decorative
                /></span
              >{/each}</span
          >
          <span
            class="forum-topic-list__count"
            aria-label={topic.replyCountAvailable === false
              ? "Reply count unavailable"
              : `${topic.replyCount} replies`}
            >{topic.replyCountAvailable === false
              ? "—"
              : topic.replyCount}</span
          >
          <span class="forum-topic-list__activity"
            >{topic.lastActivityLabel}</span
          >
        </div>
      {/snippet}
      {#if viewport && topics.length > 100}<VirtualForumTopicRows
          {topics}
          {viewport}
          {row}
        />{:else}{#each topics as topic (topic.id)}{@render row(
            topic,
          )}{/each}{/if}
    </div>
  {/if}
</section>
