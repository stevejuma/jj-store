<script lang="ts">
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
</script>

{#each searchAdvancedFields as field}
  <Field.Field orientation="responsive" class="store-settings__option-row">
    <Field.Content
      ><Field.Label for={`search-${field.key}`}>{field.label}</Field.Label
      ><Field.Description
        >Currently active: {effective?.[field.key]?.toLocaleString("en-GB") ??
          field.default.toLocaleString("en-GB")}{field.key === "snapshot_every"
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
{/each}
