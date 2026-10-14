<script lang="ts">
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Switch } from "@lapismd/design-core/shadcn/switch";
  import Workflow from "@lucide/svelte/icons/workflow";
  import Terminal from "@lucide/svelte/icons/terminal";
  import WorkflowRunnerEnvironmentFields from "./WorkflowRunnerEnvironmentFields.svelte";
  import WorkflowRunnerAdvanced from "./WorkflowRunnerAdvanced.svelte";
  import WorkflowHostConfigEditor from "./WorkflowHostConfigEditor.svelte";
  import StoreSettingsIcon from "./StoreSettingsIcon.svelte";
  import {
    runnerEnvironment,
    type RunnerEnvironment,
  } from "../../controller/workflow-runner-environment.js";
  import {
    copy,
    type Configuration,
  } from "../../controller/server-configuration.js";
  import type { ServerSettingsManagement } from "../../controller/server-settings.svelte.js";
  import type { StoreController } from "../../controller/contracts.js";
  let {
    management,
    value,
    busy,
    authors = [],
    controller,
  }: {
    management: ServerSettingsManagement;
    value: NonNullable<Configuration["workflow"]>;
    busy: boolean;
    authors?: readonly { id: string; name: string }[];
    controller?: StoreController;
  } = $props();
  const edit = $derived(management.state.drafts.workflow);
  const environment = $derived(runnerEnvironment(value.local!.configuration));
  const status = $derived(management.state.value?.workflow_runtime);
  let advanced = $state("");
  $effect(() => {
    advanced = edit.advanced ?? JSON.stringify(environment, null, 2);
  });
  function configure(key: keyof RunnerEnvironment, next: unknown) {
    const updated = copy(value);
    updated.local!.configuration = { ...environment, [key]: next };
    edit.advanced = advanced = JSON.stringify(
      updated.local!.configuration,
      null,
      2,
    );
    edit.advancedPending = false;
    management.configure("workflow", updated);
  }
</script>

<Tabs.Content value="form">
  <Field.Group class="store-settings__editable-rows">
    <Field.Field orientation="responsive" class="store-settings__option-row">
      <Field.Content
        ><Field.Label for="workflow-enabled"
          ><StoreSettingsIcon Icon={Workflow} />Enable local host</Field.Label
        ><Field.Description
          >Uses this service's registered native attachments and each job's
          pinned source. Execution access is admitted separately.</Field.Description
        ></Field.Content
      >
      <Switch
        id="workflow-enabled"
        checked={value.local!.enabled}
        disabled={busy || !status?.available}
        onCheckedChange={(enabled) =>
          management.configure("workflow", {
            ...copy(value),
            local: { ...value.local!, enabled, configuration: environment },
          })}
      />
    </Field.Field>
    <WorkflowRunnerEnvironmentFields
      configuration={environment}
      {controller}
      disabled={busy}
      onChange={configure}
    />
  </Field.Group>
  <WorkflowRunnerAdvanced
    configuration={environment}
    {authors}
    disabled={busy}
    onChange={configure}
  />
</Tabs.Content>
<Tabs.Content value="json">
  <Field.Field class="workflow-host-json">
    <Field.Content
      ><Field.Label
        ><StoreSettingsIcon Icon={Terminal} />Runner environment</Field.Label
      ><Field.Description
        >The same host draft as Form. Repository, revision and identity come
        from the local service and job context. Secret mappings contain
        environment names.</Field.Description
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
