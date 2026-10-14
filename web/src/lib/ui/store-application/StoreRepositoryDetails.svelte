<script lang="ts">
  import type { StoreController } from "../../controller/contracts.js";
  import type { StorePage } from "../community-navigation/model.js";
  import StoreRepositoryActions from "../store-mutations/StoreRepositoryActions.svelte";
  let { controller }: { controller: StoreController } = $props();
  const view = $derived(controller.state);
  const contextTitle = $derived(repositoryContextTitle(view.route.page));
  const primaryAction = $derived(
    view.route.page === "branches"
      ? ({ action: "bookmark-create", label: "Add bookmark" } as const)
      : view.route.page === "log" || view.route.page === "files"
        ? ({ action: "new", label: "Add change" } as const)
        : undefined,
  );

  function repositoryContextTitle(page: StorePage): string {
    if (page === "branches") return "Branches/Bookmarks";
    if (page === "log") return "JJ Log/Changes";
    if (page === "oplog") return "Op Log/Operations";
    if (page === "files") return "Files/Repository";
    if (page === "actions") return "Actions/Workflow runs";
    if (page === "agents") return "Agents/Sessions";
    return "Reviews/Threads";
  }
</script>

<div class="store-repository-details">
  <div class="store-repository-details__context-heading">
    <strong>{contextTitle}</strong>
    {#if primaryAction}<StoreRepositoryActions
        {controller}
        presentation="primary"
        primaryAction={primaryAction.action}
        primaryLabel={primaryAction.label}
      />{/if}
  </div>
</div>

<style>
  .store-repository-details {
    width: 100%;
    padding: 0.85rem 0.75rem 0;
    box-sizing: border-box;
  }
  .store-repository-details__context-heading {
    display: grid;
    min-height: 2rem;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    gap: 0.5rem;
    padding: 0 0.4rem 0.55rem;
    font-size: var(--community-text-secondary);
    font-weight: 500;
  }

  .store-repository-details__context-heading strong {
    overflow: hidden;
    font-weight: 500;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .store-repository-details__context-heading
    :global(> [data-ui-component="button"]) {
    width: 1.75rem;
    min-width: 1.75rem;
    height: 1.75rem;
    min-height: 1.75rem;
    margin-inline-end: -0.4rem;
  }

  .store-repository-details__context-heading
    :global(> [data-ui-component="button"]:is(:hover, :focus-visible)) {
    color: var(--foreground);
    background: var(--muted);
  }
</style>
