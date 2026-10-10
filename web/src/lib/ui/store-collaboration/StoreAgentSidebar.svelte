<script lang="ts">
  import { AppShell } from "@lapismd/design-core/shell";
  import type { StoreController } from "../../controller/contracts.js";
  import CommunityProjectsColumnAlert from "../community-projects/CommunityProjectsColumnAlert.svelte";
  import { agentMember } from "../../collaboration/presentation.js";
  import { managedRecordedAgent } from "../../collaboration/managed-agent.js";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import StoreAgentSessionMenu from "./StoreAgentSessionMenu.svelte";
  import StoreDiscussionSheet from "../store-discussion/StoreDiscussionSheet.svelte";
  import StoreAgentComments from "./StoreAgentComments.svelte";
  import StoreConversationMetadata from "../store-agent-execution/StoreConversationMetadata.svelte";
  import { metadataKey } from "../../controller/conversation-metadata.svelte.js";
  import StoreAgentContext from "./StoreAgentContext.svelte";
  import StoreCollaborationSkeleton from "./StoreCollaborationSkeleton.svelte";
  import "./store-collaboration.css";
  let {
    controller,
    error = "",
    onClose,
    focused = false,
    open = true,
    returnFocus,
  }: {
    controller?: StoreController;
    error?: string;
    onClose(): void;
    focused?: boolean;
    open?: boolean;
    returnFocus?: HTMLElement | null;
  } = $props();
  const agent = $derived(
    controller?.state.agentSession
      ? agentMember(
          controller.state.agentSession.session,
          managedRecordedAgent(
            controller.state.repository.id,
            controller.state.agentSession.session,
            controller.execution?.value?.sessions,
          ),
        )
      : undefined,
  );
  const failure = $derived(error || controller?.state.status.error);
</script>

{#snippet heading(sheet = false)}
  {#if agent}<CommunityMemberAvatar
      member={agent}
      size="small"
      decorative
    />{/if}
  {#if sheet}<h1 class="store-agent-sidebar__title">
      Agent response
    </h1>{:else}<strong class="store-agent-sidebar__title"
      >Agent response</strong
    >{/if}
  {#if controller}<StoreAgentSessionMenu {controller} />{/if}
{/snippet}
{#snippet body()}
  {#if failure}
    <CommunityProjectsColumnAlert
      title="Agent response unavailable"
      inset="collection"
      description={failure}
      tone="destructive"
    />
  {/if}
  {#if !failure && !controller?.state.agentSession}
    <StoreCollaborationSkeleton label="Loading agent response" />
  {:else if controller?.execution && controller.state.agentSession && controller.execution.metadata.details[metadataKey( { session: controller.state.agentSession.session.session_id, repository: controller.state.repository.id } )]}
    <StoreConversationMetadata
      manager={controller.execution}
      query={{
        session: controller.state.agentSession.session.session_id,
        repository: controller.state.repository.id,
      }}
    />
  {:else if controller}
    <StoreAgentContext {controller} showHeader={false} />
  {/if}
{/snippet}

<StoreAgentComments {controller}>
  {#snippet children(comments, composer)}
    {#if focused}
      <StoreDiscussionSheet
        title="Agent response"
        {open}
        {onClose}
        {returnFocus}
      >
        {#snippet header()}{@render heading(true)}{/snippet}
        {#snippet footer()}{@render composer()}{/snippet}
        {@render body()}{@render comments()}
      </StoreDiscussionSheet>
    {:else}
      <AppShell.Sidebar.Header>
        {@render heading()}
        <AppShell.Sidebar.Close
          label="Close agent response"
          onclick={onClose}
        />
      </AppShell.Sidebar.Header>
      <AppShell.Sidebar.Body
        >{@render body()}{@render comments()}</AppShell.Sidebar.Body
      >
      <AppShell.Sidebar.Footer class="store-agent-sidebar__footer"
        >{@render composer()}</AppShell.Sidebar.Footer
      >
    {/if}
  {/snippet}
</StoreAgentComments>

<style>
  :global(.store-agent-sidebar__footer) {
    padding: 0;
    border: 0;
    flex: 0 0 auto;
  }
  .store-agent-sidebar__title {
    flex: 1;
    min-width: 0;
    margin: 0;
    font-size: var(--community-text-body);
    font-weight: 600;
  }
</style>
