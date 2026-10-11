<script lang="ts">
  import StoreRepositorySettingsLoading from "../../web/src/lib/ui/store-settings/StoreRepositorySettingsLoading.svelte";
  import type {
    RepositorySettingsSection,
    RepositorySettingsTab,
  } from "../../web/src/lib/controller/repository-settings-navigation.js";
  let {
    section = "general",
    tab,
    local = false,
    narrow = false,
  }: {
    section?: RepositorySettingsSection;
    tab?: RepositorySettingsTab;
    local?: boolean;
    narrow?: boolean;
  } = $props();
</script>

<main
  style:width={narrow ? "320px" : "900px"}
  style:max-width="100%"
  style:height="700px"
  style:display="flex"
  style:flex-direction="column"
>
  <StoreRepositorySettingsLoading {section} {tab} {local} />
</main>
