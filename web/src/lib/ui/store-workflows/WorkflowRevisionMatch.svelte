<script lang="ts">
  import { SearchFilterInput } from "@lapismd/design-core/filter";
  import { historyExtensions } from "../store-history-query/history-language.js";
  import type { HistoryQueryInfo } from "../../controller/history-query-types.js";
  import "../store-history-query/history-query.css";
  let {
    id,
    value = "",
    info = null,
    symbols = [],
    disabled = false,
    onChange,
  }: {
    id: string;
    value?: string;
    info?: HistoryQueryInfo | null;
    symbols?: readonly string[];
    disabled?: boolean;
    onChange(value: string): void;
  } = $props();
  const extensions = $derived(
    historyExtensions("revset", info, symbols, () => {}),
  );
</script>

<div class="workflow-revset">
  <SearchFilterInput
    inputId={id}
    {value}
    {disabled}
    inputMode="custom"
    density="compact"
    showSearchIcon={false}
    showClearButton={false}
    ariaLabel="Revision match"
    placeholder="e.g. bookmarks(main)"
    editorExtensions={extensions}
    onValueChange={onChange}
  />
</div>

<style>
  .workflow-revset {
    min-width: 0;
    width: 100%;
  }
  .workflow-revset :global(.cv-search-filter-bar__search-pill) {
    border-radius: var(--ui-input-radius, 0.5rem);
    background: var(--ui-input-background, var(--background));
  }
</style>
