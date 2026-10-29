<script lang="ts">
  import { AppShell } from "@lapismd/design-core/shell";
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

<AppShell.Sidebar.Section class="store-repository-details">
  {#snippet heading()}<strong>{contextTitle}</strong>{/snippet}
  {#snippet actions()}
    {#if primaryAction}<StoreRepositoryActions
        {controller}
        presentation="primary"
        primaryAction={primaryAction.action}
        primaryLabel={primaryAction.label}
      />{/if}
  {/snippet}
</AppShell.Sidebar.Section>

<style>
  :global(.store-repository-details) {
    font-size: var(--community-text-secondary);
    font-weight: 500;
  }

  strong {
    overflow: hidden;
    font-weight: 500;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  :global(
    .store-repository-details
      [data-ui-part="sidebar-section-actions"]
      > [data-ui-component="button"]
  ) {
    width: 1.75rem;
    min-width: 1.75rem;
    height: 1.75rem;
    min-height: 1.75rem;
    margin-inline-end: -0.4rem;
  }

  :global(
    .store-repository-details
      [data-ui-part="sidebar-section-actions"]
      > [data-ui-component="button"]:is(:hover, :focus-visible)
  ) {
    color: var(--foreground);
    background: var(--muted);
  }
</style>
