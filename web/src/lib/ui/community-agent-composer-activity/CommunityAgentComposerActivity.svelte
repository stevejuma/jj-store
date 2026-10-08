<script lang="ts">
  import { Reasoning } from "@lapismd/design-core/ai/experimental";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  let {
    member,
    label,
    streaming = false,
    state,
  }: {
    member?: CommunityMemberModel;
    label: string;
    streaming?: boolean;
    state?: string;
  } = $props();
</script>

<div
  class="agent-composer-activity"
  data-ui-component="community-agent-composer-activity"
  data-state={state}
  role="group"
  aria-label="Agent activity"
>
  <div class="agent-composer-activity__button">
    {#if member}<CommunityMemberAvatar
        {member}
        size="xsmall"
        shape="rounded"
        decorative
      />{/if}
    <Reasoning {label} {streaming} />
  </div>
</div>

<style>
  .agent-composer-activity {
    min-height: 2rem;
    max-width: 100%;
    box-sizing: border-box;
    display: flex;
    align-items: center;
    min-width: 0;
    white-space: nowrap;
    padding-inline: 0.25rem;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
  }
  .agent-composer-activity__button {
    display: flex;
    flex: 1 1 auto;
    gap: 0.375rem;
    align-items: center;
    justify-content: flex-start;
    width: 100%;
    height: auto;
    min-height: 2rem;
    min-width: 0;
    max-width: 100%;
    overflow: hidden;
    padding-block: 0.25rem;
    padding-inline: 0.25rem;
  }
  .agent-composer-activity :global([data-ui-component="ai-chat-reasoning"]) {
    flex: 1 1 auto;
    min-width: 0;
  }
</style>
