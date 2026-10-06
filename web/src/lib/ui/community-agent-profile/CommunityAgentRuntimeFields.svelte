<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import PencilIcon from "@lucide/svelte/icons/pencil";
  let {
    harness,
    model,
    thinking,
    mode,
    disabled = false,
    harnessEditable = true,
    onEdit,
  }: {
    harness: string;
    model: string;
    thinking: string;
    mode: string;
    disabled?: boolean;
    harnessEditable?: boolean;
    onEdit: () => void;
  } = $props();
</script>

<div
  class="agent-profile__runtime-fields"
  data-ui-part="runtime-defaults"
  aria-label="Runtime defaults"
>
  {#each [{ label: "Agent harness", value: harness }, { label: "Model", value: model }, { label: "Thinking", value: thinking }, { label: "Execution mode", value: mode }] as field (field.label)}
    <Button
      type="button"
      variant="ghost"
      class="agent-profile__runtime-field"
      disabled={disabled ||
        (field.label === "Agent harness" && !harnessEditable)}
      aria-label={`Edit ${field.label.toLowerCase()}`}
      onclick={onEdit}
    >
      <span><span>{field.label}</span><strong>{field.value}</strong></span
      ><PencilIcon aria-hidden="true" />
    </Button>
  {/each}
</div>
