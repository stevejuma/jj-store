<script lang="ts">
  import { untrack } from "svelte";
  import StoreRepositoryActions from "../../web/src/lib/ui/store-mutations/StoreRepositoryActions.svelte";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";

  let { outcome = "committed" }: { outcome?: "committed" | "unknown" } =
    $props();
  const controller = untrack(() =>
    createInMemoryStoreController("branches", { mutationOutcome: outcome }),
  );
  untrack(() => controller.actions.selectBookmark("main"));
</script>

<div class="store-actions-fixture">
  <StoreRepositoryActions {controller} />
  <p role="status">{controller.state.mutation.status}</p>
  <p data-testid="repository-state">
    {controller.state.operationHeads.join(",")}
    {controller.state.route.commitId ?? "no selected commit"}
    {controller.state.bookmarks.map((bookmark) => bookmark.name).join(",")}
  </p>
</div>

<style>
  .store-actions-fixture {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 0.75rem;
    min-height: 10rem;
    padding: 1rem;
    background: var(--background);
  }
</style>
