<script lang="ts">
  import type { StoreController } from "../../controller/contracts.js";
  import type { AgentSession } from "../../controller/agent-execution-types.js";
  import StoreAgentRequestRecovery from "../store-agent-execution/StoreAgentRequestRecovery.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreAgentBindings from "./StoreAgentBindings.svelte";
  let {
    controller,
    sessions,
  }: { controller: StoreController; sessions: readonly AgentSession[] } =
    $props();
  const manager = $derived(controller.execution!);
</script>

{#each sessions as session (session.id)}
  <section aria-label={`${session.title} participation`}>
    <StoreAgentRequestRecovery {manager} {session} />
    <Button
      variant="ghost"
      size="sm"
      onclick={() => {
        const chat = manager.value?.conversations.find(
          (c) => c.id === session.conversation_id,
        );
        if (chat) manager.chat.show(chat.origin);
      }}>Open chat with {session.title}</Button
    >
    <StoreAgentBindings
      {controller}
      {session}
      events={manager.chat.events[session.id] ?? []}
    />
  </section>
{/each}
