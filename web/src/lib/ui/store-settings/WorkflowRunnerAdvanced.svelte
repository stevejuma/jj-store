<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import StoreSettingsDisclosure from "./StoreSettingsDisclosure.svelte";
  import StoreSettingsIcon from "./StoreSettingsIcon.svelte";
  import WorkflowHostPrincipalPicker from "./WorkflowHostPrincipalPicker.svelte";
  import StoreNumericSettingNumberRow from "./StoreNumericSettingNumberRow.svelte";
  import { millisecondsUnits } from "./numeric-setting.js";
  import Folder from "@lucide/svelte/icons/folder";
  import Terminal from "@lucide/svelte/icons/terminal";
  import Shield from "@lucide/svelte/icons/shield";
  import type { RunnerEnvironment } from "../../controller/workflow-runner-environment.js";
  let {
    configuration,
    authors,
    disabled,
    onChange,
  }: {
    configuration: RunnerEnvironment;
    authors: readonly { id: string; name: string }[];
    disabled: boolean;
    onChange(key: keyof RunnerEnvironment, value: unknown): void;
  } = $props();
</script>

<StoreSettingsDisclosure
  label="Advanced runner settings"
  description="Optional paths, polling and owner policy"
  inset={false}
>
  <Field.Group class="store-settings__editable-rows">
    {#each [{ key: "state_dir", label: "Recovery state directory", placeholder: "Automatic private host directory", help: "Keep the journal for exact outcome recovery.", Icon: Folder }, { key: "jj_executable", label: "JJ executable", placeholder: "Paired custom JJ CLI", help: "Leave empty to use the CLI shipped with this server.", Icon: Terminal }] as field}
      <Field.Field orientation="responsive" class="store-settings__option-row">
        <Field.Content
          ><Field.Label for={`workflow-${field.key}`}
            ><StoreSettingsIcon Icon={field.Icon} />{field.label}</Field.Label
          ><Field.Description>{field.help}</Field.Description></Field.Content
        >
        <Input
          id={`workflow-${field.key}`}
          class="store-settings__input"
          value={configuration[field.key as "state_dir"] ?? ""}
          placeholder={field.placeholder}
          {disabled}
          oninput={(e) =>
            onChange(field.key as "state_dir", e.currentTarget.value || null)}
        />
      </Field.Field>
    {/each}
    <StoreNumericSettingNumberRow
      id="workflow-poll"
      label="Polling interval"
      description="Default: one second. Does not change job scope."
      min={100}
      max={60000}
      slider
      sliderStep={100}
      value={configuration.poll_millis ?? 1000}
      units={millisecondsUnits}
      {disabled}
      onValueChange={(n) => onChange("poll_millis", n)}
    />
    <Field.Field orientation="responsive" class="store-settings__option-row">
      <Field.Content
        ><Field.Label for="workflow-allowed_owners"
          ><StoreSettingsIcon Icon={Shield} />Allowed workflow owners</Field.Label
        ><Field.Description
          >Empty uses this installation's local service identities.</Field.Description
        ></Field.Content
      >
      <WorkflowHostPrincipalPicker
        id="workflow-allowed_owners"
        label="Allowed workflow owners"
        values={configuration.allowed_owners ?? []}
        {authors}
        {disabled}
        onChange={(v) => onChange("allowed_owners", v)}
      />
    </Field.Field>
  </Field.Group>
</StoreSettingsDisclosure>
