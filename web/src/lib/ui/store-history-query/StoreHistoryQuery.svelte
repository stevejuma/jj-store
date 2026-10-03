<script lang="ts">
  import { SearchFilterInput } from "@lapismd/design-core/filter";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import type { StoreController } from "../../controller/contracts.js";
  import { historyExtensions } from "./history-language.js";
  import "./history-query.css";
  let { controller }: { controller: StoreController } = $props();
  const query = $derived(controller.historyQuery);
  const operations = $derived(controller.state.route.page === "oplog");
  const extensions = $derived(
    historyExtensions(
      operations ? "operations" : "revset",
      query?.info ?? null,
      controller.state.bookmarks.map((bookmark) => bookmark.name),
      () => void query?.run(),
    ),
  );
</script>

{#if query}
  <div class="store-history-query" aria-busy={query.pending}>
    <SearchFilterInput
      value={query.draft}
      inputMode="custom"
      density="compact"
      focusAppearance="borderless"
      ariaLabel={operations ? "Filter operation history" : "Filter JJ history"}
      placeholder={operations
        ? 'description:"rebase" user:steve'
        : 'mutable() & description("fix")'}
      editorExtensions={extensions}
      error={query.error || null}
      onValueChange={query.setDraft}
      onClearSearch={query.clear}
    >
      {#snippet actions()}<Button
          size="sm"
          variant="secondary"
          onclick={() => void query.run()}
          aria-label="Run history query">Search</Button
        >{/snippet}
    </SearchFilterInput>
    <details class="store-history-query__context">
      <summary>Query context and syntax</summary>
      {#if operations}
        <p>
          Combine fields with spaces or AND. Text matches descriptions. Use
          description, user, host, workspace, id, snapshot, after, before, or
          from. Dates use YYYY-MM-DD in UTC; after includes the day and before
          excludes it. For example: <code
            >from:@-- description:"rebase branch" snapshot:false</code
          >.
        </p>
      {:else}
        <label for="history-workspace">Workspace for @</label>
        <select
          id="history-workspace"
          value={controller.state.route.workspace ?? ""}
          onchange={(event) =>
            void controller.actions.selectWorkspace(
              event.currentTarget.value || null,
            )}
        >
          <option value="">Choose workspace</option>
          {#each controller.state.workingCopies as copy}<option
              value={copy.workspace}>{copy.workspace}</option
            >{/each}
        </select>
        <label for="history-email">Author email for mine()</label>
        <Input
          id="history-email"
          type="email"
          value={query.email}
          oninput={(event) => query.setEmail(event.currentTarget.value)}
        />
        <p>
          JJ {query.info?.jj_version ?? "0.45.1"} revsets and standard aliases. Try
          <code>mutable()</code>, <code>::@</code> or
          <code>description("fix")</code>. Custom aliases are unavailable. Email
          is kept for this session.
        </p>
        {#if query.info?.unsupported.length}<p>
            Unavailable for this repository: {query.info.unsupported.join(
              ", ",
            )}.
          </p>{/if}
      {/if}
      <p>
        Queries use the displayed operation snapshot. Enter adds a line;
        Ctrl/⌘+Enter or Search runs the query. Enter accepts an active
        completion. Escape closes completions. Invalid drafts keep the previous
        results.
      </p>
    </details>
    {#if query.pending}<p role="status">Searching history…</p>{/if}
  </div>
{/if}
