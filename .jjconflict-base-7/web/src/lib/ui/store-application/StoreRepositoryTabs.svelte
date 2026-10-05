<script lang="ts">
  // Width fitting follows CommunityProjectsRepositoryDocuments; pill styling is retained.
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as DropdownMenu from "@lapismd/design-core/shadcn/dropdown-menu";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
  import { onMount, tick } from "svelte";
  import type { StorePage } from "../community-navigation/model.js";
  import { repositorySections } from "./repository-sections.js";
  let {
    page,
    onSelect,
    local = false,
  }: {
    page: StorePage;
    onSelect(page: StorePage): void;
    local?: boolean;
  } = $props();
  let header: HTMLElement | null = $state(null);
  let tabList: HTMLElement | null = $state(null);
  let visibleCount = $state(repositorySections.length);
  let mounted = false,
    measuring = false,
    fitAgain = false;
  const sections = $derived(
    repositorySections.filter((item) => !local || item.page !== "reviews"),
  );
  const visible = $derived(sections.slice(0, visibleCount));
  const overflow = $derived(sections.slice(visibleCount));
  const overflowActive = $derived(overflow.some((item) => item.page === page));
  function select(value: string | undefined) {
    const section = repositorySections.find((item) => item.page === value);
    if (section && section.page !== page) onSelect(section.page);
  }
  async function fitTabs() {
    if (!mounted || !header || !tabList) return;
    if (measuring) {
      fitAgain = true;
      return;
    }
    measuring = true;
    do {
      fitAgain = false;
      visibleCount = sections.length;
      await tick();
      while (
        mounted &&
        visibleCount > 1 &&
        tabList.scrollWidth > tabList.clientWidth + 1
      ) {
        visibleCount -= 1;
        await tick();
      }
    } while (mounted && fitAgain);
    measuring = false;
  }
  onMount(() => {
    mounted = true;
    const observer = new ResizeObserver(() => void fitTabs());
    if (header) observer.observe(header);
    void fitTabs();
    return () => {
      mounted = false;
      observer.disconnect();
    };
  });
</script>

<div class="store-application__tabs" bind:this={header}>
  <Tabs.Root
    value={page}
    onValueChange={select}
    class="store-application__tab-root"
  >
    <Tabs.List
      bind:ref={tabList}
      class="store-application__tab-list"
      aria-label="Repository sections"
    >
      {#each visible as item (item.page)}
        <Tabs.Trigger class="store-application__tab-trigger" value={item.page}
          >{item.label}</Tabs.Trigger
        >
      {/each}
    </Tabs.List>
    {#if overflow.length}
      <DropdownMenu.Root>
        <DropdownMenu.Trigger>
          {#snippet child({ props })}
            <Button
              {...props}
              variant="ghost"
              size="sm"
              class="store-application__tab-more"
              aria-label="More repository sections"
              aria-current={overflowActive ? "page" : undefined}
              data-selected={overflowActive ? "" : undefined}
              >More<ChevronDownIcon
                data-icon="inline-end"
                aria-hidden="true"
              /></Button
            >
          {/snippet}
        </DropdownMenu.Trigger>
        <DropdownMenu.Content
          align="start"
          sideOffset={4}
          preventScroll={false}
          aria-label="More repository sections"
        >
          <DropdownMenu.Group>
            {#each overflow as item (item.page)}
              <DropdownMenu.Item
                aria-current={page === item.page ? "page" : undefined}
                onSelect={() => select(item.page)}
                >{item.label}</DropdownMenu.Item
              >
            {/each}
          </DropdownMenu.Group>
        </DropdownMenu.Content>
      </DropdownMenu.Root>
    {/if}
  </Tabs.Root>
</div>
