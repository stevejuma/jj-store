<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import Shield from "@lucide/svelte/icons/shield";
  import StoreSettingsDisclosure from "./StoreSettingsDisclosure.svelte";
  import StoreSettingsIcon from "./StoreSettingsIcon.svelte";
  import WorkflowHostPrincipalPicker from "./WorkflowHostPrincipalPicker.svelte";
  let {
    admitted,
    principal,
    authors,
    disabled,
    onChange,
  }: {
    admitted: string[];
    principal?: string | null;
    authors: readonly { id: string; name: string }[];
    disabled: boolean;
    onChange(values: string[]): void;
  } = $props();
</script>

<StoreSettingsDisclosure
  label="Execution access"
  description="Explicit host admission, separate from repository access"
  inset={false}
>
  <Field.Group class="store-settings__editable-rows">
    <Field.Field orientation="responsive" class="store-settings__option-row">
      <Field.Content
        ><Field.Label for="workflow-admission"
          ><StoreSettingsIcon Icon={Shield} />Admitted execution principals</Field.Label
        ><Field.Description
          >Save and reload to apply. Discovery and host enablement do not grant
          admission.</Field.Description
        ></Field.Content
      >
      <WorkflowHostPrincipalPicker
        id="workflow-admission"
        label="Admitted execution principals"
        values={admitted}
        authors={[
          ...authors,
          ...(principal ? [{ id: principal, name: "Local host" }] : []),
        ]}
        {disabled}
        {onChange}
      />
    </Field.Field>
    {#if principal}<Field.Field
        orientation="responsive"
        class="store-settings__option-row"
      >
        <Field.Content
          ><Field.Title
            ><StoreSettingsIcon Icon={Shield} />Local host identity</Field.Title
          ><Field.Description>{principal}</Field.Description></Field.Content
        >
        <Button
          type="button"
          variant="outline"
          disabled={disabled || admitted.includes(principal)}
          onclick={() => onChange([...admitted, principal])}
          ><Shield aria-hidden="true" />{admitted.includes(principal)
            ? "Local host admitted"
            : "Admit local host"}</Button
        >
      </Field.Field>{/if}
  </Field.Group>
</StoreSettingsDisclosure>
