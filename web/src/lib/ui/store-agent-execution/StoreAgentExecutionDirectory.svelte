<script lang="ts">
  import { onMount } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import type { AgentLaunchContext } from "../../controller/agent-execution-types.js";
  let {
    manager,
    context = {},
  }: {
    manager?: AgentExecution;
    context?: AgentLaunchContext;
  } = $props();
  const sessions = $derived(
    manager?.value?.sessions.filter(
      (s) => !context.repository || s.repository === context.repository,
    ) ?? [],
  );
  onMount(() => {
    void manager?.refresh();
  });
</script>

{#if manager?.available() && sessions.length}<section
    aria-label="Interactive agent sessions"
    class="store-live-agent-directory"
  >
    {#each sessions as session (session.id)}<Button
        variant="ghost"
        onclick={async () => {
          manager.openSession(session.id);
        }}
        >{session.conversation_title ?? "New conversation"} · {session.state}</Button
      >{/each}
  </section>{/if}

<style>
  .store-live-agent-directory {
    display: grid;
    gap: 0.5rem;
    padding: 1rem;
    border-bottom: 1px solid var(--border);
  }
</style>
