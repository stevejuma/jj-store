<script lang="ts">
  // Width fitting follows CommunityProjectsRepositoryDocuments; pill styling is retained.
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as DropdownMenu from "@lapismd/design-core/shadcn/dropdown-menu";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
  import { onMount, tick, untrack } from "svelte";
  import type { Snippet } from "svelte";
  let {
    value,
    sections,
    onSelect,
    label = "Repository sections",
    navigationLabel,
    disabled = false,
    leading,
    trailing,
  }: {
    value: string;
    sections: readonly { value: string; label: string }[];
    onSelect(value: string): void;
    label?: string;
    navigationLabel?: string;
    disabled?: boolean;
    leading?: Snippet;
    trailing?: Snippet;
  } = $props();
  let header: HTMLElement | null = $state(null);
  let tabList: HTMLElement | null = $state(null);
  let visibleCount = $state(untrack(() => sections.length));
  let mounted = false,
    measuring = false,
    fitAgain = false;
  const visible = $derived(sections.slice(0, visibleCount));
  const overflow = $derived(sections.slice(visibleCount));
  const overflowActive = $derived(
    overflow.some((item) => item.value === value),
  );
  function select(next: string | undefined) {
    if (
      !disabled &&
      next !== value &&
      sections.some((item) => item.value === next)
    )
      onSelect(next!);
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
  $effect(() => {
    sections;
    untrack(() => {
      void fitTabs();
    });
  });
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

<div
  class="store-application__tabs"
  bind:this={header}
  role={navigationLabel ? "navigation" : undefined}
  aria-label={navigationLabel}
>
  <Tabs.Root {value} onValueChange={select} class="store-application__tab-root">
    {@render leading?.()}
    <Tabs.List
      bind:ref={tabList}
      class="store-application__tab-list"
      aria-label={label}
    >
      {#each visible as item (item.value)}
        <Tabs.Trigger
          class="store-application__tab-trigger"
          value={item.value}
          {disabled}>{item.label}</Tabs.Trigger
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
              {disabled}
              aria-label={`More ${label.toLowerCase()}`}
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
          aria-label={`More ${label.toLowerCase()}`}
        >
          <DropdownMenu.Group>
            {#each overflow as item (item.value)}
              <DropdownMenu.Item
                aria-current={value === item.value ? "page" : undefined}
                onSelect={() => select(item.value)}
                >{item.label}</DropdownMenu.Item
              >
            {/each}
          </DropdownMenu.Group>
        </DropdownMenu.Content>
      </DropdownMenu.Root>
    {/if}
    {@render trailing?.()}
  </Tabs.Root>
</div>
