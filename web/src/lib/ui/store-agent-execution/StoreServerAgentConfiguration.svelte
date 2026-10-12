<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import ServerIcon from "@lucide/svelte/icons/server";
  import BotIcon from "@lucide/svelte/icons/bot";
  import StoreSettingsCard from "../store-settings/StoreSettingsCard.svelte";
  import StoreSettingsFooter from "../store-settings/StoreSettingsFooter.svelte";
  import StoreReadOnlyRow from "../store-settings/StoreReadOnlyRow.svelte";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import StoreSettingsSkeleton from "../store-settings/StoreSettingsSkeleton.svelte";
  import StoreServerAgentDefinition from "./StoreServerAgentDefinition.svelte";
  import { newAgentDefinition } from "../../controller/server-agent-administration-types.js";
  import type { ServerAgentAdministration } from "../../controller/server-agent-administration.svelte.js";
  let { manager }: { manager: ServerAgentAdministration } = $props();
  const field = $props.id();
  const disabled = $derived(manager.busy || !!manager.pending);
  function configure() {
    manager.draft = structuredClone(
      $state.snapshot(
        manager.report?.effective ?? {
          acp_executable: "",
          jj_executable: "",
          agents: [],
        },
      ),
    );
  }
</script>

{#if !manager.report}<StoreSettingsSkeleton
    cards={[{ rows: ["fact", "input", "input"], actions: 1 }]}
    label="Loading server agent configuration"
  />
{:else}
  <form
    onsubmit={(event) => {
      event.preventDefault();
      void manager.save();
    }}
    class="server-access-form"
  >
    <StoreSettingsCard
      id={`${field}-deployment`}
      title="Remote execution"
      Icon={ServerIcon}
      description="These are paths on the selected server. Tools and provider credentials must already be installed there."
    >
      <dl>
        <StoreReadOnlyRow
          label="Saved configuration"
          value={manager.report.desired ? "Enabled" : "Disabled"}
        /><StoreReadOnlyRow
          label="Running configuration"
          value={manager.report.effective ? "Enabled" : "Disabled"}
        /><StoreReadOnlyRow
          label="Activation"
          value={manager.report.restart_required
            ? "Server restart required"
            : "Saved and running configuration match"}
        />
      </dl>
      {#if manager.draft}<Field.Group class="store-settings__editable-rows">
          {#each [{ key: "acp_executable", label: "ACP executable" }, { key: "jj_executable", label: "JJ executable" }] as item}
            <StoreWorkField
              label={item.label}
              id={`${field}-${item.key}`}
              description="An absolute path to the prepared server executable."
              {disabled}
              class="store-settings__option-row"
              ><Input
                id={`${field}-${item.key}`}
                bind:value={
                  manager.draft[item.key as "acp_executable" | "jj_executable"]
                }
                required
                {disabled}
              /></StoreWorkField
            >
          {/each}
        </Field.Group>{/if}
      <StoreSettingsFooter
        >{#if manager.draft}<Button
            type="button"
            variant="outline"
            {disabled}
            onclick={() => (manager.draft = null)}>Disable after restart</Button
          >{:else}<Button type="button" {disabled} onclick={configure}
            >Configure remote agents</Button
          >{/if}</StoreSettingsFooter
      >
    </StoreSettingsCard>
    {#if manager.draft}
      {#each manager.draft.agents as definition (definition.setup.id)}<StoreServerAgentDefinition
          {definition}
          onChange={(value) => manager.updateDefinition(value)}
          principals={manager.report.principals}
          profiles={manager.profiles}
          {disabled}
          onRemove={() => {
            if (manager.draft)
              manager.draft.agents = manager.draft.agents.filter(
                (a) => a.setup.id !== definition.setup.id,
              );
          }}
        />{/each}
      <StoreSettingsCard
        id={`${field}-agents`}
        title="Configured agents"
        Icon={BotIcon}
        description="Add a definition and select its allowed principals. Saving does not start an agent."
        ><StoreSettingsFooter
          ><Button
            type="button"
            variant="outline"
            disabled={disabled || manager.draft.agents.length >= 64}
            onclick={() => manager.draft?.agents.push(newAgentDefinition())}
            >Add agent definition</Button
          ></StoreSettingsFooter
        ></StoreSettingsCard
      >
    {/if}
    <StoreSettingsFooter standalone
      >{#snippet description()}{manager.conflict
          ? "Configuration changed on the server. Review it before retaining your draft against the latest revision."
          : "Configuration and sharing changes require a server restart."}{/snippet}{#if manager.conflict}<Button
          type="button"
          variant="outline"
          {disabled}
          onclick={() => manager.useLatestRevision()}
          >Keep draft and use latest revision</Button
        >{/if}<Button
        type="button"
        variant="outline"
        {disabled}
        onclick={() => manager.reset()}>Cancel changes</Button
      ><Button
        type="submit"
        disabled={disabled || !manager.dirty || manager.conflict}
        >Save configuration</Button
      ></StoreSettingsFooter
    >
  </form>
{/if}

<style>
  dl {
    margin: 0;
  }
</style>
