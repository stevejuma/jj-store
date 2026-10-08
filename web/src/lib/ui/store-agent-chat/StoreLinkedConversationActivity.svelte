<script lang="ts">
  import type { StoreController } from "../../controller/contracts.js";
  import type { AgentSession } from "../../controller/agent-execution-types.js";
  import { object, text } from "../../controller/agent-execution-types.js";
  import { Reasoning } from "@lapismd/design-core/ai/experimental";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreAgentBindings from "./StoreAgentBindings.svelte";
  let {
    controller,
    sessions,
  }: { controller: StoreController; sessions: readonly AgentSession[] } =
    $props();
  const manager = $derived(controller.execution!);
  function stop(id: string) {
    const started = [...(manager.chat.events[id] ?? [])]
      .reverse()
      .find((e) => object(e.payload).type === "started");
    if (started)
      void manager.commandSession(id, {
        action: "cancel",
        target: text(object(started.payload).requestId),
      });
  }
</script>

{#each sessions as session (session.id)}
  <section aria-label={`${session.title} participation`}>
    {#if session.connected && session.state === "running"}<Reasoning
        label={`${session.title} is working`}
        streaming
      />
      <Button
        variant="ghost"
        size="sm"
        disabled={manager.busy || !!manager.pending}
        onclick={() => stop(session.id)}>Stop turn</Button
      >{/if}
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
