<script lang="ts">
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import type { TokenLine } from "@lapismd/design-core/shadcn/code-block";
  import type { RepositoryCommitSummary } from "../../projects/repository-log.js";
  import type { StoreRoute } from "../../controller/contracts.js";
  import { storeRouteToHash } from "../../controller/store-route-codec.js";
  import CommunityProjectsGitAuthorAvatar from "../community-projects/CommunityProjectsGitAuthorAvatar.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import { blameCommitRoute, type BlameChunk } from "./blame-model.js";
  let {
    chunk,
    lines,
    tokens,
    commit,
    route,
    now,
    pending,
    openRoute,
  }: {
    chunk: BlameChunk;
    lines: string[];
    tokens: TokenLine[];
    commit?: RepositoryCommitSummary;
    route: StoreRoute;
    now?: number;
    pending: boolean;
    openRoute?: (route: StoreRoute) => Promise<void>;
  } = $props();
  const originRoute = $derived(
    chunk.commitId ? blameCommitRoute(route, chunk.commitId) : undefined,
  );
  const href = $derived(originRoute ? storeRouteToHash(originRoute) : null);
  const description = $derived(
    commit?.message.split(/\r?\n/u)[0] || "(no description)",
  );
</script>

<div class="blame-group" data-continued={chunk.continued || undefined}>
  <div class="blame-group__gutter">
    <div
      class="blame-group__origin"
      aria-label={`Origin for lines ${chunk.start}–${chunk.end - 1}${chunk.continued ? " (continued)" : ""}`}
    >
      {#if commit && href}
        <CommunityProjectsGitAuthorAvatar
          name={commit.author}
          email={commit.authorEmail}
          size="xsmall"
        />
        <span class="blame-group__author" title={commit.author}
          >{commit.author}</span
        >
        <a
          class="blame-group__description"
          {href}
          title={description}
          aria-label={`${description} — ${commit.author}${chunk.continued ? " (continued)" : ""}`}
          onclick={(event) => {
            if (
              !openRoute ||
              !originRoute ||
              event.button ||
              event.ctrlKey ||
              event.metaKey ||
              event.shiftKey ||
              event.altKey
            )
              return;
            event.preventDefault();
            void openRoute(originRoute);
          }}>{description}</a
        >
        <span class="blame-group__time"
          ><CommunityRelativeTime timestamp={commit.createdAt} {now} /></span
        >
      {:else if pending}
        <span
          class="blame-group__loading"
          role="status"
          aria-label={`Loading origin for lines ${chunk.start}–${chunk.end - 1}`}
          ><Skeleton style="width:100%;height:0.8rem" /></span
        >
      {:else}
        <span class="blame-group__unknown">Origin unavailable</span>
      {/if}
    </div>
  </div>
  <div class="blame-group__source">
    {#each lines.slice(chunk.start - 1, chunk.end - 1) as line, index (index)}
      {@const number = chunk.start + index}
      <div class="blame-group__line" data-line={number}>
        <span class="blame-group__number" aria-label={`Line ${number}`}
          >{number}</span
        >
        <code
          >{#each tokens[number - 1] ?? [] as token, ti (ti)}{line.slice(
              ti ? tokens[number - 1]![ti - 1]!.end : 0,
              token.start,
            )}<span class={`ui-code-token-${token.type}`}
              >{line.slice(token.start, token.end)}</span
            >{/each}{line.slice(tokens[number - 1]?.at(-1)?.end ?? 0) ||
            (line ? "" : "\u200b")}</code
        >
      </div>
    {/each}
  </div>
</div>

<style>
  .blame-group {
    display: grid;
    grid-template-columns: min(22rem, 45cqw) minmax(0, 1fr);
  }
  .blame-group__gutter {
    position: sticky;
    left: 0;
    z-index: 1;
    background: var(--background);
    border-right: 1px solid var(--border);
    font-family: var(--font-sans, sans-serif);
  }
  .blame-group__origin {
    position: sticky;
    top: 0;
    display: flex;
    align-items: center;
    gap: 0.4rem;
    height: 28px;
    padding: 0 0.5rem;
    border-top: 1px solid var(--border);
  }
  .blame-group__author {
    max-width: 4.5rem;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
    font-size: 0.75rem;
  }
  .blame-group__description {
    min-width: 0;
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .blame-group__time {
    flex: none;
    color: var(--muted-foreground);
    font-size: 0.75rem;
  }
  .blame-group__loading {
    width: 100%;
    padding: 0 0.5rem;
  }
  .blame-group__unknown {
    color: var(--muted-foreground);
    font-size: 0.75rem;
  }
  .blame-group__line {
    display: flex;
    height: 28px;
    line-height: 28px;
    white-space: pre;
  }
  .blame-group__number {
    width: 5rem;
    flex: none;
    text-align: right;
    padding-right: 1rem;
    color: var(--muted-foreground);
    user-select: none;
  }
  code {
    font: inherit;
    tab-size: 4;
    padding-right: 1rem;
  }
  a:focus-visible {
    outline: 2px solid var(--ring);
  }
  a:hover {
    text-decoration: underline;
  }
</style>
