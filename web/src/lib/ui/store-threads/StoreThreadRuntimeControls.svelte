<script lang="ts">
  import StoreThreadAgentRepositories from "./StoreThreadAgentRepositories.svelte";
  import StoreConversationRuntime from "../store-agent-chat/StoreConversationRuntime.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import type { AgentSession } from "../../controller/agent-execution-types.js";
  let {
    controller,
    parent,
    selectedAgent,
    selectedSession,
    disabled,
    onAgentChange,
  }: {
    controller: StoreController;
    parent?: string;
    selectedAgent?: string;
    selectedSession?: AgentSession;
    disabled: boolean;
    onAgentChange: (id: string) => void;
  } = $props();
</script>

{#if controller.execution}<div class="thread-runtime-controls">
    {#if !parent}<StoreThreadAgentRepositories {controller} />{/if}
    <StoreConversationRuntime
      manager={controller.execution}
      agentId={selectedAgent}
      session={selectedSession}
      {disabled}
      {onAgentChange}
    />
  </div>{/if}

<style>
  .thread-runtime-controls {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    min-width: 0;
  }
</style>
