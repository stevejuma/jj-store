<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import PencilIcon from "@lucide/svelte/icons/pencil";
  import GitBranchPlusIcon from "@lucide/svelte/icons/git-branch-plus";
  import GitBranchIcon from "@lucide/svelte/icons/git-branch";
  import TrashIcon from "@lucide/svelte/icons/trash-2";
  import GitMergeIcon from "@lucide/svelte/icons/git-merge";
  import GitCommitIcon from "@lucide/svelte/icons/git-commit-horizontal";
  import FileWarningIcon from "@lucide/svelte/icons/file-warning";
  import type { StoreController } from "../../controller/contracts.js";
  import type { StoreMutationAction } from "./model.js";

  let {
    controller,
    selectedCommit,
    selectedBookmark,
    available,
    activate,
  }: {
    controller: StoreController;
    selectedCommit: string | null;
    selectedBookmark: boolean;
    available: (action: StoreMutationAction) => boolean;
    activate: (action: StoreMutationAction | "resolve-conflicts") => void;
  } = $props();
  const view = $derived(controller.state);
</script>

<div
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
</div>

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
