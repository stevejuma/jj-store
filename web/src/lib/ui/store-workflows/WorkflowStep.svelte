<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import GitBranch from "@lucide/svelte/icons/git-branch";
  import ArrowUp from "@lucide/svelte/icons/arrow-up";
  import ArrowDown from "@lucide/svelte/icons/arrow-down";
  import Trash from "@lucide/svelte/icons/trash-2";
  let {
    label,
    description,
    index,
    selected,
    expanded,
    inspectorId,
    disabled = false,
    canRemove = false,
    canMoveUp = false,
    canMoveDown = false,
    onSelect,
    onRemove,
    onMoveUp,
    onMoveDown,
  }: {
    label: string;
    description: string;
    index?: number;
    selected: boolean;
    expanded: boolean;
    inspectorId: string;
    disabled?: boolean;
    canRemove?: boolean;
    canMoveUp?: boolean;
    canMoveDown?: boolean;
    onSelect(): void;
    onRemove?(): void;
    onMoveUp?(): void;
    onMoveDown?(): void;
  } = $props();
</script>

<div
  class="workflow-step"
  class:workflow-step--action={index !== undefined}
  data-selected={selected}
>
  {#if index !== undefined}
    <div class="workflow-step__controls workflow-step__reorder">
      <Button
        variant="ghost"
        size="icon-sm"
        class="workflow-step__control"
        aria-label={`Move ${label} up`}
        title="Move up"
        disabled={disabled || !canMoveUp}
        onclick={onMoveUp}
      >
        <ArrowUp aria-hidden="true" />
      </Button>
      <Button
        variant="ghost"
        size="icon-sm"
        class="workflow-step__control"
        aria-label={`Move ${label} down`}
        title="Move down"
        disabled={disabled || !canMoveDown}
        onclick={onMoveDown}
      >
        <ArrowDown aria-hidden="true" />
      </Button>
    </div>
  {/if}
  <button
    class="workflow-node"
    aria-pressed={selected}
    aria-expanded={expanded}
    aria-controls={expanded ? inspectorId : undefined}
    onclick={onSelect}
  >
    <span class="workflow-node__icon" aria-hidden="true">
      {#if index === undefined}<GitBranch />{:else}{index + 1}{/if}
    </span>
    <span class="workflow-node__copy"
      ><strong>{label}</strong><span>{description}</span></span
    >
  </button>
  {#if index !== undefined}
    <div class="workflow-step__controls workflow-step__delete">
      <Button
        variant="ghost"
        size="icon"
        class="workflow-step__control workflow-step__control--delete"
        aria-label={`Delete ${label}`}
        title={canRemove ? "Delete action" : "Keep at least one action"}
        disabled={disabled || !canRemove}
        onclick={onRemove}
      >
        <Trash aria-hidden="true" />
      </Button>
    </div>
  {/if}
</div>
