<script lang="ts">
  import type { StoreController } from "../../controller/contracts.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import XIcon from "@lucide/svelte/icons/x";
  import StoreAgentContext from "../store-collaboration/StoreAgentContext.svelte";
  import StoreCollaborationSkeleton from "../store-collaboration/StoreCollaborationSkeleton.svelte";
  let {
    controller,
    error,
    onClose,
  }: {
    controller?: StoreController;
    error: string;
    onClose(): void;
  } = $props();
</script>

<aside class="store-projects-agent-response" aria-label="Agent response">
  <header>
    <strong>Agent response</strong><Button
      variant="ghost"
      size="icon-sm"
      aria-label="Close agent response"
      onclick={onClose}><XIcon aria-hidden="true" /></Button
    >
  </header>
  <ScrollArea type="hover" class="store-projects-agent-response__scroll">
    {#if error || controller?.state.status.error}<p role="alert">
        {error || controller?.state.status.error}
      </p>{/if}
    {#if controller?.state.status.transcript === "loading" && !controller.state.agentSession}
      <StoreCollaborationSkeleton label="Loading agent response" />
    {:else if controller}<StoreAgentContext {controller} />{/if}
  </ScrollArea>
</aside>

<style>
  .store-projects-agent-response {
    display: flex;
    flex-direction: column;
    min-height: 0;
    min-width: 0;
    border-inline-start: 1px solid var(--border);
    background: var(--background);
  }
  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem;
    border-bottom: 1px solid var(--border);
    font-size: var(--community-text-body);
  }
  :global(.store-projects-agent-response__scroll) {
    flex: 1;
    min-height: 0;
  }
  p {
    padding: 0.75rem;
  }
</style>
