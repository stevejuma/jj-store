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

  let {
    controller,
    presentation = "menu",
  }: {
    controller: StoreController;
    presentation?: "menu" | "panel";
  } = $props();
  const view = $derived(controller.state);
  const selectedCommit = $derived(
    view.route.page === "oplog"
      ? null
      : (view.route.commitId ?? view.targets[0] ?? null),
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

  function activate(value: StoreMutationAction | "resolve-conflicts") {
    if (value === "resolve-conflicts") conflictOpen = true;
    else {
      action = value;
      dialogOpen = true;
    }
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

{#if presentation === "menu"}<DropdownMenu.Root
    onOpenChangeComplete={completeSelection}
  >
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
          <GitBranchPlusIcon
            data-icon="inline-start"
            aria-hidden="true"
          />Create bookmark
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
          <TrashIcon data-icon="inline-start" aria-hidden="true" />Delete
          bookmark
        </DropdownMenu.Item>
      </DropdownMenu.Group>
      {#if view.operationHeads.length > 1}
        <DropdownMenu.Separator />
        <DropdownMenu.Group>
          <DropdownMenu.Item
            disabled={!available("reconcile")}
            onSelect={() => select("reconcile")}
          >
            <GitMergeIcon
              data-icon="inline-start"
              aria-hidden="true"
            />Reconcile operations
          </DropdownMenu.Item>
        </DropdownMenu.Group>
      {/if}
    </DropdownMenu.Content>
  </DropdownMenu.Root>{:else}<div
    class="store-repository-actions-panel"
    role="group"
    aria-label="Selected repository actions"
  >
    {#if selectedCommit}
      <Button
        type="button"
        variant="ghost"
        class="store-repository-actions-panel__action"
        disabled={!available("describe")}
        onclick={() => activate("describe")}
        ><PencilIcon data-icon="inline-start" aria-hidden="true" /><span
          >Edit description</span
        ></Button
      >
      <Button
        type="button"
        variant="ghost"
        class="store-repository-actions-panel__action"
        disabled={!available("bookmark-create")}
        onclick={() => activate("bookmark-create")}
        ><GitBranchPlusIcon data-icon="inline-start" aria-hidden="true" /><span
          >Create bookmark</span
        ></Button
      >
      <Button
        type="button"
        variant="ghost"
        class="store-repository-actions-panel__action"
        disabled={view.conflicts.length === 0 ||
          !view.mutation.available.includes("resolve_conflicts")}
        onclick={() => activate("resolve-conflicts")}
        ><FileWarningIcon data-icon="inline-start" aria-hidden="true" /><span
          >Resolve conflicts</span
        ></Button
      >
      <Button
        type="button"
        variant="ghost"
        class="store-repository-actions-panel__action"
        disabled={!available("abandon")}
        onclick={() => activate("abandon")}
        ><GitCommitIcon data-icon="inline-start" aria-hidden="true" /><span
          >Abandon change</span
        ></Button
      >
    {/if}
    {#if selectedBookmark}
      <Button
        type="button"
        variant="ghost"
        class="store-repository-actions-panel__action"
        disabled={!selectedCommit || !available("bookmark-move")}
        onclick={() => activate("bookmark-move")}
        ><GitBranchIcon data-icon="inline-start" aria-hidden="true" /><span
          >Move bookmark</span
        ></Button
      >
      <Button
        type="button"
        variant="ghost"
        class="store-repository-actions-panel__action"
        disabled={!available("bookmark-delete")}
        onclick={() => activate("bookmark-delete")}
        ><TrashIcon data-icon="inline-start" aria-hidden="true" /><span
          >Delete bookmark</span
        ></Button
      >
    {/if}
    {#if view.route.page === "oplog" && view.operationHeads.length > 1}
      <Button
        type="button"
        variant="ghost"
        class="store-repository-actions-panel__action"
        disabled={!available("reconcile")}
        onclick={() => activate("reconcile")}
        ><GitMergeIcon data-icon="inline-start" aria-hidden="true" /><span
          >Reconcile operations</span
        ></Button
      >
    {/if}
    <Button
      type="button"
      variant="ghost"
      class="store-repository-actions-panel__action"
      disabled={!available("new")}
      onclick={() => activate("new")}
      ><PlusIcon data-icon="inline-start" aria-hidden="true" /><span
        >Create change</span
      ></Button
    >
  </div>{/if}

<Dialog.Root bind:open={dialogOpen}>
  <StoreMutationDialog {controller} {action} open={dialogOpen} />
</Dialog.Root>
<Dialog.Root bind:open={conflictOpen}>
  <StoreConflictEditor {controller} open={conflictOpen} />
</Dialog.Root>

<style>
  .store-repository-actions-panel {
    display: grid;
    gap: 0.15rem;
  }

  :global(.store-repository-actions-panel__action[data-ui-component="button"]) {
    display: grid;
    width: 100%;
    height: 1.75rem;
    grid-template-columns: 0.875rem minmax(0, 1fr);
    justify-content: stretch;
    gap: 0.75rem;
    padding-inline: 0.5rem;
    color: var(--muted-foreground);
    font-size: var(--community-text-body);
    font-weight: 400;
    line-height: 1.25rem;
    text-align: start;
  }

  :global(.store-repository-actions-panel__action > span) {
    overflow: hidden;
    text-overflow: ellipsis;
  }

  :global(.store-repository-actions-panel__action > svg) {
    width: 0.875rem;
    height: 0.875rem;
  }
</style>
