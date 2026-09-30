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
    onSelect,
    onClose,
  }: {
    section: StoreSettingsSection;
    onSelect: (section: StoreSettingsSection) => void;
    onClose: () => void;
  } = $props();

  const sections = [
    { id: "service", label: "Local service", icon: ServerIcon },
    { id: "repository", label: "Repository", icon: DatabaseIcon },
    { id: "search", label: "Search", icon: SearchIcon },
  ] as const;
</script>

<nav class="store-settings-navigation" aria-label="Settings navigation">
  <Button
    variant="ghost"
    class="store-settings-navigation__back"
    onclick={onClose}
  >
    <ArrowLeftIcon data-icon="inline-start" aria-hidden="true" />
    Back to jj-store
  </Button>
  <section>
    <h2>Store</h2>
    {#each sections as item (item.id)}
      <Button
        variant="ghost"
        class="store-settings-navigation__item"
        data-active={section === item.id ? "" : undefined}
        aria-pressed={section === item.id}
        onclick={() => onSelect(item.id)}
      >
        <item.icon data-icon="inline-start" aria-hidden="true" />
        {item.label}
      </Button>
    {/each}
  </section>
</nav>
