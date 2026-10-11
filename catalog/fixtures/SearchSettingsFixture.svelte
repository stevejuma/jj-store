<script lang="ts">
  import { untrack } from "svelte";
  import StoreSettingsPage from "../../web/src/lib/ui/store-settings/StoreSettingsPage.svelte";
  import StoreSearchConfiguration from "../../web/src/lib/ui/store-settings/StoreSearchConfiguration.svelte";
  import {
    emptySearchSettings,
    searchDraft,
    searchConfiguration,
    type SearchSettings,
  } from "../../web/src/lib/controller/search-settings.js";
  import { searchSettingsValue } from "./search-settings.js";
  import "../../web/src/lib/ui/store-settings/StoreSettings.css";
  let {
    embedded = false,
    loading = false,
    supported = true,
    fail = false,
    section = "scopes",
  }: {
    embedded?: boolean;
    loading?: boolean;
    supported?: boolean;
    fail?: boolean;
    section?: "configuration" | "scopes" | "advanced";
  } = $props();
  let value = searchSettingsValue();
  const loaded = {
    value,
    draft: searchDraft(value.desired),
    dirty: false,
    state: "ready",
    error: "",
    changing: false,
  } satisfies SearchSettings;
  let settings = $state<SearchSettings>(
    untrack(() =>
      loading ? { ...emptySearchSettings(), state: "loading" } : loaded,
    ),
  );
</script>

{#if loading}
  <button onclick={() => (settings = structuredClone(loaded))}
    >Finish loading</button
  >
  <button onclick={() => (settings.state = "refreshing")}
    >Start background refresh</button
  >
{/if}

<StoreSettingsPage
  {embedded}
  section="search"
  onRefresh={async () => {
    settings.state = "ready";
    settings.error = "";
  }}
>
  <StoreSearchConfiguration
    {settings}
    {supported}
    {section}
    onDraft={(patch) => {
      settings.draft = { ...settings.draft!, ...patch };
      settings.dirty = true;
    }}
    onCancel={() => {
      settings.draft = searchDraft(settings.value!.desired);
      settings.dirty = false;
    }}
    onSave={async () => {
      if (fail) {
        settings.state = "failed";
        settings.error = "Settings changed. Refresh settings before retrying.";
        return;
      }
      settings.value!.desired = searchConfiguration(settings.draft!);
      settings.dirty = false;
    }}
    onControl={async () => {
      settings.value!.effective = structuredClone(
        $state.snapshot(settings.value!.desired),
      );
    }}
  />
</StoreSettingsPage>
