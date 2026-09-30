<script lang="ts">
  import type { StoreController } from "../../controller/contracts.js";
  import StoreRepositoryActions from "../store-mutations/StoreRepositoryActions.svelte";

  let { controller }: { controller: StoreController } = $props();
  const view = $derived(controller.state);
  const selectedId = $derived(
    view.route.page === "oplog"
      ? view.route.inspectedOperationId
      : view.route.page === "log" || view.route.page === "branches"
        ? view.route.commitId
        : null,
  );
  const selected = $derived(
    view.graph.revisions.find((revision) => revision.eventId === selectedId),
  );
  const selectedBookmark = $derived(
    view.route.page === "branches"
      ? view.bookmarks.find((bookmark) => bookmark.name === view.route.bookmark)
      : undefined,
  );
  const contextKey = $derived(
    JSON.stringify([
      view.route.repository,
      view.route.snapshotOperationId,
      view.route.page,
      selected?.eventId,
      selectedBookmark?.name,
    ]),
  );
</script>

{#if selected || selectedBookmark}
  <section class="store-repository-context-actions" aria-label="Actions">
    <p>Actions</p>
    {#key contextKey}
      <StoreRepositoryActions {controller} presentation="panel" />
    {/key}
  </section>
{/if}

<style>
  .store-repository-context-actions {
    padding: 0.85rem 0.75rem 0.35rem;
  }
  p {
    margin: 0.05rem 0.4rem 0.3rem;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
  }
</style>
