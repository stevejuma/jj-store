<script lang="ts">
  import { ReactionBar } from "@lapismd/design-core/ai/experimental";
  import * as Popover from "@lapismd/design-core/shadcn/popover";
  import { onDestroy } from "svelte";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import type { CommunityReactionModel } from "../../community/workspace-model.js";
  let {
    reaction,
    onToggle,
  }: { reaction: CommunityReactionModel; onToggle?: (emoji: string) => void } =
    $props();
  let open = $state(false);
  let anchor = $state<HTMLDivElement | null>(null);
  let closing: ReturnType<typeof setTimeout> | undefined;
  function show() {
    clearTimeout(closing);
    open = true;
  }
  function leave() {
    clearTimeout(closing);
    closing = setTimeout(() => (open = false), 120);
  }
  onDestroy(() => clearTimeout(closing));
  const name = $derived(reaction.label ?? reaction.emoji);
</script>

<Popover.Root bind:open>
  <ReactionBar
    bind:ref={anchor}
    reactions={[reaction]}
    label={`${name} reactions`}
    {onToggle}
    onpointerenter={show}
    onpointerleave={leave}
    onfocusin={show}
    onfocusout={leave}
  />
  <Popover.Content
    customAnchor={anchor}
    side="top"
    align="start"
    sideOffset={4}
    role="group"
    aria-label={`People who reacted with ${name}`}
    onpointerenter={show}
    onpointerleave={leave}
    onOpenAutoFocus={(e) => e.preventDefault()}
    onCloseAutoFocus={(e) => e.preventDefault()}
  >
    <strong>{name}</strong>
    <ul>
      {#each reaction.actors ?? [] as member (member.id)}<li>
          <CommunityMemberAvatar {member} size="xsmall" decorative />
          <span>{member.name}</span>
        </li>{/each}
    </ul>
  </Popover.Content>
</Popover.Root>

<style>
  ul {
    display: grid;
    gap: 0.5rem;
    list-style: none;
    margin: 0.5rem 0 0;
    padding: 0;
  }
  li {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }
</style>
