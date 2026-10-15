<script lang="ts">
  import type { Snippet } from "svelte";
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
    commitStack = stack,
    name,
    applied = true,
    canAddCommit,
    commit,
    empty,
    below,
    canRewrite,
    canSquash,
    disabled,
    enabled = () => true,
    reason = () => "",
    onCommand,
    afterPush,
  }: {
    afterPush?: Snippet;
    stack: string;
    commitStack?: string;
    name: string;
    applied?: boolean;
    canAddCommit: boolean;
    commit: string;
    empty: boolean;
    below: string;
    canRewrite: boolean;
    canSquash: boolean;
    disabled: boolean;
    enabled?: (kind: WorkspaceCommand["kind"]) => boolean;
    reason?: (kind: WorkspaceCommand["kind"]) => string;
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
              disabled={disabled || !enabled("create_commit")}
              onclick={() =>
                onCommand({
                  kind: "create_commit",
                  stack: commitStack,
                  message: "",
                })}><GitCommit aria-hidden="true" /></Button
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
        >Push is not available for this stack workspace.</Tooltip.Content
      ></Tooltip.Root
    >
  </Tooltip.Provider>
  {@render afterPush?.()}
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
        {enabled}
        {reason}
        {stack}
        {commit}
        {empty}
        {below}
        immutable={!canRewrite}
        {onCommand}
      />
      <Menu.Group>
        <Menu.Item
          title={reason(empty ? "create_commit" : "add_empty_commit") ||
            undefined}
          disabled={!enabled(empty ? "create_commit" : "add_empty_commit")}
          onSelect={() =>
            onCommand(
              empty
                ? { kind: "create_commit", stack: commitStack, message: "" }
                : {
                    kind: "add_empty_commit",
                    commit,
                    placement: "above",
                    message: "",
                  },
            )}><GitCommit aria-hidden="true" />Add empty commit</Menu.Item
        >
        <Menu.Item
          title={reason("squash_branch") || undefined}
          disabled={!canSquash || !enabled("squash_branch")}
          onSelect={() => onCommand({ kind: "squash_branch", stack, name })}
          ><GitMerge aria-hidden="true" />Squash all commits</Menu.Item
        >
        <Menu.Separator />
        <Menu.Item
          title={reason("rename_branch") || undefined}
          disabled={!enabled("rename_branch")}
          onSelect={() =>
            onCommand({ kind: "rename_branch", stack, branch: name, name })}
          ><Pencil aria-hidden="true" />Rename branch</Menu.Item
        >
        <Menu.Item
          title={reason("delete_branch") || undefined}
          disabled={!canRewrite || !enabled("delete_branch")}
          variant="destructive"
          onSelect={() => onCommand({ kind: "delete_branch", stack, name })}
          ><Trash aria-hidden="true" />Delete branch</Menu.Item
        >
        <Menu.Item
          title={reason("rebase") || undefined}
          disabled={!enabled("rebase")}
          onSelect={() => onCommand({ kind: "rebase", stack, branch: name })}
          ><GitBranch aria-hidden="true" />Rebase onto base</Menu.Item
        >
        <Menu.Item
          title={reason("unapply_stack") || undefined}
          disabled={!applied || !enabled("unapply_stack")}
          onSelect={() =>
            onCommand({ kind: "unapply_stack", stack, branch: name })}
          ><Eject aria-hidden="true" />Unapply branch</Menu.Item
        >
      </Menu.Group></Menu.Content
    ></Menu.Root
  >
</div>
