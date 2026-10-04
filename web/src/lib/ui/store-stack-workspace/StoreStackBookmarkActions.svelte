<script lang="ts">
  import * as Menu from "@lapismd/design-core/shadcn/dropdown-menu";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreStackCopyMenu from "./StoreStackCopyMenu.svelte";
  import StoreStackRelativeActions from "./StoreStackRelativeActions.svelte";
  import GitMerge from "@lucide/svelte/icons/git-merge";
  import Trash from "@lucide/svelte/icons/trash-2";
  import Pencil from "@lucide/svelte/icons/pencil";
  import GitBranch from "@lucide/svelte/icons/git-branch";
  import Eject from "@lucide/svelte/icons/eject";
  import Ellipsis from "@lucide/svelte/icons/ellipsis";
  import Upload from "@lucide/svelte/icons/upload";
  import GitCommit from "@lucide/svelte/icons/git-commit-horizontal";
  import type { WorkspaceCommand } from "../../controller/stack-workspace/types.js";
  let {
    stack,
    name,
    canAddCommit,
    commit,
    empty,
    below,
    canRewrite,
    canSquash,
    disabled,
    onCommand,
  }: {
    stack: string;
    name: string;
    canAddCommit: boolean;
    commit: string;
    empty: boolean;
    below: string;
    canRewrite: boolean;
    canSquash: boolean;
    disabled: boolean;
    onCommand: (command: WorkspaceCommand) => void;
  } = $props();
</script>

<div
  class="stack-workspace__bookmark-actions"
  role="group"
  aria-label={`Branch actions for ${name}`}
>
  <Tooltip.Provider>
    {#if canAddCommit}<Tooltip.Root
        ><Tooltip.Trigger>
          {#snippet child({ props })}
            <Button
              {...props}
              variant="outline"
              size="icon-sm"
              aria-label={`Add empty commit to ${name}`}
              {disabled}
              onclick={() =>
                onCommand({ kind: "create_commit", stack, message: "" })}
              ><GitCommit aria-hidden="true" /></Button
            >
          {/snippet}
        </Tooltip.Trigger><Tooltip.Content>Add empty commit</Tooltip.Content
        ></Tooltip.Root
      >{/if}
    <Tooltip.Root
      ><Tooltip.Trigger>
        {#snippet child({ props })}
          <Button
            {...props}
            variant="outline"
            size="sm"
            aria-disabled="true"
            aria-label={`Push ${name}`}
          >
            <Upload data-icon="inline-start" aria-hidden="true" />Push
          </Button>
        {/snippet}
      </Tooltip.Trigger><Tooltip.Content
        >Push requires the separately approved backend integration.</Tooltip.Content
      ></Tooltip.Root
    >
  </Tooltip.Provider>
  <Menu.Root
    ><Menu.Trigger>
      {#snippet child({ props })}<Button
          {...props}
          variant="ghost"
          size="icon-sm"
          class="stack-workspace__bookmark-menu stack-workspace__menu-trigger"
          aria-label={`Branch actions for ${name}`}
          {disabled}><Ellipsis aria-hidden="true" /></Button
        >{/snippet}
    </Menu.Trigger><Menu.Content
      align="end"
      preventScroll={false}
      class="stack-workspace__action-menu"
    >
      <StoreStackCopyMenu
        submenu={false}
        items={[{ label: "Copy branch name", value: name }]}
      />
      <Menu.Separator />
      <StoreStackRelativeActions
        {stack}
        {commit}
        {empty}
        {below}
        immutable={!canRewrite}
        {onCommand}
      />
      <Menu.Group>
        <Menu.Item
          onSelect={() =>
            onCommand(
              empty
                ? { kind: "create_commit", stack, message: "" }
                : {
                    kind: "add_empty_commit",
                    commit,
                    placement: "above",
                    message: "",
                  },
            )}><GitCommit aria-hidden="true" />Add empty commit</Menu.Item
        >
        <Menu.Item
          disabled={!canSquash}
          onSelect={() => onCommand({ kind: "squash_branch", stack, name })}
          ><GitMerge aria-hidden="true" />Squash all commits</Menu.Item
        >
        <Menu.Separator />
        <Menu.Item
          onSelect={() =>
            onCommand({ kind: "rename_branch", stack, branch: name, name })}
          ><Pencil aria-hidden="true" />Rename branch</Menu.Item
        >
        <Menu.Item
          disabled={!canRewrite}
          variant="destructive"
          onSelect={() => onCommand({ kind: "delete_branch", stack, name })}
          ><Trash aria-hidden="true" />Delete branch</Menu.Item
        >
        <Menu.Item onSelect={() => onCommand({ kind: "rebase", stack })}
          ><GitBranch aria-hidden="true" />Rebase onto base</Menu.Item
        >
        <Menu.Item onSelect={() => onCommand({ kind: "unapply_stack", stack })}
          ><Eject aria-hidden="true" />Unapply stack</Menu.Item
        >
      </Menu.Group></Menu.Content
    ></Menu.Root
  >
</div>
