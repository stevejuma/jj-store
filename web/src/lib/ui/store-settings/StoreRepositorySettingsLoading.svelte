<script lang="ts">
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import type { RepositorySettingsSection } from "../../controller/repository-settings-navigation.js";
  import StoreSettingsSkeleton from "./StoreSettingsSkeleton.svelte";
  let {
    section = "general",
    tab,
    local = false,
    primary = true,
  }: {
    section?: RepositorySettingsSection;
    tab?: string;
    local?: boolean;
    primary?: boolean;
  } = $props();
  const subtabs = $derived(
    section === "search"
      ? 4
      : section === "storage" || section === "sync"
        ? 2
        : 0,
  );
</script>

<div class="store-settings-loading">
  {#if primary}<div class="store-settings-loading__primary" aria-hidden="true">
      <Skeleton class="store-settings-loading__back" />
      {#each Array(local ? 3 : 6) as _}<Skeleton
          class="store-settings-loading__pill"
        />{/each}
    </div>{/if}
  {#if subtabs}<div
      class="store-settings-loading__secondary"
      aria-hidden="true"
    >
      {#each Array(subtabs) as _}<span
          ><Skeleton class="store-settings-loading__tab-icon" /><Skeleton
            class="store-settings-loading__tab"
          /></span
        >{/each}
    </div>{/if}
  <ScrollArea
    class="store-settings__scroll"
    aria-label="Repository settings loading content"
  >
    <div class="store-settings--repository store-settings-loading__body">
      <StoreSettingsSkeleton {section} {tab} {local} />
    </div>
  </ScrollArea>
</div>

<style>
  .store-settings-loading {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-height: 0;
    min-width: 0;
    overflow: hidden;
  }
  .store-settings-loading__primary,
  .store-settings-loading__secondary {
    display: flex;
    align-items: center;
    flex: none;
    gap: 0.5rem;
    overflow: hidden;
    padding-inline: 1rem;
  }
  .store-settings-loading__primary {
    height: 3.25rem;
  }
  .store-settings-loading__secondary {
    height: 2.75rem;
    border-block-end: 1px solid var(--border);
  }
  .store-settings-loading__secondary span {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    padding-inline: 0.25rem;
  }
  .store-settings-loading__body {
    padding: 1rem 1rem 4rem;
  }
  :global(.store-settings-loading__back) {
    width: 2rem;
    height: 2rem;
    flex: none;
    border-radius: 50%;
  }
  :global(.store-settings-loading__pill) {
    width: 5rem;
    height: 1.8rem;
    flex: none;
    border-radius: 999px;
  }
  :global(.store-settings-loading__tab) {
    width: 5rem;
    height: 0.8rem;
    flex: none;
  }
  :global(.store-settings-loading__tab-icon) {
    width: 1rem;
    height: 1rem;
    flex: none;
  }
</style>
