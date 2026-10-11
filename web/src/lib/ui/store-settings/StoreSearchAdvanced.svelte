<script lang="ts">
  import StoreNumericSettingRow from "./StoreNumericSettingRow.svelte";
  import { searchNumericFields } from "./search-numeric-fields.js";
  import { settingsPresentation } from "./settings-presentation.js";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { searchAdvancedFields } from "../../controller/search-advanced.js";
  import type {
    SearchDraft,
    SearchConfiguration,
  } from "../../controller/search-settings.js";
  let {
    draft,
    effective,
    disabled,
    onDraft,
  }: {
    draft: SearchDraft;
    effective?: SearchConfiguration;
    disabled: boolean;
    onDraft: (patch: Partial<SearchDraft>) => void;
  } = $props();
  const presentation = settingsPresentation();
</script>

{#each searchAdvancedFields as field}
  {#if presentation.value === "repository"}
    {@const numeric = searchNumericFields[field.key]}
    <StoreNumericSettingRow
      id={`search-${field.key}`}
      label={numeric.label}
      description={`${numeric.description} Currently active: ${(effective?.[field.key] ?? field.default).toLocaleString("en-GB")} ${numeric.units[0].label}.`}
      units={numeric.units}
      value={draft[field.key]}
      min={field.min}
      max={field.max}
      defaultValue={field.default}
      slider={field.key === "timeout_seconds"}
      {disabled}
      onValueChange={(value) => onDraft({ [field.key]: value })}
    />
  {:else}
    <Field.Field orientation="responsive" class="store-settings__option-row">
      <Field.Content
        ><Field.Label for={`search-${field.key}`}>{field.label}</Field.Label
        ><Field.Description
          >Currently active: {effective?.[field.key]?.toLocaleString("en-GB") ??
            field.default.toLocaleString("en-GB")}{field.key ===
          "snapshot_every"
            ? ". Zero disables optional remote snapshots."
            : ""}</Field.Description
        ></Field.Content
      >
      <Input
        class="store-settings__input"
        id={`search-${field.key}`}
        type="number"
        min={field.min}
        max={field.max}
        step="1"
        required
        {disabled}
        value={draft[field.key]}
        oninput={(event) => onDraft({ [field.key]: event.currentTarget.value })}
      />
    </Field.Field>
  {/if}
{/each}
