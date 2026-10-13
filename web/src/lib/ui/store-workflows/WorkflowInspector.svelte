<script lang="ts">
  import StoreSettingsIcon from "../store-settings/StoreSettingsIcon.svelte";
  import GitBranch from "@lucide/svelte/icons/git-branch";
  import FolderGit from "@lucide/svelte/icons/folder-git-2";
  import Tags from "@lucide/svelte/icons/tags";
  import TextCursor from "@lucide/svelte/icons/text-cursor-input";
  import Terminal from "@lucide/svelte/icons/terminal";
  import Bot from "@lucide/svelte/icons/bot";
  import StoreNumericSettingRow from "../store-settings/StoreNumericSettingRow.svelte";
  import { secondsUnits } from "../store-settings/numeric-setting.js";
  import Braces from "@lucide/svelte/icons/braces";
  import Message from "@lucide/svelte/icons/message-square";
  import Trash from "@lucide/svelte/icons/trash-2";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Textarea } from "@lapismd/design-core/shadcn/textarea";
  import { ChipAutocomplete } from "@lapismd/design-core/forms";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import WorkflowTriggerPicker from "./WorkflowTriggerPicker.svelte";
  import WorkflowConditions from "./WorkflowConditions.svelte";
  import WorkflowSelect from "./WorkflowSelect.svelte";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import X from "@lucide/svelte/icons/x";
  import type {
    StoreRepositorySummary,
    StoreController,
  } from "../../controller/contracts.js";
  import type { WorkflowDefinition } from "../../controller/workflow-types.js";
  import "../store-settings/StoreSettings.css";
  let {
    id,
    definition = $bindable(),
    selection,
    repositories,
    authors = [],
    acquireRepository,
    hidden = false,
    disabled = false,
    invalid = $bindable(false),
    onInvalid,
    onValid,
    onClose,
    canRemove = false,
    onRemove,
  }: {
    id: string;
    definition: WorkflowDefinition;
    selection: number;
    hidden?: boolean;
    repositories: readonly StoreRepositorySummary[];
    authors?: readonly { id: string; name: string }[];
    acquireRepository?: StoreController["acquireRepository"];
    disabled?: boolean;
    invalid?: boolean;
    onInvalid?(message: string): void;
    onValid?(): void;
    onClose(): void;
    canRemove?: boolean;
    onRemove(): void;
  } = $props();
  const action = $derived(
    selection >= 0 ? definition.actions[selection] : undefined,
  );
</script>

