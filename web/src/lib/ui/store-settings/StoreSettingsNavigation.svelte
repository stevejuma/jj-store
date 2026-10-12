<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import UserIcon from "@lucide/svelte/icons/user";
  import SmileIcon from "@lucide/svelte/icons/smile";
  import BotIcon from "@lucide/svelte/icons/bot";
  import PaletteIcon from "@lucide/svelte/icons/palette";
  import ArrowLeftIcon from "@lucide/svelte/icons/arrow-left";
  import ServerIcon from "@lucide/svelte/icons/server";
  import CpuIcon from "@lucide/svelte/icons/cpu";
  import { version } from "../../../../../package.json";
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

  const groups = [
    {
      label: "Personal",
      items: [
        { id: "profile", label: "Profile", icon: UserIcon },
        { id: "appearance", label: "Appearance", icon: PaletteIcon },
        { id: "custom-emoji", label: "Custom emoji", icon: SmileIcon },
      ],
    },
    {
      label: "App",
      items: [
        { id: "service", label: "Local service", icon: ServerIcon },
        { id: "agents", label: "Agents", icon: BotIcon },
        { id: "compute", label: "Compute", icon: CpuIcon },
      ],
    },
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
  <div class="store-settings-navigation__groups">
    {#each groups as group (group.label)}
      <section aria-label={group.label}>
        <h2>{group.label}</h2>
        {#each group.items as item (item.id)}
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
    {/each}
  </div>
  <footer
    class="store-settings-navigation__version"
    aria-label={`jj-store version ${version}`}
    title={`jj-store v${version}`}
  >
    v{version}
  </footer>
</nav>
