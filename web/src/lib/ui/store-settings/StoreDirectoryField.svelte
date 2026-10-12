<script lang="ts">
  import type { Component, Snippet } from "svelte";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import StoreSettingsIcon from "./StoreSettingsIcon.svelte";
  let {
    label,
    id,
    description,
    Icon,
    children,
  }: {
    label: string;
    id?: string;
    description?: string;
    Icon: Component;
    children: Snippet;
  } = $props();
</script>

<Field.Field
  orientation="horizontal"
  class="store-settings__option-row store-directory-field"
>
  <div class="store-directory-field__label">
    <StoreSettingsIcon {Icon} />
    <Field.Content>
      {#if id}<Field.Label for={id}>{label}</Field.Label>{:else}<Field.Title
          >{label}</Field.Title
        >{/if}
      {#if description}<Field.Description>{description}</Field.Description>{/if}
    </Field.Content>
  </div>
  <div class="store-directory-field__control">{@render children()}</div>
</Field.Field>
