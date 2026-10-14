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
  import { getContext, type Snippet } from "svelte";
  import {
    conflictEditorKey,
    type OpenConflictEditor,
  } from "../store-conflicts/conflict-editor.js";
  import StoreMutationDialog from "./StoreMutationDialog.svelte";
  import StoreRepositoryActionsPanel from "./StoreRepositoryActionsPanel.svelte";
  import { mutationActionKind, type StoreMutationAction } from "./model.js";

  let {
    controller,
    presentation = "menu",
    primaryAction = "new",
    primaryLabel = "Add change",
    disabled = false,
    menu = $bindable(),
    renderMenu = true,
    menuLabel = "Repository actions",
    extraItems,
    showChanges = true,
    menuTrigger = $bindable(null),
    onCloseAutoFocus,
  }: {
    controller: StoreController;
    presentation?: "menu" | "panel" | "primary";
    primaryAction?: "new" | "bookmark-create";
    primaryLabel?: string;
    disabled?: boolean;
    menu?: Snippet;
    renderMenu?: boolean;
    menuLabel?: string;
    extraItems?: Snippet;
    showChanges?: boolean;
    menuTrigger?: HTMLButtonElement | null;
    onCloseAutoFocus?(event: Event): void;
  } = $props();
  const view = $derived(controller.state);
  const selectedCommit = $derived(
    view.route.page === "oplog"
      ? null
      : (view.route.commitId ??
          (view.targets.length === 1 ? view.targets[0]! : null)),
  );
  const selectedBookmark = $derived(
    view.bookmarks.some((item) => item.name === view.route.bookmark),
  );
  let action = $state<StoreMutationAction>("new");
  let pendingAction = $state<StoreMutationAction | "resolve-conflicts" | null>(
    null,
  );
  let dialogOpen = $state(false);
  const updatePause = `repository-action-${crypto.randomUUID()}`;
  $effect(() => {
    controller.actions.setRefreshPaused?.(updatePause, dialogOpen);
    return () => controller.actions.setRefreshPaused?.(updatePause, false);
  });
  const openConflictEditor = getContext<OpenConflictEditor | undefined>(
    conflictEditorKey,
  );

  function available(value: StoreMutationAction) {
    return (
      !disabled && view.mutation.available.includes(mutationActionKind[value])
    );
  }

  function select(value: StoreMutationAction | "resolve-conflicts") {
    pendingAction = value;
  }

  function activate(value: StoreMutationAction | "resolve-conflicts") {
    if (value === "resolve-conflicts") openConflictEditor?.();
    else {
      action = value;
      dialogOpen = true;
    }
  }

  function completeSelection(open: boolean) {
    if (open || !pendingAction) return;
    if (pendingAction === "resolve-conflicts") {
      pendingAction = null;
      openConflictEditor?.();
      return;
    }
    action = pendingAction;
    pendingAction = null;
    dialogOpen = true;
  }
  $effect(() => {
    menu = menuContent;
  });
</script>

{#snippet menuContent()}<DropdownMenu.Root
    onOpenChangeComplete={completeSelection}
  >
    <DropdownMenu.Trigger>
      {#snippet child({ props })}<Button
          {...props}
          bind:ref={menuTrigger}
          type="button"
          variant="ghost"
          size="icon-sm"
          aria-label={menuLabel}
          disabled={disabled ||
            (!extraItems &&
              (!showChanges ||
                !view.authenticated ||
                view.mutation.available.length === 0))}
          ><MoreHorizontalIcon aria-hidden="true" /></Button
        >{/snippet}
    </DropdownMenu.Trigger>
    <DropdownMenu.Content
      class="store-repository-actions-menu"
      align="end"
      sideOffset={4}
      preventScroll={false}
      aria-label={menuLabel}
      {onCloseAutoFocus}
    >
      {#if showChanges}
        <DropdownMenu.Group>
          <DropdownMenu.Label>Changes</DropdownMenu.Label>
          <DropdownMenu.Item
            disabled={!available("new")}
            onSelect={() => select("new")}
          >
            <PlusIcon data-icon="inline-start" aria-hidden="true" />Create
            change
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
            <FileWarningIcon
              data-icon="inline-start"
              aria-hidden="true"
            />Resolve conflicts
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
      {/if}
      {#if extraItems}
        {#if showChanges}<DropdownMenu.Separator />{/if}
        {@render extraItems()}
      {/if}
    </DropdownMenu.Content>
  </DropdownMenu.Root>{/snippet}

{#if presentation === "menu"}{#if renderMenu}{@render menuContent()}{/if}{:else if presentation === "primary"}<Button
    type="button"
    variant="ghost"
    size="icon-sm"
    class="store-repository-actions-primary"
    aria-label={primaryLabel}
    title={primaryLabel}
    disabled={!available(primaryAction) ||
      (primaryAction === "bookmark-create" && !selectedCommit)}
    onclick={() => activate(primaryAction)}
    ><PlusIcon aria-hidden="true" /></Button
  >{:else}<StoreRepositoryActionsPanel
    {controller}
    {selectedCommit}
    {selectedBookmark}
    {available}
    {activate}
  />{/if}

<Dialog.Root bind:open={dialogOpen}>
  <StoreMutationDialog
    {controller}
    {action}
    open={dialogOpen}
    onCloseAutoFocus={(event) => {
      if (presentation === "menu" && menuTrigger?.isConnected) {
        event.preventDefault();
        menuTrigger.focus();
      }
    }}
  />
</Dialog.Root>

<style>
  :global(.store-repository-actions-menu) {
    width: 16rem;
    max-width: calc(100vw - 2rem);
  }
</style>
