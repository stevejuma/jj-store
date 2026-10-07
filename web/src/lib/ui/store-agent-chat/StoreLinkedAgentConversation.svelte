<script lang="ts">
  import { untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import type { CommunityMemberModel } from "../../community/workspace-model.js";
  import { activityRows } from "../store-agent-execution/presentation.js";
  import { agentMember } from "../community-agents/model.js";
  import StoreAgentInboxMessage from "../store-agent-execution/StoreAgentInboxMessage.svelte";
  let {
    manager,
    scope,
    home,
    member,
  }: {
    manager: AgentExecution;
    scope: string;
    home: string | null;
    member: CommunityMemberModel;
  } = $props();
  const conversations = $derived(
    manager.value?.conversations.filter(
      (c) => c.origin.scope === scope && c.origin.home === home,
    ) ?? [],
  );
  const ids = $derived(new Set(conversations.map((c) => c.id)));
  const sessions = $derived(
    manager.value?.sessions.filter(
      (s) => s.conversation_id && ids.has(s.conversation_id),
    ) ?? [],
  );
  const rows = $derived(
    sessions
      .flatMap((session) =>
        activityRows(manager.chat.events[session.id] ?? [])
          .filter((row) => row.kind === "user" || row.kind === "assistant")
          .map((row) => ({ row, session, key: `${session.id}:${row.id}` })),
      )
      .sort(
        (a, b) => (a.row.observedAtMillis ?? 0) - (b.row.observedAtMillis ?? 0),
      ),
  );
  $effect(() => {
    const currentScope = scope,
      currentHome = home;
    return untrack(() => manager.chat.watch(currentScope, currentHome));
  });
</script>

{#if conversations.length}
  <section
    aria-label="Linked agent conversation"
    class="linked-agent-conversation"
  >
    <div class="linked-agent-heading">
      <strong>Agent conversation</strong><Button
        size="sm"
        variant="ghost"
        onclick={() => {
          const c = conversations.at(-1)!;
          manager.chat.show(c.origin);
        }}>Open chat</Button
      >
    </div>
    {#each rows as { row, session, key } (key)}
      {#if row.kind === "user" || row.kind === "assistant"}<StoreAgentInboxMessage
          {manager}
          sessionId={session.id}
          row={{ ...row, kind: row.kind }}
          author={row.kind === "user"
            ? member
            : {
                ...agentMember({
                  id: session.setup_id ?? session.id,
                  name: session.title,
                  avatar: session.avatar,
                }),
                acpAgentId: session.setup_id ?? undefined,
              }}
        />{/if}
    {/each}
    {#if manager.chat.streamError}<p role="status">
        {manager.chat.streamError}
      </p>{/if}
  </section>
{/if}

<style>
  .linked-agent-conversation {
    border-top: 1px solid var(--border);
    padding: 0.5rem;
    max-height: 24rem;
    overflow-y: auto;
  }
  .linked-agent-heading {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
  }
</style>
