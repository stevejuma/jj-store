<script lang="ts">
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import WorkflowHostIdentityFields from "./WorkflowHostIdentityFields.svelte";
  import WorkflowHostSources from "./WorkflowHostSources.svelte";
  import WorkflowHostConfigEditor from "./WorkflowHostConfigEditor.svelte";
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import StoreSettingsIcon from "./StoreSettingsIcon.svelte";
  import Workflow from "@lucide/svelte/icons/workflow";
  import Terminal from "@lucide/svelte/icons/terminal";
  import Bot from "@lucide/svelte/icons/bot";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Switch } from "@lapismd/design-core/shadcn/switch";
  import type {
    StoreController,
    StoreRepositorySummary,
  } from "../../controller/contracts.js";
  import type { WorkflowHostChoices } from "../../controller/workflow-host-choices.svelte.js";
  import StoreNumericSettingNumberRow from "./StoreNumericSettingNumberRow.svelte";
  import { millisecondsUnits } from "./numeric-setting.js";
  import {
    copy,
    type Configuration,
  } from "../../controller/server-configuration.js";
  import type { ServerSettingsManagement } from "../../controller/server-settings.svelte.js";
  let {
    management,
    value,
    busy,
    choices,
    repositories = [],
    authors = [],
    controller,
  }: {
    management: ServerSettingsManagement;
    value: NonNullable<Configuration["workflow"]>;
    busy: boolean;
    choices: WorkflowHostChoices;
    repositories?: readonly StoreRepositorySummary[];
    authors?: readonly { id: string; name: string }[];
    controller?: StoreController;
  } = $props();
  const edit = $derived(management.state.drafts.workflow);
  const status = $derived(management.state.value?.workflow_runtime);
  let advanced = $state("");
  $effect(() => {
    advanced =
      edit.advanced ??
      (value.local ? JSON.stringify(value.local!.configuration, null, 2) : "");
  });
  function update(next: NonNullable<Configuration["workflow"]>) {
    management.configure("workflow", next);
  }
  function configure(key: string, next: unknown) {
    if (!value?.local) return;
    const updated = copy(value);
    updated.local!.configuration = {
      ...updated.local!.configuration,
      [key]: next,
    };
    edit.advanced = advanced = JSON.stringify(
      updated.local!.configuration,
      null,
      2,
    );
    edit.advancedPending = false;
    update(updated);
  }
</script>

<Tabs.Content value="form">
  <Field.Group class="store-settings__editable-rows">
    <Field.Field orientation="responsive" class="store-settings__option-row">
      <Field.Content
        ><Field.Label for="workflow-enabled"
          ><StoreSettingsIcon Icon={Workflow} />Enable local host</Field.Label
        ><Field.Description
          >Activation polls eligible workflows continuously. Active jobs retain
          their configuration. Its execution principal must also be admitted
          above.</Field.Description
        ></Field.Content
      >
      <Switch
        id="workflow-enabled"
        checked={value.local!.enabled}
        disabled={busy || !status?.available}
        onCheckedChange={(enabled) =>
          update({ ...copy(value), local: { ...value.local!, enabled } })}
      />
    </Field.Field>
    <WorkflowHostIdentityFields
      configuration={value.local!.configuration}
      {choices}
      {repositories}
      {authors}
      disabled={busy}
      onChange={configure}
    />
    <WorkflowHostSources
      sources={value.local!.configuration.sources}
      {choices}
      {repositories}
      targets={controller?.execution?.targets ?? []}
      onLoadTargets={() => void controller?.execution?.ensure()}
      disabled={busy}
      onChange={(v) => configure("sources", v)}
    />
    <Field.Field orientation="responsive" class="store-settings__option-row"
      ><Field.Content
        ><Field.Label for="workflow-agents"
          ><StoreSettingsIcon Icon={Bot} />Allowed agents</Field.Label
        ><Field.Description
          >Existing admitted setups; selecting one does not grant dispatch
          access.</Field.Description
        ></Field.Content
      >
      <StoreRepositoryPicker
        id="workflow-agents"
        label="Allowed agents"
        repositories={[
          ...(controller?.execution?.value?.setups ?? []).map((a) => ({
            id: a.id,
            name: a.name,
          })),
          ...(value.local!.configuration.agents ?? [])
            .filter(
              (id) =>
                !controller?.execution?.value?.setups.some((a) => a.id === id),
            )
            .map((id) => ({ id, name: id })),
        ]}
        multiple
        bind:values={
          () => value.local!.configuration.agents ?? [],
          (v) => configure("agents", v)
        }
        allowEmpty
        emptyLabel="No agents"
        selectionLabel="agents"
        disabled={busy}
        onOpen={() => void controller?.execution?.ensure()}
        searchLabel="Find an admitted agent"
        groupLabel="Agents"
      >
        {#snippet triggerIcon()}<Bot aria-hidden="true" />{/snippet}
      </StoreRepositoryPicker>
    </Field.Field>
    <StoreNumericSettingNumberRow
      id="workflow-poll"
      label="Polling interval"
      description="How often the host checks for eligible events and runs."
      min={100}
      max={60000}
      value={value.local!.configuration.poll_millis ?? 1000}
      units={millisecondsUnits}
      disabled={busy}
      onValueChange={(n) => configure("poll_millis", n)}
    />
  </Field.Group>
</Tabs.Content>
<Tabs.Content value="json">
  <Field.Field class="workflow-host-json">
    <Field.Content
      ><Field.Label
        ><StoreSettingsIcon Icon={Terminal} />Host configuration</Field.Label
      ><Field.Description
        >This edits the same local host as Form. Save validates and applies the
        JSON draft. Secret mappings contain environment names.</Field.Description
      ></Field.Content
    >
    <WorkflowHostConfigEditor
      bind:value={advanced}
      disabled={busy}
      onChange={(text) => {
        edit.advanced = text;
        edit.advancedPending = true;
      }}
    />
  </Field.Field>
</Tabs.Content>

<style>
  :global(.workflow-host-json) {
    padding: 1rem;
  }
</style>
