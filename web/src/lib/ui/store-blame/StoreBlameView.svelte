<script lang="ts">
  import {
    flatTokensToLines,
    tokenizeAsync,
    type TokenLine,
  } from "@lapismd/design-core/shadcn/code-block";
  import { resolveDiffLanguage } from "@lapismd/design-core/diff";
  import type { StoreRoute } from "../../controller/contracts.js";
  import type { BlameViewState } from "../../controller/blame-types.js";
  import CommunityProjectsVirtualRows from "../community-projects/CommunityProjectsVirtualRows.svelte";
  import { repositorySourceTokenizer } from "../community-projects/repository-source-tokenizer.js";
  import StoreBlameGroup from "./StoreBlameGroup.svelte";
  import { blameChunks, sourceLines } from "./blame-model.js";
  let {
    contents,
    path,
    route,
    view,
    now,
    openRoute,
  }: {
    contents: string;
    path: string;
    route: StoreRoute;
    view: BlameViewState;
    now?: number;
    openRoute?: (route: StoreRoute) => Promise<void>;
  } = $props();
  const lines = $derived(sourceLines(contents));
  const chunks = $derived(
    blameChunks(lines.length, view.progress?.ranges ?? []),
  );
  const columns = $derived(
    lines.reduce(
      (max, line) => Math.max(max, line.replaceAll("\t", "    ").length),
      0,
    ),
  );
  let tokens = $state<TokenLine[]>([]);
  $effect(() => {
    const abort = new AbortController();
    const code = contents;
    const language = resolveDiffLanguage(path) ?? "plaintext";
    tokens = [];
    void (async () => {
      const custom = repositorySourceTokenizer(language);
      const value = custom
        ? flatTokensToLines(custom(code, language), code)
        : await tokenizeAsync(code, language, abort.signal);
      if (!abort.signal.aborted) tokens = value;
    })().catch(() => {
      /* Highlighting is optional; plain source remains usable. */
    });
    return () => abort.abort();
  });
</script>

<!-- Keyboard focus permits native horizontal and vertical source scrolling. -->
<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
<div
  class="store-blame"
  role="region"
  aria-label={`Blame for ${path}`}
  tabindex="0"
>
  {#if lines.length === 0}
    <p class="store-blame__empty">This file has no lines to attribute.</p>
  {:else}
    <div
      class="store-blame__track"
      style:width={`max(100%, calc(28rem + ${columns}ch))`}
    >
      <CommunityProjectsVirtualRows
        rows={chunks}
        getKey={(chunk) => String(chunk.start)}
        estimateSize={(chunk) => (chunk ? chunk.end - chunk.start : 1) * 28}
        overscan={2}
      >
        {#snippet row(chunk)}
          <StoreBlameGroup
            {chunk}
            {lines}
            {tokens}
            {route}
            {now}
            {openRoute}
            commit={chunk.commitId
              ? view.progress?.commits[chunk.commitId]
              : undefined}
            pending={view.status === "loading"}
          />
        {/snippet}
      </CommunityProjectsVirtualRows>
    </div>
  {/if}
</div>

<style>
  .store-blame {
    container-type: inline-size;
    max-height: 65vh;
    min-height: 3rem;
    overflow: auto;
    position: relative;
    font-size: var(--community-text-secondary, 0.875rem);
  }
  .store-blame:focus-visible {
    outline: 2px solid var(--ring);
    outline-offset: -2px;
  }
  .store-blame__track {
    min-width: 100%;
    font-family: var(--font-mono, monospace);
  }
  .store-blame__empty {
    padding: 1rem;
    color: var(--muted-foreground);
  }
</style>
