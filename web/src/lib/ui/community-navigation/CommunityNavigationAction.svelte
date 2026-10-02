<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
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
</script>

<Button
  variant="ghost"
  size="icon-sm"
  class="lc-community-navigation__section-action"
  aria-label={item.label}
  title={item.label}
  onclick={() => onSelect(item)}><Icon aria-hidden="true" /></Button
>
