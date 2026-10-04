<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import { AppShell } from "@lapismd/design-core/shell";
  import SearchIcon from "@lucide/svelte/icons/search";

  let {
    compact,
    onSearch,
  }: {
    compact: boolean;
    onSearch?: (query: string) => void;
  } = $props();

  function requestSearch(): void {
    onSearch?.("");
  }
  function handleShortcut(event: KeyboardEvent): void {
    if (
      onSearch &&
      (event.metaKey || event.ctrlKey) &&
      event.key.toLocaleLowerCase() === "k"
    ) {
      event.preventDefault();
      requestSearch();
    }
  }
</script>

<svelte:window onkeydown={handleShortcut} />
<AppShell.Sidebar.Header class="lc-community-navigation__header">
  {#if onSearch}
    {#if compact}
      <Tooltip.Provider>
        <Tooltip.Root>
          <Tooltip.Trigger>
            {#snippet child({ props })}
              <Button
                {...props}
                variant="ghost"
                size="icon-sm"
                class="lc-community-navigation__compact-search"
                aria-label="Search repository"
                onclick={requestSearch}
              >
                <SearchIcon aria-hidden="true" />
              </Button>
            {/snippet}
          </Tooltip.Trigger>
          <Tooltip.Content side="right" sideOffset={8}
            >Search repository</Tooltip.Content
          >
        </Tooltip.Root>
      </Tooltip.Provider>
    {:else}
      <Button
        type="button"
        variant="ghost"
        class="lc-community-navigation__search-trigger"
        aria-label="Search repository"
        onclick={requestSearch}
      >
        <SearchIcon aria-hidden="true" />
        <span>Search repository</span>
        <kbd aria-hidden="true">⌘K</kbd>
      </Button>
    {/if}
  {/if}
</AppShell.Sidebar.Header>
