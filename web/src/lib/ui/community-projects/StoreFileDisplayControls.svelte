<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as ButtonGroup from "@lapismd/design-core/shadcn/button-group";
  import CodeXmlIcon from "@lucide/svelte/icons/code-xml";
  import EyeIcon from "@lucide/svelte/icons/eye";
  import PencilIcon from "@lucide/svelte/icons/pencil";
  import HistoryIcon from "@lucide/svelte/icons/history";
  const descriptionId = $props.id();
  let {
    mode,
    isMarkdown,
    localEditing,
    editDisabled,
    editTitle,
    blameReason,
    onSelect,
    onEdit,
  }: {
    mode: "source" | "preview" | "edit" | "blame";
    isMarkdown: boolean;
    localEditing: boolean;
    editDisabled: boolean;
    editTitle: string;
    blameReason: string;
    onSelect(mode: "source" | "preview" | "blame"): void;
    onEdit(): void;
  } = $props();
</script>

<ButtonGroup.Root
  class="community-projects__file-view-toggle"
  aria-label="File display"
>
  {#if isMarkdown}<Button
      type="button"
      variant="outline"
      size="sm"
      data-selected={mode === "preview"}
      aria-pressed={mode === "preview"}
      onclick={() => onSelect("preview")}
    >
      <EyeIcon data-icon="inline-start" aria-hidden="true" />Preview
    </Button>{/if}
  <Button
    type="button"
    variant="outline"
    size="sm"
    data-selected={mode === "source"}
    aria-pressed={mode === "source"}
    onclick={() => onSelect("source")}
  >
    <CodeXmlIcon data-icon="inline-start" aria-hidden="true" />Source
  </Button>
  <Button
    type="button"
    variant="outline"
    size="sm"
    data-selected={mode === "blame"}
    aria-pressed={mode === "blame"}
    disabled={Boolean(blameReason)}
    title={blameReason || "Show line origins"}
    aria-describedby={blameReason ? descriptionId : undefined}
    onclick={() => onSelect("blame")}
  >
    <HistoryIcon data-icon="inline-start" aria-hidden="true" />Blame
  </Button>
  <Button
    type="button"
    variant="outline"
    size="sm"
    data-selected={mode === "edit"}
    aria-pressed={mode === "edit"}
    disabled={editDisabled}
    title={editTitle}
    onclick={onEdit}
  >
    <PencilIcon data-icon="inline-start" aria-hidden="true" />{localEditing
      ? "Working copy"
      : "Edit"}
  </Button>
</ButtonGroup.Root>

{#if blameReason}<span id={descriptionId} class="sr-only">{blameReason}</span
  >{/if}
