<script lang="ts">
  import * as Menu from "@lapismd/design-core/shadcn/dropdown-menu";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreStackCopyMenu from "./StoreStackCopyMenu.svelte";
  import StoreStackRelativeActions from "./StoreStackRelativeActions.svelte";
  import Trash from "@lucide/svelte/icons/trash-2";
  import Pencil from "@lucide/svelte/icons/pencil";
  import GitBranch from "@lucide/svelte/icons/git-branch";
  import GitMerge from "@lucide/svelte/icons/git-merge";
  import Columns from "@lucide/svelte/icons/columns-2";
  import ArrowUp from "@lucide/svelte/icons/arrow-up";
  import ArrowDown from "@lucide/svelte/icons/arrow-down";
  import Undo from "@lucide/svelte/icons/undo-2";
  import Ellipsis from "@lucide/svelte/icons/ellipsis";
  import type {
    StackCommit,
    WorkspaceCommand,
    WorkspaceStack,
    WorkspaceSelection,
  } from "../../controller/stack-workspace/types.js";
  let {
    commit,
    stack,
    disabled,
    selection,
    enabled = () => true,
    reason = () => "",
    onCommand,
  }: {
    commit: StackCommit;
    stack: WorkspaceStack;
    disabled: boolean;
    selection: WorkspaceSelection | null;
    enabled?: (kind: WorkspaceCommand["kind"]) => boolean;
    reason?: (kind: WorkspaceCommand["kind"]) => string;
    onCommand: (command: WorkspaceCommand) => void;
  } = $props();
  const previous = $derived(
    stack.commits[stack.commits.findIndex((item) => item.id === commit.id) - 1],
  );
  const next = $derived(
    stack.commits[stack.commits.findIndex((item) => item.id === commit.id) + 1],
  );
</script>

<Menu.Root>
  <Menu.Trigger
    >{#snippet child({ props })}<Button
        {...props}
        variant="ghost"
        size="icon-sm"
        class="stack-workspace__menu-trigger"
        aria-label={`Actions for ${commit.message.split("\n")[0]}`}
        {disabled}><Ellipsis aria-hidden="true" /></Button
      >{/snippet}</Menu.Trigger
  >
  <Menu.Content
    align="end"
    preventScroll={false}
    class="stack-workspace__action-menu"
  >
    <Menu.Group>
      <Menu.Item
        title={reason("reword") || undefined}
        disabled={commit.immutable || !enabled("reword")}
        onSelect={() =>
          onCommand({
            kind: "reword",
            commit: commit.id,
            message: commit.message,
          })}><Pencil aria-hidden="true" />Reword commit</Menu.Item
      >
      <Menu.Item
        title={reason("edit_commit") || undefined}
        disabled={commit.immutable || !enabled("edit_commit")}
        onSelect={() =>
          onCommand({
            kind: "edit_commit",
            commit: commit.id,
            message: commit.message,
          })}><GitBranch aria-hidden="true" />Edit commit</Menu.Item
      >
      <Menu.Item
        title={reason("delete_commit") || undefined}
        disabled={commit.immutable || !enabled("delete_commit")}
        variant="destructive"
        onSelect={() => onCommand({ kind: "delete_commit", commit: commit.id })}
        ><Trash aria-hidden="true" />Delete commit</Menu.Item
      >
      <Menu.Separator />
      <StoreStackCopyMenu
        items={[
          { label: "Copy change ID", value: commit.changeId },
          { label: "Copy commit hash", value: commit.id },
          { label: "Copy commit message", value: commit.message },
        ]}
      />
      <StoreStackRelativeActions
        {enabled}
        {reason}
        stack={stack.id}
        commit={commit.id}
        immutable={commit.immutable}
        {onCommand}
      />
      <Menu.Separator />
      <Menu.Item
        title={reason("move_commit") || undefined}
        disabled={commit.immutable || !enabled("move_commit")}
        onSelect={() =>
          onCommand({ kind: "move_commit", commit: commit.id, stack: "" })}
        ><GitBranch aria-hidden="true" />Move to stack</Menu.Item
      >
      <Menu.Item
        title={reason("split") || undefined}
        disabled={commit.immutable ||
          !enabled("split") ||
          selection?.source !== commit.id ||
          selection.hunks?.length === 0}
        onSelect={() =>
          onCommand({
            kind: "split",
            commit: commit.id,
            message: "",
            selection: selection!,
          })}><Columns aria-hidden="true" />Split selected changes</Menu.Item
      >
      <Menu.Item
        title={reason("reorder_commit") || undefined}
        disabled={commit.immutable ||
          !enabled("reorder_commit") ||
          stack.commits.at(-1)?.id === commit.id}
        onSelect={() =>
          onCommand({
            kind: "reorder_commit",
            commit: commit.id,
            target: next!.id,
            placement: "above",
          })}><ArrowUp aria-hidden="true" />Move up</Menu.Item
      >
      <Menu.Item
        title={reason("reorder_commit") || undefined}
        disabled={commit.immutable || !enabled("reorder_commit") || !previous}
        onSelect={() =>
          onCommand({
            kind: "reorder_commit",
            commit: commit.id,
            target: previous!.id,
            placement: "below",
          })}><ArrowDown aria-hidden="true" />Move down</Menu.Item
      >
      <Menu.Item
        title={reason("squash") || undefined}
        disabled={commit.immutable ||
          !enabled("squash") ||
          !previous ||
          previous.immutable}
        onSelect={() =>
          previous &&
          onCommand({ kind: "squash", commit: commit.id, target: previous.id })}
        ><GitMerge aria-hidden="true" />Squash into previous</Menu.Item
      >
      <Menu.Item
        title={reason("uncommit") || undefined}
        disabled={commit.immutable || !enabled("uncommit")}
        onSelect={() => onCommand({ kind: "uncommit", commit: commit.id })}
        ><Undo aria-hidden="true" />Uncommit</Menu.Item
      >
    </Menu.Group>
  </Menu.Content>
</Menu.Root>
