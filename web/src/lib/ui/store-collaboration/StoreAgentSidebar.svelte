<script lang="ts">
  import { AppShell } from "@lapismd/design-core/shell";
  import type { StoreController } from "../../controller/contracts.js";
  import CommunityProjectsColumnAlert from "../community-projects/CommunityProjectsColumnAlert.svelte";
  import { agentMember } from "../../collaboration/presentation.js";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import StoreAgentSessionMenu from "./StoreAgentSessionMenu.svelte";
  import StoreAgentContext from "./StoreAgentContext.svelte";
  import StoreCollaborationSkeleton from "./StoreCollaborationSkeleton.svelte";
  import "./store-collaboration.css";
  let {
    controller,
    error = "",
    onClose,
  }: {
    controller?: StoreController;
    error?: string;
    onClose(): void;
  } = $props();
  const agent = $derived(
    controller?.state.agentSession
      ? agentMember(controller.state.agentSession.session)
      : undefined,
  );
  const failure = $derived(error || controller?.state.status.error);
</script>

<AppShell.Sidebar.Header>
  {#if agent}<CommunityMemberAvatar
      member={agent}
      size="small"
      decorative
    />{/if}
  <strong class="store-agent-sidebar__title">Agent response</strong>
  {#if controller}<StoreAgentSessionMenu {controller} />{/if}
  <AppShell.Sidebar.Close label="Close agent response" onclick={onClose} />
</AppShell.Sidebar.Header>
<AppShell.Sidebar.Body>
  {#if failure}
    <CommunityProjectsColumnAlert
      title="Agent response unavailable"
      description={failure}
      tone="destructive"
    />
  {/if}
  {#if !failure && !controller?.state.agentSession}
    <StoreCollaborationSkeleton label="Loading agent response" />
  {:else if controller}
    <StoreAgentContext {controller} showHeader={false} />
  {/if}
</AppShell.Sidebar.Body>

<style>
  .store-agent-sidebar__title {
    flex: 1;
    min-width: 0;
  }
</style>
