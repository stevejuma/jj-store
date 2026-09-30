<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import ArrowLeftIcon from "@lucide/svelte/icons/arrow-left";
  import DatabaseIcon from "@lucide/svelte/icons/database";
  import SearchIcon from "@lucide/svelte/icons/search";
  import ServerIcon from "@lucide/svelte/icons/server";
  import type { StoreSettingsSection } from "./model.js";
  import "./StoreSettingsNavigation.css";

  let {
    section,
    compact = false,
    onSelect,
    onClose,
  }: {
    section: StoreSettingsSection;
    compact?: boolean;
    onSelect: (section: StoreSettingsSection) => void;
    onClose: () => void;
  } = $props();

  const sections = [
    { id: "service", label: "Local service", icon: ServerIcon },
    { id: "repository", label: "Repository", icon: DatabaseIcon },
    { id: "search", label: "Search", icon: SearchIcon },
  ] as const;
</script>

<nav
  class="store-settings-navigation"
  aria-label="Settings navigation"
  data-compact={compact || undefined}
>
  <Button
    variant="ghost"
    class="store-settings-navigation__back"
    aria-label="Back to jj-store"
    title={compact ? "Back to jj-store" : undefined}
    onclick={onClose}
  >
    <ArrowLeftIcon data-icon="inline-start" aria-hidden="true" />
    <span>Back to jj-store</span>
  </Button>
  <section>
    <h2>Store</h2>
    {#each sections as item (item.id)}
      <Button
        variant="ghost"
        class="store-settings-navigation__item"
        data-active={section === item.id ? "" : undefined}
        aria-pressed={section === item.id}
        aria-label={item.label}
        title={compact ? item.label : undefined}
        onclick={() => onSelect(item.id)}
      >
        <item.icon data-icon="inline-start" aria-hidden="true" />
        <span>{item.label}</span>
      </Button>
    {/each}
  </section>
</nav>
