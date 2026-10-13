<script lang="ts">
  import { untrack, onMount } from "svelte";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import StoreWorkflowHostFields from "./StoreWorkflowHostFields.svelte";
  import { globalSettingsSkeletonCards } from "./global-settings-skeleton.js";
  import WorkflowHostPrincipalPicker from "./WorkflowHostPrincipalPicker.svelte";
  import { provideProfiles } from "../../profiles/context.svelte.js";
  import { workflowAuthors } from "../store-workflows/workflow-presentation.js";
  import {
    createWorkflowHostChoices,
    type WorkflowHostChoices,
  } from "../../controller/workflow-host-choices.svelte.js";
  import type { StoreController } from "../../controller/contracts.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import Workflow from "@lucide/svelte/icons/workflow";
  import Shield from "@lucide/svelte/icons/shield";
  import Save from "@lucide/svelte/icons/save";
  import Plus from "@lucide/svelte/icons/plus";
  import Trash from "@lucide/svelte/icons/trash-2";
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import StoreSettingsFooter from "./StoreSettingsFooter.svelte";
  import StoreSettingsIcon from "./StoreSettingsIcon.svelte";
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  import StoreSettingsSkeleton from "./StoreSettingsSkeleton.svelte";
  import RotateCw from "@lucide/svelte/icons/rotate-cw";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import {
    copy,
    equal,
    type Configuration,
  } from "../../controller/server-configuration.js";
  import type { ServerSettingsManagement } from "../../controller/server-settings.svelte.js";
  import type { RuntimeReload } from "../../controller/runtime-reload.svelte.js";
  let {
    management,
    reload,
    controller,
    choices: suppliedChoices,
  }: {
    management?: ServerSettingsManagement;
    reload?: RuntimeReload;
    controller?: StoreController;
    choices?: WorkflowHostChoices;
  } = $props();
  const edit = $derived(management?.state.drafts.workflow);
  const value = $derived(
    edit?.draft as NonNullable<Configuration["workflow"]> | null,
  );
  const busy = $derived(!!management?.state.saving || !!edit?.unknown);
  const status = $derived(management?.state.value?.workflow_runtime);
  const pending = $derived(
    management?.state.value &&
      !equal(
        management.state.value.desired.workflow,
        management.state.value.effective.workflow,
      ),
  );
  provideProfiles(() => controller?.directory);
  const choices = untrack(
    () =>
      suppliedChoices ??
      createWorkflowHostChoices({ available: () => !!management?.available() }),
  );
  const authors = $derived(controller ? workflowAuthors(controller) : []);
  let reloadTrigger = $state<HTMLButtonElement | null>(null);
  onMount(() => {
    void management?.observe();
    void choices.refresh();
    const timer = setInterval(() => void management?.observe(), 3000);
    return () => {
      clearInterval(timer);
      if (!suppliedChoices) choices.destroy();
    };
  });
  function update(next: NonNullable<Configuration["workflow"]>) {
    management?.configure("workflow", next);
  }
  function mode(next: string) {
    if (next === "form" && !management?.applyWorkflowJson()) return;
    edit!.advancedMode = next === "json" ? "json" : "form";
  }
  function add() {
    if (!value) return;
    const configuration = {
      version: 1,
      host: "local",
      principal: "",
      home: "",
      state_dir: "",
      jj_executable: "",
      allowed_owners: [],
      labels: [],
      sources: [],
      commands: {},
      agents: [],
      agent_endpoint: null,
      poll_millis: 1000,
    };
    edit!.advanced = JSON.stringify(configuration, null, 2);
    update({ ...copy(value), local: { enabled: false, configuration } });
  }
</script>

<StoreNotice
  scope="installation-server-settings"
  noticeKey="workflow-settings"
  title="Workflow settings unavailable"
  message={management?.state.error ||
    edit?.error ||
    edit?.validation ||
    (edit?.conflict
      ? "Server configuration changed. Your workflow host draft is retained; cancel to load the saved settings before editing again."
      : "")}
  pending={management?.state.loading || busy}
/>
<StoreNotice
  scope="installation-server-settings"
  noticeKey="workflow-host-choices"
  title="Repository choices unavailable"
  message={choices.state.error}
  pending={choices.state.loading}
  action={{ id: "retry", label: "Retry", run: choices.refresh }}
/>
<StoreNotice
  scope="installation-server-settings"
  noticeKey="workflow-runtime"
  title="Workflow host needs attention"
  message={status?.message}
/>
<StoreNotice
  scope="installation-server-settings"
  noticeKey="workflow-settings-saved"
  title="Workflow settings saved"
  message={edit?.message}
  severity="info"
  lifetime="timed"
