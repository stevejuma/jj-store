<script lang="ts">
  import { untrack } from "svelte";
  import ChatRuntimeControls from "../store-agent-chat-body/ChatRuntimeControls.svelte";
  import { runtimeChoices } from "../store-agent-execution/presentation.js";
  import { agentMember } from "../community-agents/model.js";
  import {
    object,
    type AgentSession,
  } from "../../controller/agent-execution-types.js";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  let {
    manager,
    session,
    agentId,
    onAgentChange,
    disabled = false,
  }: {
    manager: AgentExecution;
    session?: AgentSession;
    agentId?: string;
    onAgentChange?: (id: string) => void;
    disabled?: boolean;
  } = $props();
  const setup = $derived(
    manager.value?.setups.find((s) => s.id === (agentId ?? session?.setup_id)),
  );
  const member = $derived(
    agentMember(
      setup ?? {
        id: session?.id ?? "agent",
        name: session?.title ?? "Choose agent",
        avatar: session?.avatar ?? "",
      },
    ),
  );
  const preferences = $derived(setup?.preferences ?? session?.preferences);
  const facts = $derived(
    session?.connected
      ? object(object(session.capabilities).session)
      : setup
        ? manager.catalogs.values[manager.catalogs.key(setup.runner, setup.id)]
            ?.facts
        : undefined,
  );
  const models = $derived(runtimeChoices(facts, "model"));
  const thinking = $derived(runtimeChoices(facts, "thought_level"));
  $effect(() => {
    const selected = setup;
    if (selected && !session?.connected)
      untrack(
        () => void manager.catalogs.load(selected.runner, false, selected.id),
      );
  });
  async function configure(key: "model" | "effort", value: string) {
    if (disabled || manager.busy || manager.pending || !preferences) return;
    if (
      !(key === "model" ? models : thinking).some(
        (choice) => choice.id === value,
      )
    )
      return;
    const next = { ...preferences, [key]: value };
    // Save only the preference. The provider and launch environment stay attached
    // to their setup; an existing process is configured through its own receipt.
    if (setup) {
      await manager.saveSetup(
        { ...setup, preferences: next },
        undefined,
        manager.value?.revision,
      );
      if (manager.error || manager.pending) return;
    }
    if (session?.connected)
      await manager.commandSession(session.id, {
        action: "configure",
        preferences: next,
      });
  }
</script>

<ChatRuntimeControls
  agents={onAgentChange
    ? (manager.value?.setups ?? []).map(agentMember)
    : [member]}
  agent={member}
  model={preferences?.model ?? "Harness default"}
  thinking={preferences?.effort ?? "Harness default"}
  models={models.map((m) => ({ value: m.id, label: m.name }))}
  thinkingOptions={thinking.map((m) => ({ value: m.id, label: m.name }))}
  disabled={disabled || manager.busy || !!manager.pending}
  onAgentChange={(id) => onAgentChange?.(id)}
  onModelChange={(id) => void configure("model", id)}
  onThinkingChange={(id) => void configure("effort", id)}
/>
