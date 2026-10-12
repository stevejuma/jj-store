<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import * as Select from "@lapismd/design-core/shadcn/select";
  import { Checkbox } from "@lapismd/design-core/shadcn/checkbox";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Textarea } from "@lapismd/design-core/shadcn/textarea";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import BotIcon from "@lucide/svelte/icons/bot";
  import UsersIcon from "@lucide/svelte/icons/users";
  import StoreSettingsCard from "../store-settings/StoreSettingsCard.svelte";
  import StoreSettingsFooter from "../store-settings/StoreSettingsFooter.svelte";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import StoreReadOnlyRow from "../store-settings/StoreReadOnlyRow.svelte";
  import type { ServerAgentDefinition } from "../../controller/server-agent-administration-types.js";
  let {
    definition,
    principals,
    profiles,
    disabled,
    onRemove,
    onChange,
  }: {
    definition: ServerAgentDefinition;
    principals: readonly string[];
    profiles: Record<string, string>;
    disabled: boolean;
    onRemove(): void;
    onChange(definition: ServerAgentDefinition): void;
  } = $props();
  const field = $props.id();
  const choices = $derived([
    ...new Set([...principals, ...definition.principals]),
  ]);
  function share(principal: string, checked: boolean) {
    onChange({
      ...definition,
      principals: checked
        ? [...new Set([...definition.principals, principal])]
        : definition.principals.filter((p) => p !== principal),
    });
  }
  function editSetup(
    key: "name" | "avatar" | "description" | "instructions" | "runner",
    value: string,
  ) {
    onChange({ ...definition, setup: { ...definition.setup, [key]: value } });
  }
  function editPreference(key: "model" | "effort" | "mode", value: string) {
    onChange({
      ...definition,
      setup: {
        ...definition.setup,
        preferences: { ...definition.setup.preferences, [key]: value || null },
      },
    });
  }
  function editOption(key: "provider" | "endpoint" | "account", value: string) {
    onChange({
      ...definition,
      setup: {
        ...definition.setup,
        options: { ...definition.setup.options, [key]: value || null },
      },
    });
  }
</script>

<StoreSettingsCard
  id={`${field}-agent`}
  title={definition.setup.name || "New agent"}
  label={`Agent definition ${definition.setup.id}`}
  Icon={BotIcon}
  description="Runtime definitions apply to future sessions after configuration reload."
>
  <dl>
    <StoreReadOnlyRow label="Agent ID" value={definition.setup.id} copyable />
  </dl>
  <Field.Group class="store-settings__editable-rows">
    {#each [{ key: "name", label: "Name", max: 128 }, { key: "avatar", label: "Avatar", max: 2048 }, { key: "description", label: "Description", max: 280 }] as item}
      <StoreWorkField
        label={item.label}
        id={`${field}-${item.key}`}
        {disabled}
        class="store-settings__option-row"
        ><Input
          id={`${field}-${item.key}`}
          value={definition.setup[
            item.key as "name" | "avatar" | "description"
          ]}
          oninput={(event) =>
            editSetup(
              item.key as "name" | "avatar" | "description",
              event.currentTarget.value,
            )}
          required={item.key === "name"}
          maxlength={item.max}
          {disabled}
        /></StoreWorkField
      >
    {/each}
    <StoreWorkField
      label="Instructions"
      id={`${field}-instructions`}
      {disabled}
      class="store-settings__option-row"
      ><Textarea
        id={`${field}-instructions`}
        value={definition.setup.instructions}
        oninput={(event) =>
          editSetup("instructions", event.currentTarget.value)}
        maxlength={32768}
        {disabled}
      /></StoreWorkField
    >
    <StoreWorkField
      label="Harness"
      id={`${field}-runner`}
      {disabled}
      class="store-settings__option-row"
    >
      <Select.Root
        type="single"
        value={definition.setup.runner}
        onValueChange={(value) => editSetup("runner", value)}
        {disabled}
        ><Select.Trigger id={`${field}-runner`}
          >{definition.setup.runner === "codex"
            ? "Codex"
            : definition.setup.runner === "cursor"
              ? "Cursor"
              : "Local"}</Select.Trigger
        ><Select.Content
          >{#each ["codex", "cursor", "local"] as runner}<Select.Item
              value={runner}>{runner}</Select.Item
            >{/each}</Select.Content
        ></Select.Root
      >
    </StoreWorkField>
    {#each [{ key: "model", label: "Model" }, { key: "effort", label: "Thinking" }, { key: "mode", label: "Execution mode" }] as item}
      <StoreWorkField
        label={item.label}
        id={`${field}-${item.key}`}
        description={item.key === "mode"
          ? "Use an exact mode ID advertised by the installed harness."
          : "Leave empty to use the harness default."}
        {disabled}
        class="store-settings__option-row"
        ><Input
          id={`${field}-${item.key}`}
          value={definition.setup.preferences[
            item.key as "model" | "effort" | "mode"
          ] ?? ""}
          oninput={(event) =>
            editPreference(
              item.key as "model" | "effort" | "mode",
              event.currentTarget.value,
            )}
          {disabled}
        /></StoreWorkField
      >
    {/each}
    {#each [{ key: "provider", label: "Provider" }, { key: "endpoint", label: "Provider endpoint" }, { key: "account", label: "Credential account" }] as item}
      <StoreWorkField
        label={item.label}
        id={`${field}-${item.key}`}
        description={item.key === "account"
          ? "An existing host credential reference; never enter a secret."
          : "Leave empty to retain the harness default."}
        {disabled}
        class="store-settings__option-row"
        ><Input
          id={`${field}-${item.key}`}
          value={definition.setup.options[
            item.key as "provider" | "endpoint" | "account"
          ] ?? ""}
          oninput={(event) =>
            editOption(
              item.key as "provider" | "endpoint" | "account",
              event.currentTarget.value,
            )}
          {disabled}
        /></StoreWorkField
      >
    {/each}
  </Field.Group>
  <StoreSettingsFooter
    ><Button type="button" variant="destructive" {disabled} onclick={onRemove}
      >Remove from configuration</Button
    ></StoreSettingsFooter
  >
</StoreSettingsCard>
<StoreSettingsCard
  id={`${field}-sharing`}
  title={`Who can message ${definition.setup.name || "this agent"}`}
  label={`Sharing for agent ${definition.setup.id}`}
  Icon={UsersIcon}
  description="Selected principals also need independent Write access to the conversation home and attached code repositories. Sharing changes activate at reload."
>
  <Field.Group class="store-settings__editable-rows">
    {#each choices as principal (principal)}
      <StoreWorkField
        label={profiles[principal] || principal}
        id={`${field}-principal-${principal}`}
        description={profiles[principal]
          ? principal
          : principals.includes(principal)
            ? "Enrolled installation principal"
            : "Not currently enrolled; execution remains unavailable."}
        {disabled}
        class="store-settings__option-row"
        ><Checkbox
          id={`${field}-principal-${principal}`}
          checked={definition.principals.includes(principal)}
          onCheckedChange={(checked) => share(principal, checked)}
          {disabled}
        /></StoreWorkField
      >
    {:else}<p class="server-access-help">
        Enroll a principal before configuring sharing. An agent requires at
        least one allowed principal.
      </p>{/each}
  </Field.Group>
</StoreSettingsCard>

<style>
  dl {
    margin: 0;
  }
</style>
