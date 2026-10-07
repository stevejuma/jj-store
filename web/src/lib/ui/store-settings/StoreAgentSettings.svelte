<script lang="ts">
  import type { CaptureManagement } from "../../controller/capture-types.js";
  import StoreSettingsPage from "./StoreSettingsPage.svelte";
  import StoreCaptureSettings from "./StoreCaptureSettings.svelte";
  import StoreAgentDefaults from "./StoreAgentDefaults.svelte";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import StoreAgentRuntimes from "./StoreAgentRuntimes.svelte";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import type { AgentRuntimes } from "../../controller/agent-runtimes.svelte.js";
  let {
    manager,
    management,
    runtimes,
    repositoryName,
    onRepository,
    repositoryNavigationReason,
  }: {
    manager?: AgentExecution;
    management?: CaptureManagement;
    runtimes?: AgentRuntimes;
    repositoryName?: (id: string) => string;
    onRepository?: (id: string) => void;
    repositoryNavigationReason?: string;
  } = $props();
  let tab = $state("runtimes");
</script>

<StoreSettingsPage
  section="agents"
  tabs={[
    { value: "runtimes", label: "Runtimes" },
    { value: "capture", label: "Capture" },
    { value: "configuration", label: "Configuration" },
    { value: "sources", label: "Sources" },
    { value: "imports", label: "Imports" },
    { value: "diagnostics", label: "Diagnostics" },
  ]}
  bind:value={tab}
  onRefresh={tab === "runtimes"
    ? undefined
    : management?.supported
      ? () => management!.refresh()
      : undefined}
  refreshLabel="Refresh capture"
  refreshDisabled={management?.state.busy ?? false}
>
  <Tabs.Content value="runtimes"
    ><StoreAgentDefaults {manager} />
    <h2>Agent runtimes</h2>
    <StoreAgentRuntimes
      management={runtimes}
      active={tab === "runtimes"}
    /></Tabs.Content
  >
  {#if tab !== "runtimes"}
    <StoreCaptureSettings
      {management}
      {repositoryName}
      {onRepository}
      {repositoryNavigationReason}
      bind:section={tab}
    />
  {/if}
</StoreSettingsPage>
