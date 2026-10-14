<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Menu from "@lapismd/design-core/shadcn/dropdown-menu";
  import More from "@lucide/svelte/icons/ellipsis";
  import Pencil from "@lucide/svelte/icons/pencil";
  import Play from "@lucide/svelte/icons/play";
  import Copy from "@lucide/svelte/icons/copy";
  import Trash from "@lucide/svelte/icons/trash-2";
  import Power from "@lucide/svelte/icons/power";
  import type { WorkflowVersion } from "../../controller/workflow-types.js";
  let {
    workflow,
    busy = false,
    writable = true,
    onOpen,
    onToggle,
    onDuplicate,
    onRemove,
    onRun,
  }: {
    workflow: WorkflowVersion;
    busy?: boolean;
    writable?: boolean;
    onOpen(): void;
    onToggle(): void;
    onDuplicate(): void;
    onRemove(): void;
    onRun(): void;
  } = $props();
  let open = $state(false);
  function select(action: () => void) {
    open = false;
    action();
  }
</script>

<Menu.Root bind:open>
  <Menu.Trigger
    >{#snippet child({ props })}<Button
        {...props}
        variant="ghost"
        size="icon-sm"
        aria-label={`Actions for ${workflow.definition.name}`}
        ><More aria-hidden="true" /></Button
      >{/snippet}</Menu.Trigger
  >
  <Menu.Content align="end" class="workflow-actions-menu">
    <Menu.Group>
      <Menu.Item onclick={() => select(onOpen)}
        ><Pencil aria-hidden="true" />Edit workflow</Menu.Item
      >
      <Menu.Item disabled={!writable || busy} onclick={() => select(onRun)}
        ><Play aria-hidden="true" />Run workflow</Menu.Item
      >
      <Menu.Item
        disabled={!writable || busy}
        onclick={() => select(onDuplicate)}
        ><Copy aria-hidden="true" />Duplicate</Menu.Item
      >
      <Menu.Item disabled={!writable || busy} onclick={() => select(onToggle)}
        ><Power aria-hidden="true" />{workflow.definition.enabled
          ? "Disable workflow"
          : "Enable workflow"}</Menu.Item
      >
    </Menu.Group>
    <Menu.Separator />
    <Menu.Group>
      <Menu.Item
        disabled={!writable || busy}
        variant="destructive"
        onclick={() => select(onRemove)}
        ><Trash aria-hidden="true" />Remove workflow</Menu.Item
      >
    </Menu.Group>
  </Menu.Content>
</Menu.Root>

<style>
  :global(.workflow-actions-menu[data-slot="dropdown-menu-content"]) {
    min-width: min(15rem, calc(100vw - 2rem));
    max-width: calc(100vw - 2rem);
  }
  :global(.workflow-actions-menu [data-slot="dropdown-menu-item"]) {
    white-space: nowrap;
  }
</style>
