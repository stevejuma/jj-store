<script lang="ts">
  import { tick } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import Workflow from "@lucide/svelte/icons/workflow";
  import Pencil from "@lucide/svelte/icons/pencil";
  import Check from "@lucide/svelte/icons/check";
  let {
    value = $bindable(),
    disabled = false,
  }: { value: string; disabled?: boolean } = $props();
  let editing = $state(false),
    original = "",
    input = $state<HTMLInputElement | null>(null);
  async function edit() {
    original = value;
    editing = true;
    await tick();
    input?.focus();
    input?.select();
  }
</script>

<div class="workflow-editor-name" data-editing={editing}>
  <Workflow aria-hidden="true" />
  {#if editing}
    <Input
      bind:ref={input}
      bind:value
      aria-label="Name"
      {disabled}
      maxlength={256}
      onkeydown={(event) => {
        if (event.key === "Enter") {
          event.preventDefault();
          editing = false;
        }
        if (event.key === "Escape") {
          event.preventDefault();
          event.stopPropagation();
          value = original;
          editing = false;
        }
      }}
    />
  {:else}<span title={value}>{value || "Untitled workflow"}</span>{/if}
  <Button
    variant="ghost"
    size="icon-sm"
    class="workflow-editor-name__action"
    aria-label={editing ? "Save workflow name" : "Edit workflow name"}
    {disabled}
    onclick={() => (editing ? (editing = false) : edit())}
  >
    {#if editing}<Check aria-hidden="true" />{:else}<Pencil
        aria-hidden="true"
      />{/if}
  </Button>
</div>

<style>
  .workflow-editor-name {
    position: absolute;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    align-items: center;
    gap: 0.5rem;
    max-width: min(24rem, calc(100% - 12rem));
    min-height: 2.125rem;
    padding: 0 0.125rem 0 0.5rem;
    border: 0;
    border-radius: 999px;
    background: transparent;
  }
  .workflow-editor-name > :global(svg) {
    width: 1rem;
    height: 1rem;
    flex: none;
    color: var(--muted-foreground);
  }
  .workflow-editor-name > span {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 0.875rem;
    font-weight: 600;
  }
  .workflow-editor-name :global(input) {
    width: 15rem;
    min-width: 0;
    height: 2rem;
    border: 0;
    box-shadow: none;
    background: transparent;
  }
  .workflow-editor-name :global(input:focus-visible) {
    outline: none;
    box-shadow: none;
  }
  .workflow-editor-name[data-editing="true"]:focus-within {
    border-color: transparent;
    box-shadow: 0 0 0 3px
      color-mix(
        in oklab,
        var(--ui-input-focus-ring-color, var(--ring)) 50%,
        transparent
      );
  }
  .workflow-editor-name :global(.workflow-editor-name__action) {
    flex: none;
    border-radius: 50%;
  }
  .workflow-editor-name
    :global(
      .workflow-editor-name__action:is(:hover, :focus-visible):not(:disabled)
    ) {
    background: var(--background);
    color: var(--foreground);
  }
</style>
