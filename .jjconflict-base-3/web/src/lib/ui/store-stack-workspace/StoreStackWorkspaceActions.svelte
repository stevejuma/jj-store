<script lang="ts">
  import * as Menu from "@lapismd/design-core/shadcn/dropdown-menu";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { mergeProps } from "bits-ui";
  import Plus from "@lucide/svelte/icons/plus";
  import Trash from "@lucide/svelte/icons/trash-2";
  import Ellipsis from "@lucide/svelte/icons/ellipsis";
  import GitBranchPlus from "@lucide/svelte/icons/git-branch-plus";
  import Undo from "@lucide/svelte/icons/undo-2";
  import Refresh from "@lucide/svelte/icons/refresh-cw";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import type { WorkspaceCommand } from "../../controller/stack-workspace/types.js";
  let {
    controller,
    command,
  }: {
    controller: StackWorkspaceController;
    command: (value: WorkspaceCommand) => void;
  } = $props();
  const snapshot = $derived(controller.snapshot);
  const eligible = $derived(
    snapshot?.availability === "ready" && Boolean(snapshot.folder),
  );
</script>

<div
  class="stack-workspace__controls"
  role="group"
  aria-label="Stack workspace controls"
>
  <Tooltip.Provider>
    {#if snapshot}
      <Menu.Root>
        <Tooltip.Root>
          <Tooltip.Trigger>
            {#snippet child({ props: tooltipProps })}
              <Menu.Trigger>
                {#snippet child({ props })}
                  <Button
                    {...mergeProps(tooltipProps, props)}
                    variant="ghost"
                    size="icon-sm"
                    aria-label="Workspace actions"
                    disabled={controller.locked || !eligible}
                  >
                    <Ellipsis aria-hidden="true" />
                  </Button>
                {/snippet}
              </Menu.Trigger>
            {/snippet}
          </Tooltip.Trigger>
          <Tooltip.Content>Workspace actions</Tooltip.Content>
        </Tooltip.Root>
        <Menu.Content
          align="end"
          preventScroll={false}
          class="stack-workspace__action-menu"
        >
          <Menu.Group>
            <Menu.Item
              onSelect={() => command({ kind: "create_workspace", name: "" })}
              ><Plus aria-hidden="true" />Create workspace</Menu.Item
            >
            {#each snapshot.workspaces.filter((item) => item.id !== snapshot.active) as item (item.id)}
              <Menu.Item
                onSelect={() =>
                  command({ kind: "delete_workspace", workspace: item.id })}
                ><Trash aria-hidden="true" />Delete {item.name}</Menu.Item
              >
            {/each}
          </Menu.Group>
        </Menu.Content>
      </Menu.Root>
      <Tooltip.Root>
        <Tooltip.Trigger>
          {#snippet child({ props })}
            <Button
              {...props}
              variant="ghost"
              size="icon-sm"
              aria-label="Create stack"
              disabled={controller.locked || !eligible}
              onclick={() => command({ kind: "create_stack", name: "" })}
            >
              <GitBranchPlus aria-hidden="true" />
            </Button>
          {/snippet}
        </Tooltip.Trigger>
        <Tooltip.Content>Create stack</Tooltip.Content>
      </Tooltip.Root>
      <Tooltip.Root>
        <Tooltip.Trigger>
          {#snippet child({ props })}
            <Button
              {...props}
              variant="ghost"
              size="icon-sm"
              aria-label="Undo"
              disabled={controller.locked || !snapshot.canUndo || !eligible}
              onclick={() => command({ kind: "undo" })}
            >
              <Undo aria-hidden="true" />
            </Button>
          {/snippet}
        </Tooltip.Trigger>
        <Tooltip.Content>Undo</Tooltip.Content>
      </Tooltip.Root>
    {/if}
    <Tooltip.Root>
      <Tooltip.Trigger>
        {#snippet child({ props })}
          <Button
            {...props}
            variant="ghost"
            size="icon-sm"
            aria-label="Refresh"
            disabled={controller.locked || controller.loading}
            onclick={() => controller.refresh()}
          >
            <Refresh aria-hidden="true" />
          </Button>
        {/snippet}
      </Tooltip.Trigger>
      <Tooltip.Content>Refresh</Tooltip.Content>
    </Tooltip.Root>
  </Tooltip.Provider>
</div>
