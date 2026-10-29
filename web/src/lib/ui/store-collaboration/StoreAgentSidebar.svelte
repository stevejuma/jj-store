<script lang="ts">
  import type { Snippet } from "svelte";
  import { AppShell } from "@lapismd/design-core/shell";
  import { useAppearance } from "../../appearance/store.svelte.js";
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
  const appearance = useAppearance();
  const documentMode = $derived(
    (appearance?.live.conversationView ?? "document") === "document",
  );
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
{#snippet body(afterItems: Snippet | undefined)}
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
    <StoreAgentContext {controller} showHeader={false} {afterItems} />
  {/if}
  {#if !controller?.state.agentSession || (controller?.execution && controller.execution.metadata.details[metadataKey( { session: controller.state.agentSession.session.session_id, repository: controller.state.repository.id } )])}{@render afterItems?.()}{/if}
{/snippet}

<StoreAgentComments {controller} inlineComposer={documentMode}>
  {#snippet children(comments, composer)}
    {#if focused}
      <StoreDiscussionSheet
        title="Agent response"
        scrollable={!documentMode}
        {open}
        {onClose}
        {returnFocus}
        footer={documentMode ? undefined : composer}
      >
        {#snippet header()}{@render heading(true)}{/snippet}
        {#if documentMode}<div class="store-agent-sidebar__document">
            {@render body(comments)}
          </div>
        {:else}{@render body(undefined)}{@render comments()}{/if}
      </StoreDiscussionSheet>
    {:else}
      <AppShell.Sidebar.Header>
        {@render heading()}
        <AppShell.Sidebar.Close
          label="Close agent response"
          onclick={onClose}
        />
      </AppShell.Sidebar.Header>
      <AppShell.Sidebar.Body>
        {#if documentMode}<div class="store-agent-sidebar__document">
            {@render body(comments)}
          </div>
        {:else}{@render body(undefined)}{@render comments()}{/if}
      </AppShell.Sidebar.Body>
      {#if !documentMode}<AppShell.Sidebar.Footer
          class="store-agent-sidebar__footer"
          >{@render composer()}</AppShell.Sidebar.Footer
        >{/if}
    {/if}
  {/snippet}
</StoreAgentComments>

<style>
  .store-agent-sidebar__document {
    height: 100%;
    min-height: 0;
  }
  .store-agent-sidebar__document :global(.store-agent-context) {
    height: 100%;
  }
  .store-agent-sidebar__document :global(.chat-body-reading-surface) {
    flex: 1;
  }

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
