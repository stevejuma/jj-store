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
</script>

<StoreSettingsPage
  section="agents"
  onRefresh={management?.supported ? () => management!.refresh() : undefined}
  refreshLabel="Refresh capture"
  refreshDisabled={management?.state.busy ?? false}
>
  <StoreCaptureSettings {management} {repositoryName} />
</StoreSettingsPage>
