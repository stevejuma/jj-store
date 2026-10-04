<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as DropdownMenu from "@lapismd/design-core/shadcn/dropdown-menu";
  import { navigationItemIcon } from "./icons.js";
  import type { CommunityNavigationItemModel } from "./model.js";
  let {
    item,
    onSelect,
  }: {
    item: CommunityNavigationItemModel;
    onSelect: (item: CommunityNavigationItemModel) => void;
  } = $props();
  const Icon = $derived(navigationItemIcon(item));
  let button = $state<HTMLButtonElement | null>(null);
  let openingDialog = false;
</script>

{#if item.children?.length}
  <DropdownMenu.Root
    onOpenChange={(open) => {
      if (open) openingDialog = false;
    }}
  >
    <DropdownMenu.Trigger>
      {#snippet child({ props })}
        <Button
          {...props}
          bind:ref={button}
          variant="ghost"
          size="icon-sm"
          class="lc-community-navigation__section-action"
          aria-label={item.label}
          title={item.label}
          disabled={item.disabled}><Icon aria-hidden="true" /></Button
        >
      {/snippet}
    </DropdownMenu.Trigger>
    <DropdownMenu.Content
      align="end"
      preventScroll={false}
      aria-label={item.label}
      style="width: 16rem; max-width: calc(100vw - 2rem)"
      onCloseAutoFocus={(event) => {
        if (openingDialog) event.preventDefault();
      }}
    >
      <DropdownMenu.Group>
        {#each item.children as choice (choice.id)}
          {@const ChoiceIcon = navigationItemIcon(choice)}
          <DropdownMenu.Item
            disabled={choice.disabled}
            onSelect={() => {
              openingDialog = true;
              button?.focus();
              onSelect(choice);
            }}
            ><ChoiceIcon
              data-icon="inline-start"
              aria-hidden="true"
            />{choice.label}</DropdownMenu.Item
          >
        {/each}
      </DropdownMenu.Group>
    </DropdownMenu.Content>
  </DropdownMenu.Root>
{:else}
  <Button
    variant="ghost"
    size="icon-sm"
    class="lc-community-navigation__section-action"
    aria-label={item.label}
    title={item.label}
    disabled={item.disabled}
    onclick={() => onSelect(item)}><Icon aria-hidden="true" /></Button
  >
{/if}
