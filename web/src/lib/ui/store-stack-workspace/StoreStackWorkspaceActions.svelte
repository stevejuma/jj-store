<script lang="ts">
  import * as Menu from "@lapismd/design-core/shadcn/dropdown-menu";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { mergeProps } from "bits-ui";
  import Settings from "@lucide/svelte/icons/settings";
  import Plus from "@lucide/svelte/icons/plus";
  import Trash from "@lucide/svelte/icons/trash-2";
  import Ellipsis from "@lucide/svelte/icons/ellipsis";
  import GitBranchPlus from "@lucide/svelte/icons/git-branch-plus";
  import Redo from "@lucide/svelte/icons/redo-2";
  import Undo from "@lucide/svelte/icons/undo-2";
  import Refresh from "@lucide/svelte/icons/refresh-cw";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import type { WorkspaceCommand } from "../../controller/stack-workspace/types.js";
  let {
    controller,
    command,
    onSettings,
  }: {
    controller: StackWorkspaceController;
    command: (value: WorkspaceCommand) => void;
    onSettings?: () => void;
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
              disabled={!onSettings || !snapshot.active}
              onSelect={() => onSettings?.()}
              ><Settings aria-hidden="true" />Settings</Menu.Item
            >
            {#if snapshot.membershipChanged}<Menu.Item
                disabled={!controller.can("reconcile_workspace")}
                onSelect={() => command({ kind: "reconcile_workspace" })}
                ><Refresh aria-hidden="true" />Reapply saved branches</Menu.Item
              >{/if}
            <Menu.Item
              disabled={!controller.can("create_workspace")}
              title={controller.eligibility("create_workspace").reason ||
                undefined}
              onSelect={() => command({ kind: "create_workspace", name: "" })}
              ><Plus aria-hidden="true" />Create workspace</Menu.Item
            >
            {#each snapshot.workspaces.filter((item) => item.id !== snapshot.active) as item (item.id)}
              <Menu.Item
                disabled={!controller.can("delete_workspace")}
                title={controller.eligibility("delete_workspace").reason ||
                  undefined}
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
              disabled={!controller.can("create_stack")}
              onclick={() => command({ kind: "create_stack", name: "" })}
            >
              <GitBranchPlus aria-hidden="true" />
            </Button>
          {/snippet}
        </Tooltip.Trigger>
        <Tooltip.Content
          >{controller.eligibility("create_stack").reason ||
            "Create stack"}</Tooltip.Content
        >
      </Tooltip.Root>
      <Tooltip.Root>
        <Tooltip.Trigger>
          {#snippet child({ props })}
            <Button
              {...props}
              variant="ghost"
              size="icon-sm"
              aria-label="Undo"
              disabled={!controller.can("undo") || !snapshot.canUndo}
              onclick={() => command({ kind: "undo" })}
            >
              <Undo aria-hidden="true" />
            </Button>
          {/snippet}
        </Tooltip.Trigger>
        <Tooltip.Content
          >{controller.eligibility("undo").reason || "Undo"}</Tooltip.Content
        >
      </Tooltip.Root>
      {#if snapshot.capabilities.includes("redo")}
        <Tooltip.Root
          ><Tooltip.Trigger
            >{#snippet child({ props })}<Button
                {...props}
                variant="ghost"
                size="icon-sm"
                aria-label="Redo"
                disabled={!controller.can("redo") || !snapshot.canRedo}
                onclick={() => command({ kind: "redo" })}
                ><Redo aria-hidden="true" /></Button
              >{/snippet}</Tooltip.Trigger
          ><Tooltip.Content
            >{controller.eligibility("redo").reason || "Redo"}</Tooltip.Content
          ></Tooltip.Root
        >
      {/if}
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
