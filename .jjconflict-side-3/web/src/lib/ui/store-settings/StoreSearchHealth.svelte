<script lang="ts">
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreSettingsNotice from "./StoreSettingsNotice.svelte";
  import SearchXIcon from "@lucide/svelte/icons/search-x";
  import StoreSettingsEmpty from "./StoreSettingsEmpty.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  let { controller }: { controller: StoreController } = $props();
  const settings = $derived(controller.state.settings);
  const capabilities = $derived(settings.capabilities);
  const search = $derived(settings.searchStatus);
  const number = (value: number | bigint | null | undefined) =>
    value === null || value === undefined
      ? "Not available"
      : value.toLocaleString("en-GB");
</script>

<section class="store-settings__section" aria-labelledby="search-health">
  <header class="store-settings__section-heading">
    <div>
      <h2 id="search-health">Text-search health</h2>
      <p>Positions update automatically while Settings is open.</p>
    </div>
  </header>
  {#if !capabilities?.search}
    <StoreSettingsEmpty
      Icon={SearchXIcon}
      title="Search unavailable"
      description="Search is not configured for this repository."
    />
  {:else if settings.searchStatusState === "loading" && !search}
    <StoreSettingsNotice>Loading search status…</StoreSettingsNotice>
  {:else}
    <dl class="store-settings__rows">
      <StoreReadOnlyRow label="Configured" value="Yes" />
      <StoreReadOnlyRow
        label="Health"
        tone={search?.degraded
          ? "error"
          : search?.is_stale
            ? "warning"
            : "ready"}
        value={search?.degraded
          ? "Degraded"
          : search?.is_stale
            ? "Stale"
            : search
              ? "Fresh"
              : null}
      />
      <StoreReadOnlyRow
        label="Repository position"
        value={search?.repository_position}
        display={number(search?.repository_position)}
      />
      <StoreReadOnlyRow
        label="Durable position"
        value={search?.durable_position}
        display={number(search?.durable_position)}
      />
      <StoreReadOnlyRow
        label="Indexed position"
        value={search?.indexed_position}
        display={number(search?.indexed_position)}
      />
      <StoreReadOnlyRow
        copyable
        label="Index generation"
        copyLabel="Index generation"
        value={search?.index_generation}
        emptyLabel="Not available"
      />
    </dl>
  {/if}
  <StoreNotice
    noticeKey="search-health"
    title="Search status unavailable"
    message={settings.searchStatusError}
    pending={settings.searchStatusState === "loading" ||
      settings.searchStatusState === "refreshing"}
  />
</section>
