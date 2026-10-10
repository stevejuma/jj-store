<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { AgentExecution } from "../../web/src/lib/controller/agent-execution.svelte.js";
  import StoreAgentLaunchButton from "../../web/src/lib/ui/store-agent-execution/StoreAgentLaunchButton.svelte";
  let {
    manager,
    sessionId,
    history = $bindable(),
    configuration = $bindable(),
    controls = [],
  }: {
    manager: AgentExecution;
    sessionId?: string;
    history?: () => void;
    configuration?: () => void;
    controls?: unknown[];
  } = $props();
</script>

<StoreAgentLaunchButton {manager} /><Button
  disabled={!sessionId}
  onclick={() => sessionId && manager.openSession(sessionId)}
  >Open fixture conversation</Button
><Button onclick={() => manager.directory()}>Manage fixture agents</Button>

{#if history}<Button
    onclick={() => {
      history?.();
      history = undefined;
    }}>Resolve fixture history</Button
  >{/if}
{#if controls.length}<output aria-label="Agent controls"
    >{JSON.stringify(controls)}</output
  >{/if}
{#if configuration}<Button
    onclick={() => {
      configuration?.();
      configuration = undefined;
    }}>Resolve fixture configuration</Button
  >{/if}
