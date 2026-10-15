<script lang="ts">
  import { SearchFilterInput } from "@lapismd/design-core/filter";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import SearchIcon from "@lucide/svelte/icons/search";
  import SettingsIcon from "@lucide/svelte/icons/settings-2";
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import StoreHistoryQueryContext from "./StoreHistoryQueryContext.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import { historyExtensions } from "./history-language.js";
  import "./history-query.css";
  let { controller }: { controller: StoreController } = $props();
  let contextOpen = $state(false);
  const query = $derived(controller.historyQuery);
  const operations = $derived(controller.state.route.page === "oplog");
  // The production view also reads graph/loading state. Preserve the editor's
  // completion state when that view changes without changing its vocabulary.
  const bookmarkNames = $derived(
    JSON.stringify(controller.state.bookmarks.map((bookmark) => bookmark.name)),
  );
  const extensions = $derived(
    historyExtensions(
      operations ? "operations" : "revset",
      query?.info ?? null,
      JSON.parse(bookmarkNames) as string[],
      () => void query?.run(),
    ),
  );
</script>

{#if query}
  <div class="store-history-query" aria-busy={query.pending}>
    <SearchFilterInput
      value={query.draft}
      inputMode="custom"
      showSearchIcon={false}
      ariaLabel={operations ? "Filter operation history" : "Filter JJ history"}
      placeholder={operations
        ? 'description:"rebase" user:steve'
        : 'mutable() & description("fix")'}
      editorExtensions={extensions}
      error={query.error || null}
      onValueChange={query.setDraft}
      onClearSearch={query.clear}
    >
      {#snippet actions()}
        <Button
          size="icon"
          variant="ghost"
          class="store-history-query__round"
          onclick={() => void query.run()}
          aria-label="Run history query"
          title="Run history query"><SearchIcon aria-hidden="true" /></Button
        >
        <Button
          size="icon"
          variant={contextOpen ? "default" : "ghost"}
          class="store-history-query__round"
          aria-label="Query context and syntax"
          title="Query context and syntax"
          aria-expanded={contextOpen}
          aria-controls="history-query-context"
          onclick={() => (contextOpen = !contextOpen)}
          ><SettingsIcon aria-hidden="true" /></Button
        >
      {/snippet}
    </SearchFilterInput>
    <Collapsible.Root open={contextOpen}>
      <Collapsible.Content id="history-query-context"
        ><StoreHistoryQueryContext {controller} /></Collapsible.Content
      >
    </Collapsible.Root>
    {#if query.pending}<div
        role="status"
        aria-label="Searching history"
        aria-busy="true"
      >
        <Skeleton style="height: 1rem; width: 100%" />
      </div>{/if}
  </div>
{/if}
