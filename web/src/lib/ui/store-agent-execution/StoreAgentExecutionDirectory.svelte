<script lang="ts">
  import { onMount } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import type { AgentLaunchContext } from "../../controller/agent-execution-types.js";
  import StoreAgentLaunchButton from "./StoreAgentLaunchButton.svelte";
  let {
    manager,
    context = {},
  }: { manager?: AgentExecution; context?: AgentLaunchContext } = $props();
  onMount(() => {
    void manager?.refresh();
    const timer = setInterval(() => void manager?.refresh(), 5000);
    return () => clearInterval(timer);
  });
</script>

{#if manager?.available()}<section
    aria-label="Interactive agent sessions"
    class="store-live-agent-directory"
  >
    <div>
      <StoreAgentLaunchButton {manager} {context} /><Button
        variant="ghost"
        size="sm"
        onclick={async () => {
          await manager.show(context);
          manager.launchOpen = false;
        }}>Manage sessions and setups</Button
      >
    </div>
    {#each manager.value?.sessions.filter((s) => !context.repository || s.repository === context.repository) ?? [] as session (session.id)}<Button
        variant="ghost"
        onclick={async () => {
          await manager.show(context);
          manager.launchOpen = false;
          await manager.select(session.id);
        }}>{session.avatar} {session.title} · {session.state}</Button
      >{/each}
  </section>{/if}

<style>
  .store-live-agent-directory {
    display: grid;
    gap: 0.5rem;
    padding: 1rem;
    border-bottom: 1px solid var(--border);
  }
  .store-live-agent-directory > div {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }
</style>
