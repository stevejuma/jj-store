<script lang="ts">
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import BotIcon from "@lucide/svelte/icons/bot";
  import { agentMember } from "../community-agents/model.js";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  let { manager }: { manager: AgentExecution } = $props();
  const selected = $derived(
    manager.value?.setups.find((s) => s.id === manager.setupId),
  );
</script>

<StoreRepositoryPicker
  label="Agent"
  variant="ghost"
  value={manager.setupId}
  controlled
  disabled={manager.busy || !!manager.pending}
  searchLabel="Search agents"
  groupLabel="Agents"
  emptyLabel="Choose agent"
  emptyResult="No matching agents."
  repositories={manager.value?.setups.map((s) => ({
    id: s.id,
    name: s.name,
    description: `${s.runner} · ${s.preferences.model || "Harness default model"}`,
  })) ?? []}
  onChange={(id) => {
    const draft = manager.drafts[`new:${manager.setupId || "unassigned"}`];
    if (!manager.drafts[`new:${id}`] && draft)
      manager.drafts[`new:${id}`] = draft;
    manager.setupId = id;
    manager.onNavigate?.("new", id);
  }}
>
  {#snippet triggerIcon()}{#if selected}<CommunityMemberAvatar
        member={agentMember(selected)}
        size="xsmall"
        shape="rounded"
        decorative
      />{:else}<BotIcon aria-hidden="true" />{/if}{/snippet}
  {#snippet itemIcon(id)}{@const agent = manager.value?.setups.find(
      (s) => s.id === id,
    )}{#if agent}<CommunityMemberAvatar
        member={agentMember(agent)}
        size="xsmall"
        shape="rounded"
        decorative
      />{/if}{/snippet}
</StoreRepositoryPicker>
