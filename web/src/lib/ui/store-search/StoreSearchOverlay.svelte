<script lang="ts">
  import { onDestroy } from "svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import type { StoreSearchResult } from "../../controller/contracts.js";
  import StoreSearchCommand from "./StoreSearchCommand.svelte";

  let {
    controller,
    open = $bindable(false),
  }: { controller: StoreController; open?: boolean } = $props();
  let query = $state("");
  let timer: ReturnType<typeof setTimeout> | undefined;
  let selectionFrame: ReturnType<typeof requestAnimationFrame> | undefined;
  let previouslyOpen = $state(open);
  let pendingResult: StoreSearchResult | null = null;

  function cancelTimer() {
    if (timer !== undefined) clearTimeout(timer);
    timer = undefined;
  }

  function cancelSelectionFrame() {
    if (selectionFrame !== undefined) cancelAnimationFrame(selectionFrame);
    selectionFrame = undefined;
  }

  function updateQuery(value: string) {
    query = value;
    cancelTimer();
    if (!value.trim()) {
      void controller.actions.search("");
      return;
    }
    timer = setTimeout(() => {
      timer = undefined;
      void controller.actions.search(value);
    }, 250);
  }

  function reset() {
    cancelTimer();
    query = "";
    void controller.actions.search("");
  }

  $effect(() => {
    if (previouslyOpen && !open) reset();
    previouslyOpen = open;
  });
  onDestroy(() => {
    cancelTimer();
    cancelSelectionFrame();
  });
</script>

<StoreSearchCommand
  bind:open
  {query}
  search={controller.state.search}
  now={controller.state.now}
  onQueryChange={updateQuery}
  onSelect={(result) => {
    cancelTimer();
    cancelSelectionFrame();
    pendingResult = result;
    open = false;
  }}
  onOpenChangeComplete={(completedOpen) => {
    if (completedOpen) {
      cancelSelectionFrame();
      return;
    }
    if (!pendingResult) return;
    const result = pendingResult;
    pendingResult = null;
    selectionFrame = requestAnimationFrame(() => {
      selectionFrame = undefined;
      void controller.actions.selectSearchResult(result);
    });
  }}
  onLoadMore={() => controller.actions.loadMore("search")}
/>
