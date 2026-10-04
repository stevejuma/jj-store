<script lang="ts">
  // Adapted from CommunityProjectsRepositoryRail's Details rows.
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { navigationItemIcon } from "../community-navigation/icons.js";
  import type { StorePage } from "../community-navigation/model.js";
  import { repositoryRailSections } from "./repository-sections.js";
  let { page, onSelect }: { page: StorePage; onSelect(page: StorePage): void } =
    $props();
</script>

<nav class="store-repository-navigation" aria-label="Repository views">
  <p>Details</p>
  {#each repositoryRailSections as item (item.page)}
    {@const Icon = navigationItemIcon({
      id: item.page,
      kind: item.page,
      label: item.label,
    })}
    <Button
      variant="ghost"
      class="store-repository-navigation__item"
      aria-current={page === item.page ? "page" : undefined}
      data-active={page === item.page ? "" : undefined}
      onclick={() => onSelect(item.page)}
    >
      <Icon aria-hidden="true" />
      <span>{item.label}</span>
    </Button>
  {/each}
</nav>

<style>
  .store-repository-navigation {
    display: grid;
    padding: 0.85rem 0.75rem 0.35rem;
    gap: 0.15rem;
  }
  p {
    margin: 0.05rem 0.4rem 0.3rem;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
  }
  :global(.store-repository-navigation__item[data-ui-component="button"]) {
    display: grid;
    width: 100%;
    height: 1.75rem;
    grid-template-columns: 0.875rem minmax(0, 1fr);
    justify-content: stretch;
    gap: 0.75rem;
    padding-inline: 0.5rem;
    color: var(--muted-foreground);
    font-size: var(--community-text-body);
    font-weight: 400;
    line-height: 1.25rem;
    text-align: start;
  }
  :global(.store-repository-navigation__item[data-active]),
  :global(.store-repository-navigation__item:hover:not(:disabled)),
  :global(.store-repository-navigation__item:focus-visible) {
    background-color: color-mix(in oklab, currentColor 10%, transparent);
  }
  :global(.store-repository-navigation__item > svg) {
    width: 0.875rem;
    height: 0.875rem;
  }
  :global(.store-repository-navigation__item > span) {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
