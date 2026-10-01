<script lang="ts">
  import StoreSearchConfiguration from "../../web/src/lib/ui/store-settings/StoreSearchConfiguration.svelte";
  import {
    searchDraft,
    searchConfiguration,
    type SearchSettings,
  } from "../../web/src/lib/controller/search-settings.js";
  import { searchSettingsValue } from "./search-settings.js";
  import "../../web/src/lib/ui/store-settings/StoreSettings.css";
  let {
    supported = true,
    fail = false,
  }: { supported?: boolean; fail?: boolean } = $props();
  let value = searchSettingsValue();
  let settings = $state<SearchSettings>({
    value,
    draft: searchDraft(value.desired),
    dirty: false,
    state: "ready",
    error: "",
    changing: false,
  });
</script>

<div class="store-settings">
  <StoreSearchConfiguration
    {settings}
    {supported}
    section="scopes"
    onRefresh={async () => {
      settings.state = "ready";
      settings.error = "";
    }}
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
</div>
