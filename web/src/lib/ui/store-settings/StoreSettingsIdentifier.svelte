<script lang="ts">
  import * as ButtonGroup from "@lapismd/design-core/shadcn/button-group";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import CopyIcon from "@lucide/svelte/icons/copy";
  let {
    label,
    value,
    emptyLabel = "Unavailable",
  }: {
    label: string;
    value?: string | null;
    emptyLabel?: string;
  } = $props();
  let feedback = $state<{ value: string; text: string } | null>(null);
  async function copy() {
    if (!value) return;
    const copiedValue = value;
    try {
      await navigator.clipboard.writeText(copiedValue);
      feedback = { value: copiedValue, text: `${label} copied` };
    } catch {
      feedback = {
        value: copiedValue,
        text: "Could not copy. Select the value and copy it manually.",
      };
    }
  }
</script>

{#if value}
  <div class="store-settings__identifier">
    <ButtonGroup.Root
      class="store-settings__field-action"
      style="width: 100%"
      aria-label={label}
    >
      <Input {value} readonly aria-label={label} spellcheck={false} />
      <Button
        type="button"
        variant="outline"
        aria-label={`Copy ${label}`}
        onclick={() => void copy()}
      >
        <CopyIcon data-icon="inline-start" aria-hidden="true" />Copy
      </Button>
    </ButtonGroup.Root>
    <span role="status">{feedback?.value === value ? feedback.text : ""}</span>
  </div>
{:else}
  <span>{emptyLabel}</span>
{/if}