/>
{#if !management?.available()}<p>
    Workflow host settings require local administrator access.
  </p>
{:else if management.state.value && !value}<p>
    This service does not support workflow host settings.
  </p>
{:else if !value}<StoreSettingsSkeleton
    label="Loading workflow host settings"
    cards={globalSettingsSkeletonCards("service", "workflows")}
  />
{:else}
  <form
    onsubmit={(event) => {
      event.preventDefault();
      void management.save("workflow");
    }}
  >
    <StoreSettingsCard
      id="workflow-host-status"
      title="Workflow hosts"
      Icon={Workflow}
      description="Configure this server's execution admission and optional local event/action host. Save first, then reload saved settings."
    >
      <dl class="store-settings__rows">
        <StoreReadOnlyRow
          label="Local host"
          value={!status?.available
            ? "External hosts only"
            : status.busy
              ? "Settling a poll or job"
              : status.message
                ? "Needs attention"
                : status.running
                  ? "Running"
                  : "Stopped"}
        />
        <StoreReadOnlyRow
          label="Configuration"
          value={pending ? "Saved changes awaiting reload" : "Current"}
        />
        <StoreReadOnlyRow
          label="Running admission"
          value={`${management.state.value?.effective.workflow?.admitted.length ?? 0} principals`}
        />
      </dl>
      <Field.Group class="store-settings__editable-rows">
        <Field.Field
          orientation="responsive"
          class="store-settings__option-row"
        >
          <Field.Content
            ><Field.Label for="workflow-admission"
              ><StoreSettingsIcon Icon={Shield} />Admitted execution principals</Field.Label
            ><Field.Description
              >Select known authors or enter a full principal ID. Repository
              access alone does not grant execution.</Field.Description
            ></Field.Content
          >
          <WorkflowHostPrincipalPicker
            id="workflow-admission"
            label="Admitted execution principals"
            values={value.admitted}
            {authors}
            disabled={busy}
            onChange={(admitted) => update({ ...copy(value), admitted })}
          />
        </Field.Field>
      </Field.Group>
    </StoreSettingsCard>
    <Tabs.Root bind:value={() => edit!.advancedMode, mode}>
      <StoreSettingsCard
        id="workflow-local-host"
        title="Local execution host"
        Icon={Workflow}
        description="Named executors and source attachments are owned by the operator. Secret fields contain environment references only."
      >
        {#snippet badge()}
          {#if value.local}
            <Tabs.List aria-label="Local host editor">
              <Tabs.Trigger value="form">Form</Tabs.Trigger>
              <Tabs.Trigger value="json">JSON</Tabs.Trigger>
            </Tabs.List>
          {/if}
        {/snippet}
        {#if value.local}
          <StoreWorkflowHostFields
            {management}
            {value}
            {busy}
            {choices}
            {authors}
            {controller}
            repositories={controller?.state.repositories ?? []}
          />
          <StoreSettingsFooter
            ><Button
              type="button"
              variant="ghost"
              disabled={busy}
              onclick={() => update({ ...copy(value), local: null })}
              ><Trash aria-hidden="true" />Remove local configuration</Button
            ></StoreSettingsFooter
          >
        {:else}<StoreSettingsFooter
            ><Button
              type="button"
              variant="outline"
              disabled={busy}
              onclick={add}><Plus aria-hidden="true" />Add local host</Button
            ></StoreSettingsFooter
          >{/if}
      </StoreSettingsCard>
    </Tabs.Root>
    <StoreSettingsFooter standalone>
      {#snippet description()}Save retains your draft without starting work.
        Reload activates saved settings; busy or uncertain jobs defer
        replacement.{/snippet}
      {#if reload?.value}<Button
          type="button"
          variant="outline"
          bind:ref={reloadTrigger}
          disabled={!!management.state.saving}
          aria-haspopup="dialog"
          onclick={() => {
            reload.returnFocus = () => reloadTrigger?.focus();
            reload.open = true;
            void reload.observe();
          }}><RotateCw aria-hidden="true" />Reload saved settings</Button
        >{/if}
      <Button
        type="button"
        variant="outline"
        disabled={(!edit?.dirty && !edit?.advancedPending) ||
          !!management.state.saving ||
          (!!edit?.unknown && !edit.unknownObserved)}
        onclick={() => {
          management.cancel("workflow");
        }}>Cancel</Button
      >
      <Button
        type="submit"
        disabled={(!edit?.dirty && !edit?.advancedPending) ||
          busy ||
          !!edit?.conflict}
        ><Save aria-hidden="true" />Save workflow settings</Button
      >
    </StoreSettingsFooter>
  </form>
{/if}
