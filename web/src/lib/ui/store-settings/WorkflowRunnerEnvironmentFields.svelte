<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { TagList } from "@lapismd/design-core/shadcn/tag-list";
  import { ChipAutocomplete } from "@lapismd/design-core/forms";
  import Workflow from "@lucide/svelte/icons/workflow";
  import Tags from "@lucide/svelte/icons/tags";
  import Terminal from "@lucide/svelte/icons/terminal";
  import Bot from "@lucide/svelte/icons/bot";
  import WorkflowForgejoFields from "./WorkflowForgejoFields.svelte";
  import StoreSettingsIcon from "./StoreSettingsIcon.svelte";
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import { agentMember } from "../community-agents/model.js";
  import type { RunnerEnvironment } from "../../controller/workflow-runner-environment.js";
  import type { StoreController } from "../../controller/contracts.js";
  let {
    configuration,
    controller,
    disabled,
    onChange,
  }: {
    configuration: RunnerEnvironment;
    controller?: StoreController;
    disabled: boolean;
    onChange(key: keyof RunnerEnvironment, value: unknown): void;
  } = $props();
</script>

<Field.Field orientation="responsive" class="store-settings__option-row">
  <Field.Content
    ><Field.Label for="workflow-host"
      ><StoreSettingsIcon Icon={Workflow} />Host name</Field.Label
    ><Field.Description
      >A stable name for this execution environment.</Field.Description
    ></Field.Content
  >
  <Input
    id="workflow-host"
    class="store-settings__input"
    value={configuration.host}
    {disabled}
    oninput={(e) => onChange("host", e.currentTarget.value)}
  />
</Field.Field>
<Field.Field orientation="responsive" class="store-settings__option-row">
  <Field.Content
    ><Field.Label for="workflow-labels"
      ><StoreSettingsIcon Icon={Tags} />Runner labels</Field.Label
    ><Field.Description
      >Capabilities used to match eligible jobs.</Field.Description
    ></Field.Content
  >
  <fieldset {disabled} class="workflow-runner-labels">
    <ChipAutocomplete
      id="workflow-labels"
      label="Runner labels"
      showLabel={false}
      uppercase={false}
      placeholder="Add runner label…"
      value={configuration.labels ?? []}
      onChange={(items) => {
        if (!disabled) onChange("labels", items);
      }}
    />
  </fieldset>
</Field.Field>
<Field.Field orientation="responsive" class="store-settings__option-row">
  <Field.Content
    ><Field.Title
      ><StoreSettingsIcon Icon={Terminal} />Command presets</Field.Title
    ><Field.Description
      >Named executors, environment variables and artifact paths are configured
      in JSON.</Field.Description
    ></Field.Content
  >
  <div class="workflow-runner-commands">
    {#if Object.keys(configuration.commands ?? {}).length}
      <TagList
        label="Command presets"
        tags={Object.keys(configuration.commands ?? {}).map((command) => ({
          id: command,
          label: command,
          variant: "secondary",
        }))}
      />
    {:else}<span>No command presets</span>{/if}
  </div>
</Field.Field>
<Field.Field orientation="responsive" class="store-settings__option-row">
  <Field.Content
    ><Field.Label for="workflow-agents"
      ><StoreSettingsIcon Icon={Bot} />Allowed agents</Field.Label
    ><Field.Description
      >Existing admitted setups. Source attachments come from job context.</Field.Description
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
      ...(configuration.agents ?? [])
        .filter(
          (id) =>
            !controller?.execution?.value?.setups.some((a) => a.id === id),
        )
        .map((id) => ({ id, name: id })),
    ]}
    multiple
    keepItemIcon
    bind:values={() => configuration.agents ?? [], (v) => onChange("agents", v)}
    allowEmpty
    emptyLabel="No agents"
    selectionLabel="agents"
    {disabled}
    onOpen={() => void controller?.execution?.ensure()}
    searchLabel="Find an admitted agent"
    groupLabel="Agents"
  >
    {#snippet triggerIcon()}<Bot aria-hidden="true" />{/snippet}
    {#snippet itemIcon(id)}{@const agent =
        controller?.execution?.value?.setups.find(
          (a) => a.id === id,
        )}{#if agent}<CommunityMemberAvatar
          member={agentMember(agent)}
          size="xsmall"
          shape="rounded"
          decorative
        />{:else}<Bot aria-hidden="true" />{/if}{/snippet}
  </StoreRepositoryPicker>
</Field.Field>

<WorkflowForgejoFields {configuration} {disabled} {onChange} />

<style>
  .workflow-runner-labels {
    width: min(22rem, 100%);
    min-width: 0;
    margin: 0;
    padding: 0;
    border: 0;
  }
  .workflow-runner-commands {
    width: min(22rem, 100%);
    min-width: 0;
  }
</style>
