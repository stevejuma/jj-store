<script lang="ts">
  import BotIcon from "@lucide/svelte/icons/bot";
  import StoreSettingsCard from "../store-settings/StoreSettingsCard.svelte";
  import { provideSettingsPresentation } from "../store-settings/settings-presentation.js";
  import StoreAgentConfiguration from "./StoreAgentConfiguration.svelte";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import type { AgentConfiguration } from "../../controller/agent-configuration.js";
  let {
    manager,
    draft = $bindable(),
    secrets,
    pending,
    active,
    scope,
  }: {
    manager: AgentExecution;
    draft: AgentConfiguration;
    secrets: Record<string, string>;
    pending: boolean;
    active: boolean;
    scope: string;
  } = $props();
  const id = $props.id();
  provideSettingsPresentation({ value: "global" });
</script>

<div class="agent-configuration-card store-settings--global">
  <StoreSettingsCard
    {id}
    title="AI configuration"
    Icon={BotIcon}
    description="Harness, model and execution preferences for this agent."
  >
    <StoreAgentConfiguration
      {manager}
      bind:draft
      {secrets}
      {pending}
      {active}
      {scope}
    />
  </StoreSettingsCard>
</div>

<style>
  .agent-configuration-card :global(.store-settings__section) {
    margin-block-start: 0;
  }
</style>
