<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Menu from "@lapismd/design-core/shadcn/dropdown-menu";
  import More from "@lucide/svelte/icons/ellipsis";
  import Play from "@lucide/svelte/icons/play";
  import Pencil from "@lucide/svelte/icons/pencil";
  import Copy from "@lucide/svelte/icons/copy";
  import Power from "@lucide/svelte/icons/power";
  import Trash from "@lucide/svelte/icons/trash-2";
  let {
    enabled,
    disabled = false,
    toggleDisabled = false,
    onEnabled,
    onEdit,
    onDuplicate,
    onTrigger,
    onDelete,
  }: {
    enabled: boolean;
    disabled?: boolean;
    toggleDisabled?: boolean;
    onEnabled(value: boolean): void;
    onEdit(): void;
    onDuplicate(): void;
    onTrigger?(): void;
    onDelete?(): void;
  } = $props();
</script>

<Menu.Root>
  <Menu.Trigger>
    {#snippet child({ props })}<Button
        {...props}
        variant="ghost"
        size="icon-sm"
        aria-label="Workflow actions"><More aria-hidden="true" /></Button
      >{/snippet}
  </Menu.Trigger>
  <Menu.Content align="end" class="workflow-editor-menu">
    <Menu.Group>
      <Menu.Item disabled={disabled || !onTrigger} onclick={onTrigger}
        ><Play aria-hidden="true" />Trigger</Menu.Item
      >
      <Menu.Item {disabled} onclick={onEdit}
        ><Pencil aria-hidden="true" />Edit</Menu.Item
      >
      <Menu.Item {disabled} onclick={onDuplicate}
        ><Copy aria-hidden="true" />Duplicate</Menu.Item
      >
      <Menu.CheckboxItem
        class="workflow-editor-menu__enable"
        checked={enabled}
        onCheckedChange={onEnabled}
        closeOnSelect={false}
        disabled={disabled || toggleDisabled}
      >
        <Power aria-hidden="true" /><span>Enable</span>
        <span
          class="workflow-editor-menu__toggle"
          data-enabled={enabled}
          aria-hidden="true"><span></span></span
        >
      </Menu.CheckboxItem>
      <Menu.Separator />
      <Menu.Item
        variant="destructive"
        disabled={disabled || !onDelete}
        onclick={onDelete}><Trash aria-hidden="true" />Delete</Menu.Item
      >
    </Menu.Group>
  </Menu.Content>
</Menu.Root>

<style>
  :global(.workflow-editor-menu) {
    min-width: 14rem;
  }
  :global(
    .workflow-editor-menu__enable[data-slot="dropdown-menu-checkbox-item"]
  ) {
    padding-left: 0.5rem;
  }
  :global(
    .workflow-editor-menu[data-slot="dropdown-menu-content"]
      .workflow-editor-menu__enable
      > [data-ui-component="dropdown-menu"][data-slot="dropdown-menu-checkbox-item-indicator"]
  ) {
    display: none;
  }
  .workflow-editor-menu__toggle {
    display: flex;
    align-items: center;
    width: 2.25rem;
    height: 1.25rem;
    margin-left: auto;
    padding: 2px;
    border-radius: 999px;
    background: var(--input);
  }
  .workflow-editor-menu__toggle > span {
    width: 1rem;
    height: 1rem;
    border-radius: 50%;
    background: var(--background);
    box-shadow: 0 1px 2px
      color-mix(in oklab, var(--foreground) 15%, transparent);
  }
  .workflow-editor-menu__toggle[data-enabled="true"] {
    justify-content: flex-end;
    background: var(--primary);
  }
</style>