<aside {hidden} {id} class="workflow-inspector" aria-label="Workflow inspector">
  <header class="workflow-inspector__header">
    <h3>{action ? `Step ${selection + 1}` : "Trigger"}</h3>
    <div class="workflow-inspector__controls">
      {#if action}
        <Button
          variant="ghost"
          size="icon-sm"
          class="workflow-step__control workflow-step__control--delete"
          aria-label="Delete selected action"
          title="Delete action"
          disabled={disabled || !action || !canRemove}
          onclick={onRemove}><Trash aria-hidden="true" /></Button
        >
      {/if}
      <Button
        variant="ghost"
        size="icon-sm"
        aria-label="Close inspector"
        title="Hide inspector"
        onclick={onClose}><X aria-hidden="true" /></Button
      >
    </div>
  </header>
  <div class="workflow-inspector__form">
    <Field.Group class="store-settings__editable-rows">
      {#if !action}
        <Field.Field orientation="horizontal" class="store-settings__option-row"
          ><Field.Content
            ><Field.Label for={`${id}-repository`}
              ><StoreSettingsIcon Icon={FolderGit} />Source repository</Field.Label
            ></Field.Content
          ><StoreRepositoryPicker
            id={`${id}-repository`}
            label="Source repository"
            {repositories}
            bind:value={definition.repository}
            {disabled}
          /></Field.Field
        >
        <Field.Field orientation="horizontal" class="store-settings__option-row"
          ><Field.Content
            ><Field.Label for={`${id}-trigger`}
              ><StoreSettingsIcon Icon={GitBranch} />When</Field.Label
            ></Field.Content
          ><WorkflowTriggerPicker
            {id}
            bind:definition
            {disabled}
          /></Field.Field
        >
        {#if definition.trigger.on === "bookmark_changed"}<Field.Field
            orientation="horizontal"
            class="store-settings__option-row"
            ><Field.Content
              ><Field.Label for={`${id}-bookmark`}
                ><StoreSettingsIcon Icon={GitBranch} />Bookmark pattern</Field.Label
              ></Field.Content
            ><Input
              id={`${id}-bookmark`}
              bind:value={definition.trigger.bookmark}
              placeholder="e.g. main or release/*"
              {disabled}
            /></Field.Field
          >{/if}
        {#key definition.trigger.on}<WorkflowConditions
            {acquireRepository}
            {id}
            bind:definition
            {authors}
            {disabled}
          />{/key}
        <Field.Field
          orientation="horizontal"
          class="store-settings__option-row workflow-inspector-row--wide"
          ><Field.Content
            ><Field.Label for={`${id}-labels`}
              ><StoreSettingsIcon Icon={Tags} />Runner labels</Field.Label
            ><Field.Description
              >All labels must match an admitted host.</Field.Description
            ></Field.Content
          >
          <fieldset {disabled} class="workflow-labels">
            <ChipAutocomplete
              id={`${id}-labels`}
              label="Runner labels"
              showLabel={false}
              uppercase={false}
              placeholder="Add runner label…"
              value={definition.labels ?? []}
              onChange={(items) => {
                if (!disabled) definition.labels = items;
              }}
            />
          </fieldset>
        </Field.Field>
      {:else}
        <Field.Field orientation="horizontal" class="store-settings__option-row"
          ><Field.Content
            ><Field.Label for={`${id}-action`}
              ><StoreSettingsIcon Icon={TextCursor} />Action name</Field.Label
            ></Field.Content
          ><Input
            id={`${id}-action`}
            bind:value={action.id}
            {disabled}
          /></Field.Field
        >
        <Field.Field orientation="horizontal" class="store-settings__option-row"
          ><Field.Content
            ><Field.Label for={`${id}-type`}
              ><StoreSettingsIcon Icon={Terminal} />Executor</Field.Label
            ></Field.Content
          ><WorkflowSelect
            id={`${id}-type`}
            label="Executor"
            value={action.executor.type}
            {disabled}
            options={[
              { value: "command", label: "Named command", Icon: Terminal },
              { value: "agent", label: "Agent", Icon: Bot },
            ]}
            onChange={(value) =>
              (action.executor =
                value === "agent"
                  ? { type: "agent", agent: "", prompt: "" }
                  : { type: "command", name: "build" })}
          /></Field.Field
        >
        {#if action.executor.type === "command"}<Field.Field
            orientation="horizontal"
            class="store-settings__option-row"
            ><Field.Content
              ><Field.Label for={`${id}-command`}
                ><StoreSettingsIcon Icon={Terminal} />Command name</Field.Label
              ><Field.Description
                >Configured on the execution host.</Field.Description
              ></Field.Content
            ><Input
              id={`${id}-command`}
              bind:value={action.executor.name}
              {disabled}
            /></Field.Field
          >
        {:else}<Field.Field
            orientation="horizontal"
            class="store-settings__option-row"
            ><Field.Content
              ><Field.Label for={`${id}-agent`}
                ><StoreSettingsIcon Icon={Bot} />Agent</Field.Label
              ></Field.Content
            ><Input
              id={`${id}-agent`}
              bind:value={action.executor.agent}
              {disabled}
            /></Field.Field
          ><Field.Field
            orientation="horizontal"
            class="store-settings__option-row workflow-inspector-row--wide"
            ><Field.Content
              ><Field.Label for={`${id}-prompt`}
                ><StoreSettingsIcon Icon={Message} />Prompt</Field.Label
              ></Field.Content
            ><CommunityChatComposer
              ariaLabel="Prompt"
              bind:value={action.executor.prompt}
              mode="form"
              {disabled}
            /></Field.Field
          >{/if}
        <StoreNumericSettingRow
          id={`${id}-timeout`}
          label="Timeout"
          description="Maximum time for this action."
          value={String(action.timeout_seconds ?? 300)}
          units={secondsUnits}
          min={1}
          max={86400}
          defaultValue={300}
          slider
          sliderMax={3600}
          sliderStep={30}
          density="compact"
          {disabled}
          onValueChange={(value) => (action.timeout_seconds = Number(value))}
        />
        <Field.Field
          orientation="horizontal"
          class="store-settings__option-row workflow-inspector-row--wide"
          ><Field.Content
            ><Field.Label for={`${id}-inputs`}
              ><StoreSettingsIcon Icon={Braces} />Inputs (JSON)</Field.Label
            ></Field.Content
          ><Textarea
            id={`${id}-inputs`}
            value={JSON.stringify(action.inputs ?? {}, null, 2)}
            {disabled}
            aria-invalid={invalid || undefined}
            onchange={(event) => {
              try {
                const parsed: unknown = JSON.parse(event.currentTarget.value);
                if (
                  parsed &&
                  typeof parsed === "object" &&
                  !Array.isArray(parsed) &&
                  Object.values(parsed).every((v) => typeof v === "string")
                ) {
                  action.inputs = parsed as Record<string, string>;
                  invalid = false;
                  onValid?.();
                  event.currentTarget.setCustomValidity("");
                } else throw Error("Invalid inputs");
              } catch {
                invalid = true;
                onInvalid?.("Inputs must be a JSON object of strings");
                event.currentTarget.setCustomValidity(
                  "Inputs must be a JSON object of strings",
                );
              }
            }}
          /></Field.Field
        >
      {/if}
    </Field.Group>
  </div>
</aside>
