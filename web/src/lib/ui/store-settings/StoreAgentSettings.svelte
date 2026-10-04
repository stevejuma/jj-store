<script lang="ts">
  import type { CaptureManagement } from "../../controller/capture-types.js";
  import StoreSettingsPage from "./StoreSettingsPage.svelte";
  import StoreCaptureSettings from "./StoreCaptureSettings.svelte";
  let {
    management,
    repositoryName,
  }: {
    management?: CaptureManagement;
    repositoryName?: (id: string) => string;
  } = $props();
  let tab = $state("capture");
</script>

<StoreSettingsPage
  section="agents"
  tabs={[
    { value: "capture", label: "Capture" },
    { value: "configuration", label: "Configuration" },
    { value: "sources", label: "Sources" },
    { value: "imports", label: "Imports" },
    { value: "diagnostics", label: "Diagnostics" },
  ]}
  bind:value={tab}
  onRefresh={management?.supported ? () => management!.refresh() : undefined}
  refreshLabel="Refresh capture"
  refreshDisabled={management?.state.busy ?? false}
>
  <StoreCaptureSettings {management} {repositoryName} bind:section={tab} />
</StoreSettingsPage>
