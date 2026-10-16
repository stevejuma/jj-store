<script lang="ts">
  import "./community-work-forms.css";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import type { Component, Snippet } from "svelte";
  import StoreSettingsIcon from "../store-settings/StoreSettingsIcon.svelte";
  let {
    label,
    id,
    description = "",
    Icon,
    disabled = false,
    compact = false,
    bordered = true,
    hideLabel = false,
    class: className = "",
    children,
  }: {
    label: string;
    id?: string;
    description?: string;
    Icon?: Component;
    disabled?: boolean;
    compact?: boolean;
    bordered?: boolean;
    hideLabel?: boolean;
    class?: string;
    children: Snippet;
  } = $props();
</script>

{#snippet labelContent()}<Field.Content
    class={hideLabel ? "sr-only" : undefined}
  >
    {#if id}<Field.Label id={`${id}-label`} for={id}>{label}</Field.Label
      >{:else}<Field.Title>{label}</Field.Title>{/if}
    {#if description}<Field.Description>{description}</Field.Description>{/if}
  </Field.Content>{/snippet}

<Field.Field
  orientation={compact ? "vertical" : "responsive"}
  data-disabled={disabled || undefined}
  class={[
    "store-work__field community-settings__option-row",
    bordered && "store-work__field--bordered",
    className,
  ]}
>
  {#if Icon}<div class="store-work__field-label">
      <StoreSettingsIcon {Icon} />
      {@render labelContent()}
    </div>{:else}{@render labelContent()}{/if}
  <div class="store-work__control">{@render children()}</div>
</Field.Field>

<style>
  .store-work__field-label {
    display: flex;
    align-items: center;
    min-width: 0;
    gap: 0.6rem;
  }
</style>
