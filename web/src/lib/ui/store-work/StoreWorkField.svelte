<script lang="ts">
  import "./community-work-forms.css";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import type { Snippet } from "svelte";
  let {
    label,
    id,
    description = "",
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
    disabled?: boolean;
    compact?: boolean;
    bordered?: boolean;
    hideLabel?: boolean;
    class?: string;
    children: Snippet;
  } = $props();
</script>

<Field.Field
  orientation={compact ? "vertical" : "responsive"}
  data-disabled={disabled || undefined}
  class={[
    "store-work__field community-settings__option-row",
    bordered && "store-work__field--bordered",
    className,
  ]}
>
  <Field.Content class={hideLabel ? "sr-only" : undefined}>
    {#if id}<Field.Label for={id}>{label}</Field.Label>{:else}<Field.Title
        >{label}</Field.Title
      >{/if}
    {#if description}<Field.Description>{description}</Field.Description>{/if}
  </Field.Content>
  <div class="store-work__control">{@render children()}</div>
</Field.Field>
