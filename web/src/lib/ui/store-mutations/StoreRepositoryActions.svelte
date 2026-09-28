<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import * as DropdownMenu from "@lapismd/design-core/shadcn/dropdown-menu";
  import MoreHorizontalIcon from "@lucide/svelte/icons/ellipsis";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import PencilIcon from "@lucide/svelte/icons/pencil";
  import GitBranchPlusIcon from "@lucide/svelte/icons/git-branch-plus";
  import GitBranchIcon from "@lucide/svelte/icons/git-branch";
  import TrashIcon from "@lucide/svelte/icons/trash-2";
  import GitMergeIcon from "@lucide/svelte/icons/git-merge";
  import GitCommitIcon from "@lucide/svelte/icons/git-commit-horizontal";
  import FileWarningIcon from "@lucide/svelte/icons/file-warning";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreConflictEditor from "../store-conflicts/StoreConflictEditor.svelte";
  import StoreMutationDialog from "./StoreMutationDialog.svelte";
  import { mutationActionKind, type StoreMutationAction } from "./model.js";

  let { controller }: { controller: StoreController } = $props();
  const view = $derived(controller.state);
  const selectedCommit = $derived(
    view.route.commitId ?? view.targets[0] ?? null,
  );
  const selectedBookmark = $derived(
    view.bookmarks.some((item) => item.name === view.route.bookmark),
  );
  let action = $state<StoreMutationAction>("new");
  let pendingAction = $state<StoreMutationAction | "resolve-conflicts" | null>(
    null,
  );
  let dialogOpen = $state(false);
  let conflictOpen = $state(false);

  function available(value: StoreMutationAction) {
    return view.mutation.available.includes(mutationActionKind[value]);
  }

  function select(value: StoreMutationAction | "resolve-conflicts") {
    pendingAction = value;
  }

  function completeSelection(open: boolean) {
    if (open || !pendingAction) return;
    if (pendingAction === "resolve-conflicts") {
      pendingAction = null;
      conflictOpen = true;
      return;
    }
    action = pendingAction;
    pendingAction = null;
    dialogOpen = true;
  }
</script>

<DropdownMenu.Root onOpenChangeComplete={completeSelection}>
  <DropdownMenu.Trigger>
    {#snippet child({ props })}<Button
        {...props}
        type="button"
        variant="ghost"
        size="icon-sm"
        aria-label="Repository actions"
        disabled={!view.authenticated || view.mutation.available.length === 0}
        ><MoreHorizontalIcon aria-hidden="true" /></Button
      >{/snippet}
  </DropdownMenu.Trigger>
  <DropdownMenu.Content
    align="end"
    sideOffset={4}
    preventScroll={false}
    aria-label="Repository actions"
  >
    <DropdownMenu.Group>
      <DropdownMenu.Label>Changes</DropdownMenu.Label>
      <DropdownMenu.Item
        disabled={!available("new")}
        onSelect={() => select("new")}
      >
        <PlusIcon data-icon="inline-start" aria-hidden="true" />Create change
      </DropdownMenu.Item>
      <DropdownMenu.Item
        disabled={!selectedCommit || !available("describe")}
        onSelect={() => select("describe")}
      >
        <PencilIcon data-icon="inline-start" aria-hidden="true" />Edit
        description
      </DropdownMenu.Item>
      <DropdownMenu.Item
        disabled={!selectedCommit || !available("abandon")}
        onSelect={() => select("abandon")}
      >
        <GitCommitIcon data-icon="inline-start" aria-hidden="true" />Abandon
        change
      </DropdownMenu.Item>
      <DropdownMenu.Item
        disabled={!selectedCommit ||
          view.conflicts.length === 0 ||
          !view.mutation.available.includes("resolve_conflicts")}
        onSelect={() => select("resolve-conflicts")}
      >
        <FileWarningIcon data-icon="inline-start" aria-hidden="true" />Resolve
        conflicts
      </DropdownMenu.Item>
    </DropdownMenu.Group>
    <DropdownMenu.Separator />
    <DropdownMenu.Group>
      <DropdownMenu.Label>Bookmarks</DropdownMenu.Label>
      <DropdownMenu.Item
        disabled={!selectedCommit || !available("bookmark-create")}
        onSelect={() => select("bookmark-create")}
      >
        <GitBranchPlusIcon data-icon="inline-start" aria-hidden="true" />Create
        bookmark
      </DropdownMenu.Item>
      <DropdownMenu.Item
        disabled={!selectedCommit ||
          !selectedBookmark ||
          !available("bookmark-move")}
        onSelect={() => select("bookmark-move")}
      >
        <GitBranchIcon data-icon="inline-start" aria-hidden="true" />Move
        bookmark
      </DropdownMenu.Item>
      <DropdownMenu.Item
        disabled={!selectedBookmark || !available("bookmark-delete")}
        onSelect={() => select("bookmark-delete")}
      >
        <TrashIcon data-icon="inline-start" aria-hidden="true" />Delete bookmark
      </DropdownMenu.Item>
    </DropdownMenu.Group>
    {#if view.operationHeads.length > 1}
      <DropdownMenu.Separator />
      <DropdownMenu.Group>
        <DropdownMenu.Item
          disabled={!available("reconcile")}
          onSelect={() => select("reconcile")}
        >
          <GitMergeIcon data-icon="inline-start" aria-hidden="true" />Reconcile
          operations
        </DropdownMenu.Item>
      </DropdownMenu.Group>
    {/if}
  </DropdownMenu.Content>
</DropdownMenu.Root>

<Dialog.Root bind:open={dialogOpen}>
  <StoreMutationDialog {controller} {action} open={dialogOpen} />
</Dialog.Root>
<Dialog.Root bind:open={conflictOpen}>
  <StoreConflictEditor {controller} open={conflictOpen} />
</Dialog.Root>
